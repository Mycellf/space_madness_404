use macroquad::prelude::*;
use std::{
    collections::HashSet,
    ops::{Index, IndexMut},
};

#[derive(Clone, Debug)]
pub struct TileMap {
    pub contents: Vec<Vec<Tile>>,
    pub image: Image,
    pub updates: HashSet<UVec2>,
}

impl TileMap {
    pub fn new(size: UVec2) -> Self {
        Self {
            contents: (0..size.x)
                .map(|_| {
                    (0..size.y)
                        .map(|_| Tile {
                            tile_type: TileType::Empty,
                        })
                        .collect()
                })
                .collect(),
            image: Image::gen_image_color(size.x as u16, size.y as u16, BLANK),
            updates: HashSet::new(),
        }
    }

    pub fn update_to_texture(&mut self, texture: &mut Texture2D) {
        for &update_index in &self.updates {
            let update_translation = update_index * Tile::SIZE_PIXELS;

            texture.update_part(
                &self.image,
                update_translation.x as i32,
                update_translation.y as i32,
                Tile::SIZE_PIXELS as i32,
                Tile::SIZE_PIXELS as i32,
            )
        }

        self.updates.clear();
    }

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

        self.updates.insert(index);

        Some(())
    }

    pub fn update_tile(&mut self, index: UVec2) -> Option<()> {
        if index.x >= self.size().x && index.y >= self.size().y {
            return None;
        }

        self.updates.insert(index);

        Some(())
    }

    pub fn size(&self) -> UVec2 {
        UVec2::new(self.contents.len() as u32, self.contents[0].len() as u32)
    }
}

impl Index<UVec2> for TileMap {
    type Output = Tile;

    fn index(&self, index: UVec2) -> &Self::Output {
        self.get(index).unwrap()
    }
}

impl IndexMut<UVec2> for TileMap {
    fn index_mut(&mut self, index: UVec2) -> &mut Self::Output {
        self.get_mut(index).unwrap()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Tile {
    pub tile_type: TileType,
}

impl Tile {
    pub const SIZE_PIXELS: u32 = 8;
}

#[derive(Clone, Copy, Debug)]
pub enum TileType {
    Empty,
}
