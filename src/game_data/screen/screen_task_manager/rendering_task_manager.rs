
pub struct RenderingTaskManager {
    block_update_tasks: Vec<[i32; 3]>,
}

impl RenderingTaskManager {
    pub fn new() -> RenderingTaskManager {
        RenderingTaskManager {
            block_update_tasks: Vec::new(),
        }
    }

    pub fn add_block_render_task(&mut self, cords: [i32; 3]) {
        self.block_update_tasks.push(cords);
    }

    pub fn execute_render_updates_drone(&mut self) {
        self.block_update_tasks.clear();
    }
}
