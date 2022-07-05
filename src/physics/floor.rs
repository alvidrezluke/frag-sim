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
            material: materials.add(Color::rgb(0.9, 0.9, 0.9).into()),
            ..default()
        })
        .insert(Collider::cuboid(1000.0, 0.0, 100.0))
        .insert(Friction {
            coefficient: 100000000.0,
            combine_rule: CoefficientCombineRule::Multiply
        })
        .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, 1.0, 0.0)));
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box { min_x: -50.0, max_x: 50.0, min_y: 0.0, max_y: 75.0, min_z: 0.0, max_z: 1.0})),
            material: materials.add(Color::rgb(0.9, 0.9, 0.9).into()),
            ..default()
        })
        .insert(Collider::cuboid(100.0, 75.0, 1.0))
        .insert(Friction {
            coefficient: 100000000.0,
            combine_rule: CoefficientCombineRule::Multiply
        })
        .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, 0.0, 50.0)));
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box { min_x: -50.0, max_x: 50.0, min_y: 0.0, max_y: 75.0, min_z: 0.0, max_z: 1.0})),
            material: materials.add(Color::rgb(0.9, 0.9, 0.9).into()),
            ..default()
        })
        .insert(Collider::cuboid(100.0, 75.0, 1.0))
        .insert(Friction {
            coefficient: 100000000.0,
            combine_rule: CoefficientCombineRule::Multiply
        })
        .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, 0.0, -50.0)));
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box { min_x: 0.0, max_x: 1.0, min_y: 0.0, max_y: 75.0, min_z: -50.0, max_z: 50.0})),
            material: materials.add(Color::rgb(0.9, 0.9, 0.9).into()),
            ..default()
        })
        .insert(Collider::cuboid(100.0, 75.0, 1.0))
        .insert(Friction {
            coefficient: 100000000.0,
            combine_rule: CoefficientCombineRule::Multiply
        })
        .insert_bundle(TransformBundle::from(Transform::from_xyz(50.0, 0.0, 0.0)));
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box { min_x: 0.0, max_x: 1.0, min_y: 0.0, max_y: 75.0, min_z: -50.0, max_z: 50.0})),
            material: materials.add(Color::rgb(0.9, 0.9, 0.9).into()),
            ..default()
        })
        .insert(Collider::cuboid(100.0, 75.0, 1.0))
        .insert(Friction {
            coefficient: 100000000.0,
            combine_rule: CoefficientCombineRule::Multiply
        })
        .insert_bundle(TransformBundle::from(Transform::from_xyz(-50.0, 0.0, 0.0)));
}

pub fn spawn_light(
    mut commands: Commands
) {
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 1.0
    })
}