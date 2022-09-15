#![warn(clippy::all, clippy::pedantic)]

use crate::prelude::*;

const NUM_TITLES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct Map {
    pub tiles: Vec<TileType>,
}

pub fn get_idx(x: i32, y: i32) -> usize {
    ((y * SCREEN_WIDTH) + x) as usize
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TITLES],
        }
    }

    pub fn render(&self, ctx: &mut BTerm) {
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                let idx = get_idx(x, y);
                match self.tiles[idx] {
                    TileType::Floor => {
                        ctx.set(x, y, YELLOW, BLACK, to_cp437('.'));
                    }
                    TileType::Wall => {
                        ctx.set(x, y, GREEN, BLACK, to_cp437('#'));
                    }
                }
            }
        }
    }

    pub fn in_bounds(point: Point) -> bool {
        (point.x >= 0 && point.x < SCREEN_WIDTH)
            && (point.y >= 0 && point.y < SCREEN_HEIGHT)
    }

    pub fn can_enter_tile(&self, point: Point) -> bool {
        Self::in_bounds(point)
            && self.tiles[get_idx(point.x, point.y)] == TileType::Floor
    }

    pub fn try_idx(point: Point) -> Option<usize> {
        if Self::in_bounds(point) {
            Some(get_idx(point.x, point.y))
        } else {
            None
        }
    }
}
