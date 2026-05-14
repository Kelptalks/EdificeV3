pub mod event_manager;

pub mod game_event_manager;
pub mod dispatch_event_manager;
pub mod input_event_manager;

pub mod prelude;

pub use game_event_manager::render_event_manager;
pub use game_event_manager::widget_event_manager;
pub use game_event_manager::player_data_event_manager;

pub mod debug_data;