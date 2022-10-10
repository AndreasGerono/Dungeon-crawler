#![warn(clippy::all, clippy::pedantic)]

use crate::prelude::*;

mod combat;
mod end_turn;
mod entities_renderer;
mod hud;
mod map_renders;
mod movements;
mod player_input;
mod random_move;
mod tooltips;
mod chasing;

pub fn build_input_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(player_input::player_input_system())
        .flush()
        .add_system(map_renders::map_render_system())
        .add_system(entities_renderer::render_entities_system())
        .add_system(hud::hud_system())
        .add_system(tooltips::tooltips_system())
        .build()
}

pub fn build_player_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(combat::combat_system())
        .flush()
        .add_system(movements::movement_system())
        .flush()
        .add_system(map_renders::map_render_system())
        .add_system(entities_renderer::render_entities_system())
        .add_system(hud::hud_system())
        .add_system(end_turn::end_turn_system())
        .build()
}

pub fn build_monsters_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(random_move::random_move_system())
        .add_system(chasing::chasing_system())
        .flush()
        .add_system(combat::combat_system())
        .flush()
        .add_system(movements::movement_system())
        .flush()
        .add_system(map_renders::map_render_system())
        .add_system(entities_renderer::render_entities_system())
        .add_system(hud::hud_system())
        .add_system(end_turn::end_turn_system())
        .build()
}
