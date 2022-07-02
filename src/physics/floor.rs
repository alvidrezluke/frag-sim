use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn spawn_floor(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 100.0 })),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..default()
        })
        .insert(Collider::cuboid(100.0, 0.1, 100.0))
        .insert(Friction {
            coefficient: 100000000.0,
            combine_rule: CoefficientCombineRule::Multiply
        })
        .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)));
}