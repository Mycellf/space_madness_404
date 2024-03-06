use macroquad::prelude::*;
use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct TileMap {
    pub contents: Vec<Vec<Tile>>,
    pub image: Image,
    pub texture: Texture2D,
    pub updates: HashSet<UVec2>,
}

impl TileMap {
    const TILE_SIZE: u32 = 8;

    pub fn get(&self, index: UVec2) -> Option<&Tile> {
        let index = (index.x as usize, index.y as usize);

        Some(self.contents.get(index.0)?.get(index.1)?)
    }

    pub fn get_mut(&mut self, index: UVec2) -> Option<&mut Tile> {
        let index = (index.x as usize, index.y as usize);

        Some(self.contents.get_mut(index.0)?.get_mut(index.1)?)
    }

    pub fn set(&mut self, index: UVec2, tile: Tile) -> Option<()> {
        *self.get_mut(index)? = tile;

        Some(())
    }

    pub fn update_full_texture(&mut self) {
        self.texture.update(&self.image);
    }

    pub fn update_tile_texture(&mut self, tile: UVec2) -> Option<()> {
        let tile_translation = tile * Self::TILE_SIZE;

        self.texture.update_part(
            &self.get(tile)?.image,
            tile_translation.x as i32,
            tile_translation.y as i32,
            Self::TILE_SIZE as i32,
            Self::TILE_SIZE as i32,
        );

        Some(())
    }

    pub fn size(&self) -> UVec2 {
        UVec2::new(self.contents.len() as u32, self.contents[0].len() as u32)
    }
}

#[derive(Clone, Debug)]
pub struct Tile {
    pub image: Image,
}

impl Tile {
    pub const SIZE_PIXELS: usize = 8;
}
