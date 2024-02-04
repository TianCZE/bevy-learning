use crate::spaceship::Spaceship;
use bevy::prelude::*;

const DESPAWN_DISTANCE: f32 = 100.0;

pub struct DespawnPlugin;

impl Plugin for DespawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, despawn_far_away_entities);
    }
}

fn despawn_far_away_entities(
    mut commands: Commands,
    query: Query<(Entity, &GlobalTransform), Without<Spaceship>>,
) {
    for (enity, transform) in query.iter() {
        let distance = transform.translation().distance(Vec3::ZERO);
        if distance > DESPAWN_DISTANCE {
            commands.entity(enity).despawn_recursive();
        }
    }
}
