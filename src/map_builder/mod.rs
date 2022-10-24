#![warn(clippy::all, clippy::pedantic)]

mod automata;
mod empty;
mod rooms;
mod drunkard;

use crate::prelude::*;

const MAX_NUM_ROOMS: usize = 25;

trait MapArchitect {
    fn build(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder;
}

pub struct MapBuilder {
    pub map: Map,
    pub rooms: Vec<Rect>,
    pub monster_spawns: Vec<Point>,
    pub player_start: Point,
    pub amulet_start: Point,
}

trait Relation {
    fn apart(&self, other: &Self, dist: i32) -> bool;
}

impl Relation for Rect {
    fn apart(&self, other: &Rect, dist: i32) -> bool {
        self.intersect(other)
            || i32::abs(self.y1 - other.y2) <= dist
            || i32::abs(self.x1 - other.x2) <= dist
            || i32::abs(self.y2 - other.y1) <= dist
            || i32::abs(self.x2 - other.x1) <= dist
    }
}

impl MapBuilder {
    pub fn new(rng: &mut RandomNumberGenerator) -> Self {
        let mut architect: Box<dyn MapArchitect> = match rng.range(0, 3) {
            0 => Box::new(drunkard::Architect{}),
            1 => Box::new(automata::Architect{}),
            _ => Box::new(rooms::Architect{}),
        };

        architect.build(rng)
    }

    fn fill(&mut self, tile: TileType) {
        self.map.tiles.iter_mut().for_each(|t| *t = tile);
    }

    fn find_most_distant(&self) -> Point {
        const UNREACHABLE: &f32 = &f32::MAX;

        let dijkstramap = DijkstraMap::new(
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            &[self.map.point2d_to_index(self.player_start)],
            &self.map,
            1024.0,
        );

        self.map.index_to_point2d(
            dijkstramap
                .map
                .iter()
                .enumerate()
                .filter(|(_, dist)| *dist < UNREACHABLE)
                .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
                .unwrap()
                .0,
        )
    }

    fn build_random_rooms(&mut self, rng: &mut RandomNumberGenerator) {
        let mut tries = MAX_NUM_ROOMS * 1000;
        while (tries > 0) && self.rooms.len() < MAX_NUM_ROOMS {
            tries -= 1;
            let room = Rect::with_size(
                rng.range(1, SCREEN_WIDTH - 10),
                rng.range(1, SCREEN_HEIGHT - 10),
                rng.range(3, 10),
                rng.range(3, 10),
            );
            let mut overlap = false;
            for r in &self.rooms {
                if r.apart(&room, 4) {
                    overlap = true;
                    break;
                }
            }
            if !overlap {
                room.for_each(|p| {
                    // Carve inside of a room if in bounds of a map
                    // Why we do not use in_bounds function??
                    // why > and not >= like in in_bounds?
                    if (p.x > 0 && p.x < SCREEN_WIDTH)
                        && (p.y > 0 && p.y < SCREEN_HEIGHT)
                    {
                        let idx = get_idx(p.x, p.y);
                        self.map.tiles[idx] = TileType::Floor;
                    }
                });

                self.rooms.push(room);
            }
        }
    }

    fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        use std::cmp::{max, min};
        for y in min(y1, y2)..=max(y1, y2) {
            if let Some(idx) = Map::try_idx(Point::new(x, y)) {
                self.map.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        use std::cmp::{max, min};
        for x in min(x1, x2)..=max(x1, x2) {
            if let Some(idx) = Map::try_idx(Point::new(x, y)) {
                self.map.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    fn build_corridors(&mut self, rng: &mut RandomNumberGenerator) {
        let mut rooms = self.rooms.clone();
        // sort rooms by center -> better hallways
        rooms.sort_by(|a, b| a.center().x.cmp(&b.center().x));

        for (i, room) in rooms.iter().enumerate().skip(1) {
            let prev = rooms[i - 1].center();
            let new = room.center();

            if rng.range(0, 2) == 1 {
                self.apply_horizontal_tunnel(prev.x, new.x, prev.y);
                self.apply_vertical_tunnel(prev.y, new.y, new.x);
            } else {
                self.apply_vertical_tunnel(prev.y, new.y, prev.x);
                self.apply_horizontal_tunnel(prev.x, new.x, new.y);
            }
        }
    }
    fn spawn_monsters(
        &self,
        start: Point,
        rng: &mut RandomNumberGenerator,
    ) -> Vec<Point> {

        const NUM_MONSTERS: usize = 50;

        let mut spawnable_tiles: Vec<Point> = self
            .map
            .tiles
            .iter()
            .enumerate()
            .filter(|(idx, t)| {
                **t == TileType::Floor
                    && DistanceAlg::Pythagoras
                        .distance2d(start, self.map.index_to_point2d(*idx))
                        > 10.0
            })
            .map(|(idx, _)| self.map.index_to_point2d(idx))
            .collect();

        let mut spawns = Vec::new();
        for _ in 0..NUM_MONSTERS {
            let target_index =
                rng.random_slice_index(&spawnable_tiles).unwrap();
            spawns.push(spawnable_tiles[target_index]);
            spawnable_tiles.remove(target_index);
        }

        spawns
    }
}

pub fn display(title: &str, map: &Map, player_start: Point, amulet_start: Point, monster_spawns: &[Point]) {
    use colored::Colorize;
    use std::io::stdin;
    let mut output = vec!['.'; NUM_TILES];

    map.tiles.iter().enumerate().for_each(|(idx, t)| {
        match *t {
            TileType::Floor => output[idx] = '.',
            TileType::Wall => output[idx] = '#'
        }
    });

    output[map.point2d_to_index(player_start)] = '@';
    output[map.point2d_to_index(amulet_start)] = 'A';
    for p in monster_spawns.iter() {
        output[map.point2d_to_index(*p)] = 'M';
    }

    print!("\x1B[2J"); // CLS!
    println!("----------------------\n{}\n----------------------", title.bright_yellow());
    for y in 0..SCREEN_HEIGHT {
        for x in 0..SCREEN_WIDTH {
            match output[get_idx(x,y)] {
                '#' => print!("{}", "#".bright_green()),
                '@' => print!("{}", "@".bright_yellow()),
                'M' => print!("{}", "M".bright_red()),
                'A' => print!("{}", "A".bright_magenta()),
                _ => print!("{}", ".".truecolor(64, 64, 64))
            }
        }
        println!();
    }

    let mut ignore_me = String::new();
    stdin()
        .read_line(&mut ignore_me)
        .expect("Failed to read line");
}
