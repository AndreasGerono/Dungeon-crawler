#![warn(clippy::all, clippy::pedantic)]

use crate::prelude::*;
use std::collections::HashSet;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Render {
    pub color: ColorPair,
    pub glyph: FontCharType,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Player;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Enemy;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MovingRandomly;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ChasingPlayer;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WantsToMove {
    pub entity: Entity, // Reference to entity in legion that wants to move
    pub destination: Point,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Name(pub String);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WantsToAttack {
    pub attacker: Entity,
    pub victim: Entity,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Item;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AmuletOfYala;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ProvidesHealing {
    pub amount: i32
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ProvidesDungeonMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Carried(pub Entity);
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ActivateItem {
    pub used_by: Entity,
    pub item: Entity,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FieldOfView {
    pub visable_tiles: HashSet<Point>,
    pub radius: i32,
    pub is_dirty: bool,
}

impl FieldOfView {
    pub fn new(radius: i32) -> Self {
        Self {
            visable_tiles: HashSet::new(),
            radius,
            is_dirty: true,
        }
    }

    pub fn clone_dirty(&self) -> Self {
        Self {
            visable_tiles: HashSet::new(),
            radius: self.radius,
            is_dirty: true,
        }
    }
}
