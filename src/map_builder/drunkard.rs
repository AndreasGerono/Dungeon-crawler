#![warn(clippy::all, clippy::pedantic)]

use super::MapArchitect;
use crate::prelude::*;

const STAGGER_DISTANCE: usize = 400;
const DESIRED_FLOOR: usize = NUM_TILES / 3;

pub struct Architect {}

impl Architect {
    fn drunkard(start: Point, rng: &mut RandomNumberGenerator, map: &mut Map) {
        let mut drunken_pos = start;
        let mut distance_staggered = 0;

        loop {
            let drunk_idx = map.point2d_to_index(drunken_pos);
            map.tiles[drunk_idx] = TileType::Floor;

            match rng.range(0, 4) {
                0 => drunken_pos.x -= 1,
                1 => drunken_pos.x += 1,
                2 => drunken_pos.y -= 1,
                _ => drunken_pos.y += 1,
            }

            if !map.in_bounds(drunken_pos) {
                break;
            }

            distance_staggered += 1;

            if distance_staggered > STAGGER_DISTANCE {
                break;
            }
        }
    }
}

impl MapArchitect for Architect {
    fn build(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mut mb = MapBuilder {
            map: Map::new(),
            rooms: Vec::new(),
            monster_spawns: Vec::new(),
            player_start: Point::zero(),
            amulet_start: Point::zero(),
        };

        mb.fill(TileType::Wall);
        let center = Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2);
        Architect::drunkard(center, rng, &mut mb.map);

        while mb
            .map
            .tiles
            .iter()
            .filter(|t| **t == TileType::Floor)
            .count()
            < DESIRED_FLOOR
        {
            Architect::drunkard(
                Point::new(
                    rng.range(0, SCREEN_WIDTH),
                    rng.range(0, SCREEN_HEIGHT),
                ),
                rng,
                &mut mb.map,
            );

            let dijkstramap = DijkstraMap::new(
                SCREEN_HEIGHT,
                SCREEN_WIDTH,
                &[mb.map.point2d_to_index(center)],
                &mb.map,
                1024.0,
            );

            dijkstramap
                .map
                .iter()
                .enumerate()
                .filter(|(_, distance)| *distance > &2000.0)
                .for_each(|(idx, _)| mb.map.tiles[idx] = TileType::Wall);
        }

        mb.monster_spawns = mb.spawn_monsters(center, rng);
        mb.player_start = center;
        mb.amulet_start = mb.find_most_distant();

        mb
    }
}
