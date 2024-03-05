use macroquad::prelude::*;
use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct TileMap {
    contents: Vec<Vec<Tile>>,
    image: Image,
    texture: Texture2D,
    updates: HashSet<UVec2>,
}

#[derive(Clone, Debug)]
pub struct Tile {
    image: Image,
}

impl Tile {
    pub const SIZE_PIXELS: usize = 8;
}
