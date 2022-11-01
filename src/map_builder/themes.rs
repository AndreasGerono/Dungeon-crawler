#![warn(clippy::all, clippy::pedantic)]

use crate::prelude::*;

pub struct Dungeon {}

impl Dungeon {
    pub fn build() -> Box<dyn MapTheme> {
        Box::new(Self {})
    }
}

impl MapTheme for Dungeon {
    fn tile_render(&self, tile_type: TileType) -> FontCharType {
        match tile_type {
            TileType::Floor => to_cp437('.'),
            TileType::Wall => to_cp437('#'),
        }
    }
}

pub struct Forest {}

impl Forest {
    pub fn build() -> Box<dyn MapTheme> {
        Box::new(Self {})
    }
}

impl MapTheme for Forest {
    fn tile_render(&self, tile_type: TileType) -> FontCharType {
        match tile_type {
            TileType::Floor => to_cp437(';'),
            TileType::Wall => to_cp437('"'),
        }
    }
}
