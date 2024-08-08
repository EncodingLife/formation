use apis::bevy::shape::*;
use apis::prelude::*;
use bevy::{
    color::palettes::css::{GREEN, PURPLE, WHITE},
    prelude::*,
    sprite::MaterialMesh2dBundle,
};
use bevy_pancam::{PanCam, PanCamPlugin};
use simulation::map::spawn_info::get_spawn_coords;

const CELL_SIZE: f32 = 64.0;

pub fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PanCamPlugin::default()))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn(Camera2dBundle::default())
        .insert(PanCam::default());
    let orientation = HexOrientation::Flat;
    let world_shape = HexWorldShape::Square(11, orientation);
    let indexer = MapIndex::new(world_shape);
    let world = HexWorld::new(world_shape, CELL_SIZE);
    let mesh_shape = Hexagon::new(orientation, CELL_SIZE - 1.0);

    let mesh_handle = meshes.add(mesh_shape);
    let circle_mesh = meshes.add(Circle::new(16.0));

    for i in 0..indexer.capacity() {
        let coords = indexer.coord(i);
        let translation = world.coord_to_world_v3(coords);
        let colour: Color = WHITE.into();
        let material: Handle<ColorMaterial> = materials.add(ColorMaterial::from(colour));
        commands.spawn(MaterialMesh2dBundle {
            mesh: mesh_handle.clone().into(),
            material: material.clone(),
            transform: Transform::from_translation(translation),
            ..Default::default()
        });
    }

    for coords in get_spawn_coords(world_shape, 10) {
        {
            let translation = world.coord_to_world_v3(coords);
            let colour: Color = PURPLE.into();
            let material: Handle<ColorMaterial> = materials.add(ColorMaterial::from(colour));
            commands.spawn(MaterialMesh2dBundle {
                mesh: circle_mesh.clone().into(),
                material: material.clone(),
                transform: Transform::from_translation(translation.with_z(1.0)),
                ..Default::default()
            });
        }
        let reflection = (coords - world.center()).reflect_q() + world.center();
        {
            let translation = world.coord_to_world_v3(reflection);
            let colour: Color = GREEN.into();
            let material: Handle<ColorMaterial> = materials.add(ColorMaterial::from(colour));
            commands.spawn(MaterialMesh2dBundle {
                mesh: circle_mesh.clone().into(),
                material: material.clone(),
                transform: Transform::from_translation(translation.with_z(1.0)),
                ..Default::default()
            });
        }
    }
}
