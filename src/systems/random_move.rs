#![warn(clippy::all, clippy::pedantic)]

use crate::prelude::*;

#[system]
#[write_component(Point)]
#[read_component(Health)]
#[read_component(Player)]
#[read_component(MovingRandomly)]
pub fn random_move(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    <(Entity, &Point, &MovingRandomly)>::query()
        .iter(ecs)
        .for_each(|(entity, pos, _)| {
            let mut rng = RandomNumberGenerator::new();
            let destination = match rng.range(0, 4) {
                0 => Point::new(-1, 0),
                1 => Point::new(1, 0),
                2 => Point::new(0, -1),
                _ => Point::new(0, 1),
            } + *pos;

            let mut hit_something = false;

            <(Entity, &Point, &Health)>::query()
                .iter(ecs)
                .filter(|(_, target_pos, _)| **target_pos == destination)
                .for_each(|(victim, _, _)| {
                    hit_something = true;
                    println!("Hit somerhing!");
                    if (ecs.entry_ref(*victim))
                        .unwrap()
                        .get_component::<Player>()
                        .is_ok()
                    {
                        commands.push((WantsToAttack {
                            attacker: *entity,
                            victim: *victim,
                        },));
                    }
                });
            // move if not attack
            if !hit_something {
                commands.push((WantsToMove {
                    entity: *entity,
                    destination,
                },));
            }
        });
}
