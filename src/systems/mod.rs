#![warn(clippy::all, clippy::pedantic)]

use crate::prelude::*;

mod player_input;
mod map_renders;
mod entities_renderer;
mod colissions;

pub fn build_scheduler() -> Schedule {
    // Include systems in the scheduler
    Schedule::builder()
    .add_system(player_input::player_input_system())
    .add_system(map_renders::map_render_system())
    .add_system(entities_renderer::render_entities_system())
    .add_system(colissions::colissions_system())
    .build()
}
