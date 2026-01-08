use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, RwLock};
use std::thread;
use crossbeam_channel::{Sender, bounded};

use crate::game_data::screen::camera_data::CameraData;
use crate::game_data::screen::renderer::casted_block_manager::casted_chunk::CastedChunk;
use crate::game_data::{World};



struct RaycastTask {
    chunk: Arc<RwLock<CastedChunk>>,
    camera_data: Arc<CameraData>,
    world : Arc<RwLock<World>>,
}

// Main thread pool structure
pub struct RaycastThreadPool {
    task_sender: Sender<RaycastTask>,
    _worker_handles: Vec<thread::JoinHandle<()>>,
    pending_tasks: Arc<AtomicUsize>,  // Track number of active tasks
}

impl RaycastThreadPool {
    pub fn new(num_threads: usize) -> Self {
        let (task_sender, task_receiver) = bounded::<RaycastTask>(1000);
        let pending_tasks = Arc::new(AtomicUsize::new(0));

        let mut worker_handles = Vec::new();
        
        for thread_id in 0..num_threads {
            let receiver = task_receiver.clone();
            let pending = Arc::clone(&pending_tasks);
            
            let handle = thread::spawn(move || {
                
                // Each task now contains its own chunk reference
                while let Ok(task) = receiver.recv() {                    
                    // Lock the chunk that was passed in the task
                    let world = task.world.read().unwrap(); //Lock as read


                    let mut chunk = task.chunk.write().unwrap(); 
                    let camera_data = Arc::clone(&task.camera_data);
                    
                    // Perform raycasting on the chunk          
                    chunk.raycast_chunk(&camera_data, &world);
                    chunk.set_ray_casted(true);    

                    // Decrement counter when task completes
                    pending.fetch_sub(1, Ordering::SeqCst);  
                }
                

            });
            
            worker_handles.push(handle);
        }
        
        drop(task_receiver);
        
        Self {
            task_sender,
            _worker_handles: worker_handles,
            pending_tasks: pending_tasks,
        }
    }

    /// Submit a task to the thread pool
    pub fn submit_task(
        &self,
        chunk: Arc<RwLock<CastedChunk>>,
        camera_data: Arc<CameraData>,
        world: Arc<RwLock<World>>,
    ) -> Result<(), String> {
        // Increment counter before submitting
        self.pending_tasks.fetch_add(1, Ordering::SeqCst);

        let task = RaycastTask {
            chunk,
            camera_data,
            world,
        };
        
        self.task_sender.send(task)
            .map_err(|_| {
                // If send fails, decrement the counter back
                "Failed to send task to worker threads".to_string()
            })
    }

    pub fn get_total_tasks(&self) -> usize {
        return self.pending_tasks.load(Ordering::SeqCst);
    }

    /// Wait for all worker threads to finish processing current tasks
    pub fn wait_for_completion(&mut self) {
        // Spin until all tasks are done
        while self.pending_tasks.load(Ordering::SeqCst) > 0 {
            std::thread::sleep(std::time::Duration::from_micros(100));
        }
    }

}