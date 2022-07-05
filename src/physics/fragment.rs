use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use csv::Writer;
use rand::prelude::*;
use std::{error::Error, fs};

use super::{sim_settings::SimSettings, grenade::GrenadeData};

#[derive(Component)]
pub struct Fragment;

// Generate all fragments
pub fn generate_fragments(
    grenade_data: Res<GrenadeData>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    sim_settings: Res<SimSettings>,
    mut commands: Commands
) {
    // Remove data file if one exists
    let res = fs::remove_file(sim_settings.csv_location.clone());

    // If it does not exist fail quietly.
    if res.is_err() {
        println!("Could not delete file.")
    }

    // Iterate through the size of fragment count
    for _ in 0..sim_settings.fragment_count {

        // Initialize a random-number-generator
        let mut rng = thread_rng();

        // Calculate random offsets from grenade
        let offset_x: f32 = rng.gen_range(-100..100) as f32 / 100.0;
        let offset_y: f32 = rng.gen_range(-100..100) as f32 / 100.0;
        let offset_z: f32 = rng.gen_range(-100..100) as f32 / 100.0;

        // Calculate velocity from the offsets
        let vel_x: f32 = grenade_data.last_vel.linvel.x + (sim_settings.explosion_vel * offset_x);
        let vel_y: f32 = grenade_data.last_vel.linvel.y + (sim_settings.explosion_vel * offset_y);
        let vel_z: f32 = grenade_data.last_vel.linvel.z + (sim_settings.explosion_vel * offset_z);

        // Set positions
        let mut x_pos: f32 = grenade_data.last_location.translation.x + offset_x;
        let mut y_pos: f32 = grenade_data.last_location.translation.y + offset_y;
        let mut z_pos: f32 = grenade_data.last_location.translation.z + offset_z;

        // Ensure that they stay contained inside the box
        if x_pos < 0.0 {
            x_pos = 0.0;
        } else if x_pos > 100.0 {
            x_pos = 100.0;
        }
        if y_pos < 0.0 {
            y_pos = 0.0;
        }
        if z_pos < 0.0 {
            z_pos = 0.0;
        } else if z_pos > 100.0 {
            z_pos = 100.0;
        }

        // Spawn a fragment
        commands.spawn_bundle(PbrBundle {
            
            // Set a mesh for the fragment in the shape of a cube
            mesh: meshes.add(Mesh::from(shape::Cube {size: 0.05})),

            // Set the color to red
            material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),

            // Initialize all other functions to their defaults
            ..default()
        })

        // Add component to each fragment
        .insert(Fragment)

        // Add component for physics engine calculations
        .insert(RigidBody::Dynamic)

        // Set initial velocity to the last velocity linearly
        // TODO: Add random angular velocity
        .insert(Velocity {
            linvel: Vec3::new(vel_x, vel_y, vel_z),
            angvel: grenade_data.last_vel.angvel,
        })

        // Set a collider to the size of the mesh
        .insert(Collider::cuboid(0.1, 0.1, 0.1))

        // Set fricition to value from SimSettings
        .insert(Friction::coefficient(sim_settings.friction))

        // Set high density
        .insert(ColliderMassProperties::Density(100.0))

        // Move to position around the grenade explosion location
        .insert_bundle(TransformBundle::from(Transform::from_xyz(x_pos, y_pos, z_pos)));
    }
}

// Export data to csv during simulation
pub fn write_fragment_data(
    fragments: Query<(&Transform, &Velocity), With<Fragment>>,
    time: Res<Time>,
    sim_settings: Res<SimSettings>
) {
    // Try writing to file
    let result = write_to_file(fragments, time, sim_settings.csv_location.clone());

    // Handle error
    if result.is_err() {
        println!("Could not output data to file.")
    }
}

fn write_to_file(fragments: Query<(&Transform, &Velocity), With<Fragment>>, time: Res<Time>, path: String) -> Result<(), Box<dyn Error>>{
    // Open file as editable and create it if not created
    let file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(path)
        .unwrap();
    
    // Create csv writer
    let mut wtr = Writer::from_writer(file);

    // Create blank vector to store row data
    let mut record = vec![];

    // Add the time since last update
    record.push(format!("{}", time.delta().as_millis()));

    // Iterate through every fragment and store the pos and vel at every update
    for (pos, vel) in fragments.iter() {
        record.push(format!("XPos:{}|Ypos:{}|Zpos:{}|Xvel:{}|Yvel:{}|Zvel:{}", pos.translation.x, pos.translation.y, pos.translation.z, vel.linvel.x, vel.linvel.y, vel.linvel.z));
    }

    // Write to file
    wtr.write_record(record)?;

    // Flush all changes to file and clear cache
    wtr.flush()?;

    // Return successful completion
    Ok(())
}

// Remove fragments when simulation ended
pub fn clean_fragments(
    fragments: Query<Entity, With<Fragment>>,
    mut commands: Commands
) {
    // Iterate through all fragments
    for fragment in fragments.iter() {

        // Despawn fragment
        commands.entity(fragment).despawn_recursive();
    }
}
