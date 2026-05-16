use std::collections::HashMap;
use std::time::{Duration, Instant};
use std::cell::RefCell;

pub struct FrameProfiler {
    sections: HashMap<&'static str, Duration>,
    known_sections: Vec<&'static str>,
    display: Vec<String>,
    frame_count: u32,
    print_interval: u32,
}

impl FrameProfiler {
    fn new() -> Self {
        Self {
            sections: HashMap::new(),
            known_sections: Vec::new(),
            display: Vec::new(),
            frame_count: 0,
            print_interval: 120,
        }
    }

    pub fn register(&mut self, name: &'static str) {
        if !self.known_sections.contains(&name) {
            self.known_sections.push(name);
        }
    }

    pub fn record(&mut self, name: &'static str, duration: Duration) {
        *self.sections.entry(name).or_default() += duration;
    }

    pub fn end_frame(&mut self) {
        self.frame_count += 1;
        if self.frame_count >= self.print_interval {
            let n = self.print_interval as u128;
            self.display.clear();

            // Registered sections always appear, in registration order, with 0 as default.
            for &name in &self.known_sections {
                let avg = self.sections
                    .get(name)
                    .map(|d| d.as_micros() / n)
                    .unwrap_or(0);
                self.display.push(format!("{}: {} µs/frame", name, avg));
            }

            self.sections.clear();
            self.frame_count = 0;
        }
    }

    pub fn get_display(&self) -> &Vec<String> {
        &self.display
    }
}

thread_local! {
    static PROFILER: RefCell<FrameProfiler> = RefCell::new(FrameProfiler::new());
}

/// Register all timing sections up front so they always appear in the debug menu,
/// even when 0. Call once at startup.
pub fn prof_init() {
    PROFILER.with(|p| {
        let mut p = p.borrow_mut();
        // Top-level frame sections
        p.register("render_screen");
        p.register("  tile_map_clean");
        p.register("  tile_map_flatten");
        p.register("  world_render_world");
        p.register("    chunk_tile_render");
        p.register("    chunk_entity_render");
        p.register("    cache_bake");
        p.register("    render_from_cache");
        p.register("    render_tiles_direct");
        p.register("tik_manager");
        p.register("  tik_game_entities");
        p.register("  tik_drones");
        p.register("  tik_world");
        p.register("  tik_world_events");
        p.register("events_dispatch");
        p.register("events_world");
        p.register("events_render");
        p.register("gpu_flush");
        p.register("  gpu_flush_cache_baking");
        p.register("  gpu_flush_cache_drawing");
        p.register("  gpu_flush_main");
    });
}

pub fn prof_record(name: &'static str, duration: Duration) {
    PROFILER.with(|p| p.borrow_mut().record(name, duration));
}

pub fn prof_end_frame() {
    PROFILER.with(|p| p.borrow_mut().end_frame());
}

pub fn prof_get_display() -> Vec<String> {
    PROFILER.with(|p| p.borrow().get_display().clone())
}

pub struct ProfTimer {
    name: &'static str,
    start: Instant,
}

impl ProfTimer {
    pub fn new(name: &'static str) -> Self {
        Self { name, start: Instant::now() }
    }
}

impl Drop for ProfTimer {
    fn drop(&mut self) {
        prof_record(self.name, self.start.elapsed());
    }
}
