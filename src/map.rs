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

    pub fn valid_exit(&self, loc: Point, delta: Point) -> Option<usize> {
        let destination = loc + delta;
        if self.can_enter_tile(destination) {
            let idx = self.point2d_to_index(destination);
            Some(idx)
        } else {
            None
        }
    }
}

impl BaseMap for Map {
    fn is_opaque(&self, _idx: usize) -> bool {
        true
    }

    fn get_available_exits(&self, idx: usize) -> SmallVec<[(usize, f32); 10]> {
        let mut exits = SmallVec::new();

        let location = self.index_to_point2d(idx);
        // 1.0 is cost of moving (usually 1.0, but can be higher for diagonal
        //    movement)
        // also different terrain can have different cost
        if let Some(exit) = self.valid_exit(location, Point::new(-1, 0)) {
            exits.push((exit, 1.0));
        }
        if let Some(exit) = self.valid_exit(location, Point::new(1, 0)) {
            exits.push((exit, 1.0));
        }
        if let Some(exit) = self.valid_exit(location, Point::new(0, -1)) {
            exits.push((exit, 1.0));
        }
        if let Some(exit) = self.valid_exit(location, Point::new(0, 1)) {
            exits.push((exit, 1.0));
        }

        exits
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        DistanceAlg::Pythagoras.distance2d(
            self.index_to_point2d(idx1),
            self.index_to_point2d(idx2),
        )
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(SCREEN_WIDTH, SCREEN_HEIGHT)
    }

    fn in_bounds(&self, pos: Point) -> bool {
        Self::in_bounds(pos)
    }
}
