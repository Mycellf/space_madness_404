use macroquad::prelude::*;
use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct TileMap {
    pub contents: Vec<Vec<Tile>>,
    pub image: Image,
    pub texture: Texture2D,
    pub updates: HashSet<UVec2>,
}

#[derive(Clone, Debug)]
pub struct Tile {
    pub image: Image,
}

impl Tile {
    pub const SIZE_PIXELS: usize = 8;
}
