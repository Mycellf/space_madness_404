use macroquad::prelude::*;
use rapier2d::prelude::*;
use std::{
    collections::HashSet,
    ops::{Index, IndexMut},
};

#[derive(Clone, Debug)]
pub struct TileMap {
    pub contents: Vec<Vec<Tile>>,
    pub updates: HashSet<UVec2>,
    pub tile_images: Vec<Option<Image>>,
}

impl TileMap {
    pub async fn new(size: UVec2) -> Self {
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
            updates: HashSet::new(),
            tile_images: TileType::load_images().await,
        }
    }

    pub fn update_to_texture(&mut self, texture: &mut Texture2D) {
        for &update_index in &self.updates {
            let update_translation = update_index * Tile::SIZE_PIXELS;

            match &self.tile_images[self[update_index].tile_type as usize] {
                Some(image) => {
                    texture.update_part(
                        image,
                        update_translation.x as i32,
                        update_translation.y as i32,
                        Tile::SIZE_PIXELS as i32,
                        Tile::SIZE_PIXELS as i32,
                    );
                }
                None => (),
            }
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
        uvec2(self.contents.len() as u32, self.contents[0].len() as u32)
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
    pub const SIZE_TEXELS: u32 = 8;
    pub const SIZE_PIXELS: u32 = Tile::SIZE_TEXELS * 2;
}

#[derive(Clone, Copy, Debug)]
#[repr(usize)]
pub enum TileType {
    Empty,
    Wall,
}

impl TileType {
    pub const TYPES: [TileType; 2] = [Self::Empty, Self::Wall];

    pub fn path_to_image(self) -> Option<&'static str> {
        match self {
            Self::Empty => None,
            Self::Wall => Some("assets/wall.png"),
        }
    }

    pub fn shape(self) -> Option<TileShape> {
        match self {
            Self::Empty => None,
            Self::Wall => Some(TileShape::Box),
        }
    }

    pub async fn load_images() -> Vec<Option<Image>> {
        let mut images = Vec::with_capacity(TileType::TYPES.len());

        for tile_type in TileType::TYPES {
            let image = match tile_type.path_to_image() {
                Some(path) => Some(load_image(path).await.unwrap()),
                None => None,
            };

            images.push(image);
        }

        images
    }
}

#[derive(Clone, Copy, Debug)]
pub enum TileShape {
    Box,
}

impl TileShape {
    pub fn get_shape(self) -> Polyline {
        match self {
            Self::Box => Polyline::new(
                vec![
                    point![0.0, 0.0],
                    point![1.0, 0.0],
                    point![1.0, 1.0],
                    point![0.0, 1.0],
                ],
                Some(loop_indicies(4)),
            ),
        }
    }
}

fn loop_indicies(length: u32) -> Vec<[u32; 2]> {
    (0..length).map(|i| [i, (i + 1) % length]).collect()
}
