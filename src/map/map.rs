use bevy::prelude::*;
use rand::seq::SliceRandom;

use super::{rooms::Room, tiles::TileType};

#[derive(Debug)]
pub struct Map {
    pub tiles: Vec<Vec<TileType>>,
    pub rooms: Vec<Room>,
}

impl Map {
    const WIDTH: usize = 101;
    const HEIGHT: usize = 61;

    pub fn new() -> Self {
        let tiles = vec![vec![TileType::Wall; Map::WIDTH]; Map::HEIGHT];
        let mut map = Self {
            tiles,
            rooms: Vec::new(),
        };
        map.set_borders();
        map
    }

    pub fn insert_room(&mut self, room: Room) {
        let x = room.position.x;
        let y = room.position.y;
        let w = room.w as f32;
        let h = room.h as f32;

        let start = room.position;
        let end = Vec2::new(x + w, y + h);

        self.set_rect(start, end, TileType::Floor);

        self.rooms.push(room);
    }

    pub fn set(&mut self, pos: Vec2, tile: TileType) {
        self.tiles[pos.y as usize][pos.x as usize] = tile;
    }

    pub fn set_rect(&mut self, start: Vec2, end: Vec2, tile: TileType) {
        let sx = start.x as usize;
        let sy = start.y as usize;
        let ex = end.x as usize;
        let ey = end.y as usize;

        for i in sy..ey {
            for j in sx..ex {
                self.tiles[i][j] = tile;
            }
        }
    }

    pub fn set_horizontal(&mut self, start: Vec2, length: usize, tile: TileType) {
        let sx = start.x as usize;
        let sy = start.y as usize;

        for i in sx..sx + length {
            self.tiles[sy][i] = tile;
        }
    }

    pub fn set_vertical(&mut self, start: Vec2, length: usize, tile: TileType) {
        let sx = start.x as usize;
        let sy = start.y as usize;

        for i in sy..sy + length {
            self.tiles[i][sx] = tile;
        }
    }

    fn set_borders(&mut self) {
        // bottom
        self.set_horizontal(Vec2::new(0.0, 0.0), Map::WIDTH, TileType::Impassable);

        // top
        self.set_horizontal(
            Vec2::new(0.0, (Map::HEIGHT - 1) as f32),
            Map::WIDTH,
            TileType::Impassable,
        );

        // left
        self.set_vertical(Vec2::new(0.0, 0.0), Map::HEIGHT, TileType::Impassable);

        // right
        self.set_vertical(
            Vec2::new((Map::WIDTH - 1) as f32, 0.0),
            Map::HEIGHT,
            TileType::Impassable,
        );
    }

    pub fn connect_rooms(&mut self, start: Room, end: Room) {
        let mut current_pos = start.center();
        info!("Found edge at: {:?}", current_pos);
        let target = end.center();

        let mut directions = [Vec2::X, Vec2::NEG_X, Vec2::Y, Vec2::NEG_Y];

        let mut distance = target - current_pos;

        loop {
            directions.shuffle(&mut rand::thread_rng());
            self.set(current_pos, TileType::Floor);
            if distance.x.abs() < 2.0 && distance.y.abs() < 2.0 {
                info!("Distance is too low. Breaking: {:?}", distance);
                break;
            }

            for dir in directions {
                let new_pos = current_pos + dir;
                let new_dist = target - new_pos;

                if new_dist.x.abs() < distance.x.abs() || new_dist.y.abs() < distance.y.abs() {
                    info!("Old: {:?} - New: {:?}", distance, new_dist);
                    info!("Using dir: {:?}", dir);
                    info!("New pos: {:?}", new_pos);
                    current_pos = new_pos;
                    distance = new_dist;
                    break;
                }
            }
        }
    }
}
