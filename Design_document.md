# Design Document

## Project Name: Rust Roglike

## Short Description

A dungeon crawler with procedurally generated levels, monsters of increasing difficulty and turn-based movments

## Story

The hero's hometown is suffering from a plague of monsters. Weulling uo from the deep, they seem unstoppable. Legend tells of the Amulet of Yala - Yet Another Lost Amulet - that can be used to steam the tide. After a long night a the tavern, the hero promises to save the day - and sets forth into the dungeon.

## Basic Game Loops

1. Enter dungeon level,
2. explote, revealing the map,
3. encounter enemies whom the player fights or flees from,
4. find power-ups and use the to strehgthen the player,
5. locate the exit to the level - go to 1.

## Minimum Viable Product

1. Create a basic dungeon map,
2. place the player adn let them walk around,
3. spawn monsters, draw them and let the player kill them by walking into them,
4. add health and combat system that uses it,
5. add healing potions,
6. display a "game over" screen when the player dies,
7. add the Amulet of Yala to the lebel and let the player win by reaching it.

## Stretch goals

1. Add Fields-of-View,
2. add more interesting dungeon designs,
3. add some dungeon themes,
4. add multiple layers to the dungeon, with the Amulet on the last one,
5. add varied weapons to the game,
6. move to a data-driven design for spawning enemies,
7. consider some visual effects to make combat more visceral,
8. consider keeping score.
