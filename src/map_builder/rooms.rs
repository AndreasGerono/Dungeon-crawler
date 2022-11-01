#![warn(clippy::all, clippy::pedantic)]

use super::MapArchitect;
use crate::prelude::*;

pub struct Architect {}

impl MapArchitect for Architect {
    fn build(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mut mb = MapBuilder::empty();
        mb.fill(TileType::Wall);
        mb.build_random_rooms(rng);
        mb.build_corridors(rng);
        mb.player_start = mb.rooms[0].center();
        mb.amulet_start = mb.find_most_distant();
        mb.theme = Some(super::themes::Dungeon::build());

        for room in mb.rooms.iter().skip(1) {
            mb.monster_spawns.push(room.center());
        }

        mb
    }
}
