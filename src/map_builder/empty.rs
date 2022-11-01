#![warn(clippy::all, clippy::pedantic)]

use super::MapArchitect;
use crate::prelude::*;

pub struct Architect {}

impl MapArchitect for Architect {
    fn build(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mut mb = MapBuilder::empty();
        mb.fill(TileType::Floor);
        mb.player_start = Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2);
        mb.amulet_start = mb.find_most_distant();
        mb.theme = Some(super::themes::Dungeon::build());
        for _ in 0..50 {
            mb.monster_spawns.push(Point::new(
                rng.range(1, SCREEN_WIDTH),
                rng.range(1, SCREEN_HEIGHT),
            ));
        }
        mb
    }
}
