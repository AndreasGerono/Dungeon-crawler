#![warn(clippy::all, clippy::pedantic)]

use crate::prelude::*;

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

#[derive(Clone, Copy,Debug, PartialEq, Eq)]
pub struct WantsToMove {
    pub entity: Entity, // Reference to entity in legion that wants to move
    pub destination: Point
}
