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
}

impl RaycastThreadPool {
    pub fn new(num_threads: usize) -> Self {
        let (task_sender, task_receiver) = bounded::<RaycastTask>(100);
        
        let mut worker_handles = Vec::new();
        
        for thread_id in 0..num_threads {
            let receiver = task_receiver.clone();
            
            let handle = thread::spawn(move || {
                println!("Raycasting Thread {} started", thread_id);
                
                // Each task now contains its own chunk reference
                while let Ok(task) = receiver.recv() {                    
                    // Lock the chunk that was passed in the task
                    let world = task.world.read().unwrap(); //Lock as read


                    let mut chunk = task.chunk.write().unwrap(); 
                    let camera_data = Arc::clone(&task.camera_data);
                    
                    // Perform raycasting on the chunk
                    chunk.set_ray_casted(true);                
                    chunk.raycast_chunk(&camera_data, &world);
    
                }
                

            });
            
            worker_handles.push(handle);
        }
        
        drop(task_receiver);
        
        Self {
            task_sender,
            _worker_handles: worker_handles,
        }
    }

    /// Submit a task to the thread pool
    pub fn submit_task(
        &self,
        chunk: Arc<RwLock<CastedChunk>>,
        camera_data: Arc<CameraData>,
        world: Arc<RwLock<World>>,
    ) -> Result<(), String> {
        
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
}