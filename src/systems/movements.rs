#![warn(clippy::all, clippy::pedantic)]

use crate::prelude::*;

/*
  Systems that run only one for single query.
  This is the same as declaring a query that reads Entity and WantsToMove and
  iterating it as you have with other systems.

  What is done:
    - itarate all entities with WantsToMove component
    - check if move is valid and replace point component of a target
    - if entity is a player also update the camera
*/

#[system(for_each)]
#[read_component(Player)]
#[read_component(FieldOfView)]
pub fn movement(
    entity: &Entity,
    want_move: &WantsToMove,
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] map: &mut Map,
    #[resource] camera: &mut Camera,
) {
    if map.can_enter_tile(want_move.destination) {
        // use commands to update position of a target
        commands.add_component(want_move.entity, want_move.destination);

        // update foi and map if exists
        if let Ok(entry) = ecs.entry_ref(want_move.entity) {
            if let Ok(fov) = entry.get_component::<FieldOfView>() {
                commands.add_component(want_move.entity, fov.clone_dirty());

                if entry.get_component::<Player>().is_ok() {
                    camera.on_player_move(want_move.destination);
                    fov.visable_tiles.iter().for_each(|pos| {
                        map.revealed_tiles[get_idx(pos.x, pos.y)] = true;
                    });
                }
            }
        }
    }
    commands.remove(*entity); // move done, delete WantsToMove
}
