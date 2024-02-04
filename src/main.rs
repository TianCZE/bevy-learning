mod asset_loader;
mod asteroids;
mod camera;
mod collision_detection;
mod debug;
mod despawn;
mod movement;
mod spaceship;

use crate::asset_loader::AssetLoaderPlugin;
use crate::asteroids::AsteroidsPlugin;
use crate::camera::CameraPlugin;
use crate::collision_detection::CollisionDetectionPlugin;
use crate::debug::DebugPlugin;
use crate::despawn::DespawnPlugin;
use crate::movement::MovementPlugin;
use crate::spaceship::SpaceshipPlugin;
use bevy::prelude::*;

fn main() {
    App::new()
        // Bevy built-ins.
        .insert_resource(ClearColor(Color::rgb(0.1, 0.0, 0.15)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 0.75,
        })
        .add_plugins(DefaultPlugins)
        // User defined plugins.
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(AsteroidsPlugin)
        // .add_plugins(DebugPlugin)
        .add_plugins(CollisionDetectionPlugin)
        .add_plugins(SpaceshipPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(DespawnPlugin)
        .run();
}
