use super::map::Map;
use super::rooms::{Edge, Room};
use super::tiles::TileType;
use bevy::prelude::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_tiles);
    }
}

fn generate_map() -> Map {
    let mut map = Map::new();

    map.insert_room(Room::new(10, 10, 17, 9));
    map.insert_room(Room::new(33, 19, 21, 29));
    map.insert_room(Room::new(69, 18, 14, 25));

    let room_1 = map.rooms[0];
    let room_2 = map.rooms[1];
    let room_3 = map.rooms[2];

    map.connect_rooms(room_1, room_2);
    map.connect_rooms(room_1, room_3);
    map.connect_rooms(room_2, room_3);

    map
}

fn spawn_tiles(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let map = generate_map();

    let texture_handle = asset_server.load("colored-transparent.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(16.0, 16.0),
        49,
        22,
        Some(Vec2::new(1.0, 1.0)),
        None,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    for (y, row) in map.tiles.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            let index = tile.get_index();
            commands.spawn(SpriteSheetBundle {
                texture_atlas: texture_atlas_handle.clone(),
                sprite: TextureAtlasSprite::new(index),
                transform: Transform::from_xyz(x as f32, y as f32, 1.0)
                    .with_scale(Vec3::new(0.065, 0.065, 1.0)),
                ..default()
            });
        }
    }
}
