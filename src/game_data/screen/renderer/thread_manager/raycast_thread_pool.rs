use std::sync::{Arc, Mutex};
use std::thread;
use crossbeam_channel::{Receiver, Sender, bounded, unbounded};

use crate::game_data::screen::CameraData;
use crate::game_data::screen::renderer::casted_block_manager::casted_chunk::CastedChunk;
use crate::game_data::screen::renderer::ray_caster;
use crate::game_data::{World, world};



struct RaycastTask {
    chunk: Arc<Mutex<CastedChunk>>,
    // any other task-specific data
    frame_number: u64,
}

// Main thread pool structure
pub struct RaycastThreadPool {
    task_sender: Sender<RaycastTask>,
    _worker_handles: Vec<thread::JoinHandle<()>>,
}

impl RaycastThreadPool {
    pub fn new(num_threads: usize, world: Arc<World>, camera_data: Arc<CameraData>, ) -> Self {
        let (task_sender, task_receiver) = bounded::<RaycastTask>(100);
        
        let mut worker_handles = Vec::new();
        
        for thread_id in 0..num_threads {
            let receiver = task_receiver.clone();
            let world = Arc::clone(&world);
            let camera_data = Arc::clone(&camera_data);
            
            let handle = thread::spawn(move || {
                println!("Worker thread {} started", thread_id);
                
                // Each task now contains its own chunk reference
                while let Ok(task) = receiver.recv() {
                    println!("Thread {} received task for frame {}", 
                             thread_id, task.frame_number);
                    

                    
                    // Lock the chunk that was passed in the task
                    let mut chunk = task.chunk.lock().unwrap();
                    
                    // Perform raycast
                    
                    chunk.raycast_chunk(&camera_data, &world);
                    
                    println!("Thread {} finished raycasting", thread_id);
                }
                
                println!("Worker thread {} shutting down", thread_id);
            });
            
            worker_handles.push(handle);
        }
        
        drop(task_receiver);
        
        Self {
            task_sender,
            _worker_handles: worker_handles,
        }
    }
}