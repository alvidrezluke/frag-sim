use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use csv::Writer;
use rand::prelude::*;
use std::{error::Error, fs};

use super::{sim_settings::SimSettings, grenade::GrenadeData, GrenadeState};

#[derive(Component)]
pub struct Fragment;

pub fn generate_fragments(
    grenade_data: Res<GrenadeData>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    sim_settings: Res<SimSettings>,
    mut commands: Commands
) {
    let res = fs::remove_file(sim_settings.csv_location.clone());
    if res.is_err() {
        println!("Could not delete file.")
    }
    for _ in 0..sim_settings.fragment_count {
        let mut rng = thread_rng();
        let offset_x: f32 = rng.gen_range(-50..50) as f32 / 100.0;
        let offset_y: f32 = rng.gen_range(-50..50) as f32 / 100.0;
        let offset_z: f32 = rng.gen_range(-50..50) as f32 / 100.0;
        let vel_x: f32 = grenade_data.last_vel.linvel.x + (sim_settings.explosion_vel * offset_x);
        let vel_y: f32 = grenade_data.last_vel.linvel.y + (sim_settings.explosion_vel * offset_y);
        let vel_z: f32 = grenade_data.last_vel.linvel.z + (sim_settings.explosion_vel * offset_z);
        let x_pos: f32 = grenade_data.last_location.translation.x + offset_x;
        let mut y_pos: f32 = grenade_data.last_location.translation.y + offset_y;
        let z_pos: f32 = grenade_data.last_location.translation.z + offset_z;
        if y_pos < 0.0 {
            y_pos = 0.0;
        }
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
        .insert(ColliderMassProperties::Density(4.0))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(x_pos, y_pos, z_pos)));
    }
}

pub fn write_fragment_data(
    fragments: Query<(&Transform, &Velocity), With<Fragment>>,
    time: Res<Time>,
    sim_settings: Res<SimSettings>
) {
    let result = write_to_file(fragments, time, sim_settings.csv_location.clone());
    if result.is_err() {
        println!("Could not output data to file.")
    }
}

fn write_to_file(fragments: Query<(&Transform, &Velocity), With<Fragment>>, time: Res<Time>, path: String) -> Result<(), Box<dyn Error>>{
    let file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(path)
        .unwrap();
    let mut wtr = Writer::from_writer(file);
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
    mut commands: Commands
) {
    for fragment in fragments.iter() {
        commands.entity(fragment).despawn_recursive();
    }
}