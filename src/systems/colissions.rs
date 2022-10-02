#![warn(clippy::all, clippy::pedantic)]

use crate::prelude::*;


// get all entieties with Point and Render components
#[system]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Enemy)]
pub fn colissions(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut player_pos = Point::zero();
    <&Point>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .for_each(|pos| {
            player_pos = *pos;
        });

    <(Entity, &Point)>::query()
        .filter(component::<Enemy>())
        .iter(ecs)
        .filter(|(_, pos)| **pos == player_pos)
        .for_each(|(entity, _)| {
            commands.remove(*entity);
        });
}
