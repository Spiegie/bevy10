use bevy::prelude::*;
use bevy_ecs_tilemap::helpers::square_grid::neighbors::Neighbors;
use bevy_ecs_tilemap::prelude::*;

pub fn spawn_tilemap(
    mut commands: Commands, 
    asset_server:Res<AssetServer>,
) {
    let texture_handle: Handle<Image> = asset_server.load("tiles.png");

    let map_size = TilemapSize { x: 5, y: 5 };

    let tilemap_entity = commands.spawn_empty().id();

    let mut tile_storage = TileStorage::empty(map_size);

    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let tile_pos = TilePos { x, y };
            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    texture_index: TileTextureIndex(1),
                    visible: TileVisible(true),
                    flip: TileFlip{x:false, y:false, d:false},
                    ..Default::default()
                })
                .insert(BlockingTile{blocking: false, ..default()})
                .id();
            if y % 2 == 0 {
                commands
                    .entity(tile_entity)
                    .insert(BlockingTile{blocking: true, layer: 5});
            }
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    let tile_size = TilemapTileSize { x: 16.0, y: 16.0 };
    let grid_size = tile_size.into();
    let map_type = TilemapType::default();

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle),
        tile_size,
        transform: get_tilemap_center_transform(&map_size, &grid_size,  &map_type, 0.0),
        ..Default::default()
    });

}


pub fn get_tilemap(
    mut commands: Commands,
    mut tilemap_query: Query<&TileStorage>,
    tile_query: Query<&BlockingTile>,
) {
    let tilemap = tilemap_query.get_single_mut().unwrap();

    let tilemap_size = &tilemap.size;
    for x in 0..tilemap_size.x {
        for y in 0..tilemap_size.y/2 {
            let entity = &tilemap.get(&TilePos{x,y}).unwrap();
            let _is_blocking = tile_query.get_component::<BlockingTile>(*entity).unwrap().blocking;
            let _layer = tile_query.get_component::<BlockingTile>(*entity).unwrap().layer;
            commands 
                .entity(*entity)
                .insert(TileTextureIndex(3)
            );
        }
    }
}


#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct BlockingTile{ 
    blocking: bool,
    layer: u32
}