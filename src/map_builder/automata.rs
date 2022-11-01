#![warn(clippy::all, clippy::pedantic)]

use super::MapArchitect;
use crate::prelude::*;

pub struct Architect {}

impl Architect {
    fn random_noise_map(rng: &mut RandomNumberGenerator, map: &mut Map) {
        map.tiles.iter_mut().enumerate()
        .for_each(|(_, t)| {
            let roll = rng.range(0, 100);
            if roll > 55 {
                *t = TileType::Floor;
            } else {
                *t = TileType::Wall;
            }
        });
    }

    fn count_neighbours(x: i32, y: i32, map: &Map) -> usize {
        let mut neighbours = 0;
        for iy in -1..=1 {
            for ix in -1..=1 {
                if !(ix == 0 && iy == 0)
                    && (map.tiles[get_idx(x + ix, y + iy)] == TileType::Wall)
                {
                    neighbours += 1;
                }
            }
        }
        neighbours
    }

    fn iteration(map: &mut Map) {
        // work on clone
        let mut new_tiles = map.tiles.clone();

        for y in 4..SCREEN_HEIGHT - 1 {
            for x in 4..SCREEN_WIDTH - 1 {
                let neighbours = Architect::count_neighbours(x, y, map);
                let idx = get_idx(x, y);
                if neighbours > 4 || neighbours == 0 {
                    new_tiles[idx] = TileType::Wall;
                } else {
                    new_tiles[idx] = TileType::Floor;
                }
            }
        }

        map.tiles = new_tiles;
    }

    fn find_start(map: &Map) -> Point {
        let center = Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2);
        let closest_point = map
            .tiles
            .iter()
            .enumerate()
            .filter(|(_, t)| **t == TileType::Floor)
            .map(|(idx, _)| {
                (
                    idx,
                    DistanceAlg::Pythagoras
                        .distance2d(center, map.index_to_point2d(idx)),
                )
            })
            .min_by(|(_, distance), (_, distance2)| {
                distance.partial_cmp(distance2).unwrap()
            })
            .map(|(idx, _)| idx)
            .unwrap();

        map.index_to_point2d(closest_point)
    }
}

impl MapArchitect for Architect {
    fn build(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mut mb = MapBuilder::empty();
        Architect::random_noise_map(rng, &mut mb.map);
        for _ in 0..10 {
            Architect::iteration(&mut mb.map);
        }
        let start = Architect::find_start(&mb.map);
        mb.monster_spawns = mb.spawn_monsters(start, rng);
        mb.player_start = start;
        mb.amulet_start = mb.find_most_distant();
        mb.theme = Some(super::themes::Forest::build());
        mb
    }
}
