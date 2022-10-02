#![warn(clippy::all, clippy::pedantic)]

use crate::prelude::*;

#[system]
#[write_component(Point)]
pub fn random_move(ecs: &mut SubWorld, #[resource] map: &Map) {
    <&mut Point>::query()
        .filter(component::<MovingRandomly>())
        .iter_mut(ecs)
        .for_each(|pos| {
            let mut rng = RandomNumberGenerator::new();
            let destination = match rng.range(0, 4) {
                0 => Point::new(-1, 0),
                1 => Point::new(1, 0),
                2 => Point::new(0, -1),
                _ => Point::new(0, 1),
            };
            let new_pos = *pos + destination;
            if map.can_enter_tile(new_pos) {
                *pos = new_pos;
            }
        });
}
