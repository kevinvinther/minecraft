use bevy::{
    prelude::*,
};
use bevy_flycam::{FlyCam, NoCameraPlayerPlugin};

mod world_gen;

fn main() {
    // App::new()
    //     .insert_resource(Msaa { samples: 4 })   // Anti-Aliasing
    //     .add_plugins(DefaultPlugins)
    //     .add_plugin(NoCameraPlayerPlugin)   // FlyCam plugin
    //     .add_startup_system(setup)
    //     .run();

    world_gen::gen_noise_img();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Camera
    commands
        .spawn_bundle(Camera3dBundle::default())
        .insert(FlyCam);    // makes camera easy to manipulate for development

    // Temp Cube
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            // The cube has no material aka. missing texture type look to it
            ..default()
        });
}
