#![warn(clippy::all, clippy::pedantic)]

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TurnState {
    AwaitingInput,
    PlayerTurn,
    MonsterTurn,
}
