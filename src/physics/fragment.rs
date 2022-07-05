use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use csv::Writer;
use rand::prelude::*;
use std::{error::Error, fs};

use super::{sim_settings::SimSettings, grenade::{GrenadeData, Grenade}, GrenadeState};

#[derive(Component)]
pub struct Fragment;

pub fn generate_fragments(
    grenade_data: Res<GrenadeData>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    sim_settings: Res<SimSettings>,
    mut commands: Commands
) {
    // println!("Pos: {}, LinVel: {}, AngVel: {}", grenade_data.last_location.translation, grenade_data.last_vel.linvel, grenade_data.last_vel.angvel);
    for _ in 0..sim_settings.fragment_count {

        let mut rng = thread_rng();
        let offset_x: f32 = rng.gen_range(-50..50) as f32 / 100.0;
        let offset_y: f32 = rng.gen_range(-50..50) as f32 / 100.0;
        let offset_z: f32 = rng.gen_range(-50..50) as f32 / 100.0;
        // W = F * d = 1/2 m v^2
        let vel_x: f32 = grenade_data.last_vel.linvel.x + (sim_settings.explosion_vel * offset_x);
        let vel_y: f32 = grenade_data.last_vel.linvel.y + (sim_settings.explosion_vel * offset_y);
        let vel_z: f32 = grenade_data.last_vel.linvel.z + (sim_settings.explosion_vel * offset_z);
        commands.spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube {size: 0.05})),
            material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
            ..default()
        })
        .insert(Fragment)
        .insert(RigidBody::Dynamic)
        .insert(Velocity {
            linvel: Vec3::new(vel_x, vel_y, vel_z),
            angvel: grenade_data.last_vel.angvel,
        })
        .insert(Collider::cuboid(0.1, 0.1, 0.1))
        .insert(Friction::coefficient(sim_settings.friction))
        .insert(ExternalForce {..default()})
        .insert_bundle(TransformBundle::from(Transform::from_xyz(grenade_data.last_location.translation.x + offset_x, grenade_data.last_location.translation.y + offset_y, grenade_data.last_location.translation.z + offset_z)));
    }
}

pub fn write_fragment_data(
    fragments: Query<(&Transform, &Velocity), With<Fragment>>,
    time: Res<Time>,
) {
    println!("{:?}", time.delta());
    let result = write_to_file(fragments, time);
}

fn write_to_file(fragments: Query<(&Transform, &Velocity), With<Fragment>>, time: Res<Time>) -> Result<(), Box<dyn Error>>{
    let mut wtr = Writer::from_path("data.csv")?;
    let mut record = vec![];
    record.push(format!("{}", time.delta().as_millis()));
    for (pos, vel) in fragments.iter() {
        record.push(format!("XPos:{}|Ypos:{}|Zpos:{}|Xvel:{}|Yvel:{}|Zvel:{}", pos.translation.x, pos.translation.y, pos.translation.z, vel.linvel.x, vel.linvel.y, vel.linvel.z));
    }
    wtr.write_record(record)?;
    wtr.flush()?;
    Ok(())
}

pub fn clean_fragments(
    fragments: Query<Entity, With<Fragment>>,
    mut grenade_state: ResMut<State<GrenadeState>>,
    mut commands: Commands
) {
    for fragment in fragments.iter() {
        commands.entity(fragment).despawn_recursive();
    }
    grenade_state.set(GrenadeState::Grenade);
}