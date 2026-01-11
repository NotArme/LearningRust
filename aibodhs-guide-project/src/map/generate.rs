use bevy_procedural_tilemaps::prelude::*;
use bevy::prelude::*;

use crate::map::{
    assets::{load_assets, prepare_tilemap_handles},
    rules::build_world,
};


pub const MAP_SIZE_X: u32 = 25;
pub const MAP_SIZE_Y: u32 = 18;
pub const MAP_SIZE_Z: u32 = 1;

const ASSETS_PATH: &str = "map";
const TILEMAP_FILE: &str = "tilemap.png";

pub const TILE_SIZE: f32 = 32.;
const NODE_SIZE: Vec3 = Vec3::new(TILE_SIZE, TILE_SIZE, 1.);

const ASSETS_SCALE: Vec3 = Vec3::ONE;

pub fn map_pixel_dimensions() -> Vec2 {
    Vec2::new(TILE_SIZE * MAP_SIZE_X as f32, TILE_SIZE * MAP_SIZE_Y as f32)
}


pub fn setup_generator(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>
) {
     // 1. Rules Initialization - Get tile definitions and connection rules
    let (assets_definitions, models, socket_collection) = build_world();

    let rules = RulesBuilder::new_cartesian_3d(models, socket_collection)
        .with_rotation_axis(Direction::ZForward)
        .build()
        .unwrap();


     // 2. Grid - Create 3D world space with wrapping behavior (false, false, false)
    let grid = CartesianGrid::new_cartesian_3d(MAP_SIZE_X, MAP_SIZE_Y, MAP_SIZE_Z, false, false, false);


    // 3. Configuring the Algorithm - Set up WFC behavior
    let gen_builder = GeneratorBuilder::new()
        .with_rules(rules)
        .with_grid(grid.clone())
        .with_rng(RngMode::RandomSeed)
        .with_node_heuristic(NodeSelectionHeuristic::MinimumRemainingValue)
        .with_model_heuristic(ModelSelectionHeuristic::WeightedProbability);

    let generator = gen_builder.build().unwrap();


     // 4. Loading Assets - Load sprite atlas and convert to renderable assets
    let tilemap_handles = prepare_tilemap_handles(&asset_server, &mut atlas_layouts, ASSETS_PATH, TILEMAP_FILE);
    let models_assets = load_assets(&tilemap_handles, assets_definitions);


    // 5. Spawning the Generator - Create entity with Transform and NodesSpawner
    commands.spawn((
        Transform::from_translation(Vec3 {
            x: -TILE_SIZE * grid.size_x() as f32 / 2.,
            y: -TILE_SIZE * grid.size_y() as f32 / 2.,
            z: 0.,
        }),
        grid,
        generator,
        NodesSpawner::new(models_assets, NODE_SIZE, ASSETS_SCALE).with_z_offset_from_y(true),
    ));
}