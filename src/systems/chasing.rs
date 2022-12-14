#![warn(clippy::all, clippy::pedantic)]

use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(ChasingPlayer)]
#[read_component(Health)]
#[read_component(Player)]
#[read_component(FieldOfView)]
pub fn chasing(
    #[resource] map: &Map,
    ecs: &SubWorld,
    commands: &mut CommandBuffer,
) {
    let mut movers = <(Entity, &Point, &ChasingPlayer, &FieldOfView)>::query()
        .filter(component::<ChasingPlayer>());
    let mut positions = <(Entity, &Point, &Health)>::query();
    let mut player = <&Point>::query().filter(component::<Player>());

    let player_pos = player.iter(ecs).next().unwrap();
    let player_idx = get_idx(player_pos.x, player_pos.y);

    let search_targets = vec![player_idx];
    let dijkstra_map = DijkstraMap::new(
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        &search_targets,
        map,
        1024.0,
    );

    movers
        .iter(ecs)
        .filter(|(_, _, _, fov)| fov.visable_tiles.contains(player_pos))
        .for_each(|(entity, pos, _, _)| {
            let idx = get_idx(pos.x, pos.y);
            if let Some(destination) =
                DijkstraMap::find_lowest_exit(&dijkstra_map, idx, map)
            {
                let distance =
                    DistanceAlg::Pythagoras.distance2d(*pos, *player_pos);
                // if not near player, go to him
                let destination = if distance > 1.2 {
                    map.index_to_point2d(destination)
                } else {
                    *player_pos
                };

                let mut hit_something = false;

                positions
                    .iter(ecs)
                    .filter(|(_, target_pos, _)| **target_pos == destination)
                    .for_each(|(victim, _, _)| {
                        if ecs
                            .entry_ref(*victim)
                            .unwrap()
                            .get_component::<Player>()
                            .is_ok()
                        {
                            commands.push((WantsToAttack {
                                attacker: *entity,
                                victim: *victim,
                            },));
                        }
                        hit_something = true;
                    });

                if !hit_something {
                    commands.push((WantsToMove {
                        entity: *entity,
                        destination,
                    },));
                }
            }
        });
}
