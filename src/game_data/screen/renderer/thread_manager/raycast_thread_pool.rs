use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use crossbeam_channel::{Receiver, Sender, bounded, unbounded};

use crate::game_data::screen::camera_data::CameraData;
use crate::game_data::screen::renderer::casted_block_manager::casted_chunk::CastedChunk;
use crate::game_data::screen::renderer::ray_caster;
use crate::game_data::{World, world};



struct RaycastTask {
    chunk: Arc<Mutex<CastedChunk>>,
    camera_data: Arc<CameraData>,
    world : Arc<RwLock<World>>,

    // any other task-specific data
    frame_number: u64,
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
                println!("Worker thread {} started", thread_id);
                
                // Each task now contains its own chunk reference
                while let Ok(task) = receiver.recv() {
                    println!("Thread {} received task for frame {}", 
                             thread_id, task.frame_number);
                    

                    
                    // Lock the chunk that was passed in the task
                    let world = task.world.read().unwrap(); //Lock as read
                    let mut chunk = task.chunk.lock().unwrap(); 
                    let camera_data = Arc::clone(&task.camera_data);
                    
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