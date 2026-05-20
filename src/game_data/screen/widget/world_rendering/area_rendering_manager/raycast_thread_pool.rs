
use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{self, Sender};
use std::thread;

use crate::game_data::{
    World,
    locations::world_area::WorldArea,
    screen::{
        iso_cord_tool,
        widget::world_rendering::area_rendering_manager::{
            area_rendering_manager::AreaRenderingManager,
            block_lair_manager::lair_block::LairBlockMod,
            ray_caster::casted_tile::CastedTile,
        },
    },
};

static NEXT_TASK_ID: AtomicU32 = AtomicU32::new(0);

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct RayCastingTaskId {
    id: u32,
}

impl RayCastingTaskId {
    fn new() -> Self {
        RayCastingTaskId {
            id: NEXT_TASK_ID.fetch_add(1, Ordering::Relaxed),
        }
    }
}

type TaskMap = HashMap<[i32; 2], CastedTile>;

struct RayCastJob {
    world_area: WorldArea,
    lair_block_mods: Vec<LairBlockMod>,
    world_snapshot: World,
    result: Arc<Mutex<Option<TaskMap>>>,
}

pub struct RayCastingThreadPool {
    sender: Sender<RayCastJob>,
    pending: HashMap<RayCastingTaskId, Arc<Mutex<Option<TaskMap>>>>,
}

impl RayCastingThreadPool {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel::<RayCastJob>();
        let receiver = Arc::new(Mutex::new(receiver));

        let num_threads = thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(4);

        for _ in 0..num_threads {
            let rx = Arc::clone(&receiver);
            thread::spawn(move || loop {
                let job = rx.lock().unwrap().recv();
                match job {
                    Ok(job) => {
                        let mut mgr = AreaRenderingManager::new();
                        mgr.set_world_area(job.world_area);
                        let tiles = mgr.get_casted_tile_rays(&job.world_snapshot, &job.lair_block_mods);

                        let mut map = HashMap::new();
                        for tile in tiles {
                            let iso_cords = iso_cord_tool::flatten_world_cords(tile.get_world_cords());
                            map.insert(iso_cords, tile);
                        }

                        *job.result.lock().unwrap() = Some(map);
                    }
                    Err(_) => break,
                }
            });
        }

        RayCastingThreadPool {
            sender,
            pending: HashMap::new(),
        }
    }

    pub fn submit(
        &mut self,
        world_area: WorldArea,
        lair_block_mods: Vec<LairBlockMod>,
        world_snapshot: World,
    ) -> RayCastingTaskId {
        let id = RayCastingTaskId::new();
        let result: Arc<Mutex<Option<TaskMap>>> = Arc::new(Mutex::new(None));
        self.pending.insert(id, Arc::clone(&result));
        let _ = self.sender.send(RayCastJob {
            world_area,
            lair_block_mods,
            world_snapshot,
            result,
        });
        id
    }

    // Returns the completed map and removes the task. None if still running.
    pub fn get_task_map(&mut self, id: RayCastingTaskId) -> Option<TaskMap> {
        if let Some(result_arc) = self.pending.get(&id) {
            let mut guard = result_arc.lock().unwrap();
            if guard.is_some() {
                let map = guard.take().unwrap();
                drop(guard);
                self.pending.remove(&id);
                return Some(map);
            }
        }
        None
    }

    // Drops the pending entry so a stale result is never applied.
    // The worker thread still finishes, but the result Arc is released here.
    pub fn cancel_task(&mut self, id: RayCastingTaskId) {
        self.pending.remove(&id);
    }
}
