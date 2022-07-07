use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use std::fs;

use super::sim_settings::SimSettings;

#[derive(Component)]
pub struct Wall;

pub fn spawn_solid_surfaces(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,

) {
    // Spawn floor
    commands
        .spawn_bundle(PbrBundle {

            // Make a box 100x10x100
            mesh: meshes.add(Mesh::from(shape::Box {min_x: -50.0, max_x: 50.0, min_y: 0.0, max_y: 10.0, min_z: -50.0, max_z: 50.0 })),

            // Set color to grey
            material: materials.add(Color::rgb(0.9, 0.9, 0.9).into()),

            // Use default values for rest
            ..default()
        })
        
        // Add a collider that is the same size as the block
        .insert(Collider::cuboid(100.0, 10.0, 100.0))

        // Add component to find easier
        .insert(Wall)

        // Move into position
        .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, -10.0, 0.0)));
    
    // Spawn wall 1
    commands
        .spawn_bundle(PbrBundle {

            // Add a box that is 100x75x1
            mesh: meshes.add(Mesh::from(shape::Box { min_x: -50.0, max_x: 50.0, min_y: 0.0, max_y: 75.0, min_z: 0.0, max_z: 1.0})),

            // Set color to grey
            material: materials.add(Color::rgb(0.9, 0.9, 0.9).into()),

            // Use default values for rest
            ..default()
        })

        // Add a collider the same size as the box
        .insert(Collider::cuboid(100.0, 75.0, 1.0))

        // Add component to find easier
        .insert(Wall)
        
        // Move into position
        .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, 0.0, 50.0)));
    
    // Spawn wall 2
    commands
        .spawn_bundle(PbrBundle {

            // Add a box that is 100x75x1
            mesh: meshes.add(Mesh::from(shape::Box { min_x: -50.0, max_x: 50.0, min_y: 0.0, max_y: 75.0, min_z: 0.0, max_z: 1.0})),

            // Set color to grey
            material: materials.add(Color::rgb(0.9, 0.9, 0.9).into()),
            
            // Use default values for rest
            ..default()
        })

        // Add a collider the same size as the box
        .insert(Collider::cuboid(100.0, 75.0, 1.0))

        // Add component to find easier
        .insert(Wall)

        // Move into position
        .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, 0.0, -50.0)));

    // Spawn wall 3
    commands
        .spawn_bundle(PbrBundle {
            
            // Add a box that is 1x75x100
            mesh: meshes.add(Mesh::from(shape::Box { min_x: 0.0, max_x: 1.0, min_y: 0.0, max_y: 75.0, min_z: -50.0, max_z: 50.0})),

            // Set color to grey
            material: materials.add(Color::rgb(0.9, 0.9, 0.9).into()),

            // Use default values for rest
            ..default()
        })

        // Add a collider the same size as the box
        .insert(Collider::cuboid(1.0, 75.0, 100.0))

        // Add component to find easier
        .insert(Wall)

        // Move into position
        .insert_bundle(TransformBundle::from(Transform::from_xyz(50.0, 0.0, 0.0)));

    // Spawn wall 4
    commands
        .spawn_bundle(PbrBundle {

            // Add a box that is 1x75x100
            mesh: meshes.add(Mesh::from(shape::Box { min_x: 0.0, max_x: 1.0, min_y: 0.0, max_y: 75.0, min_z: -50.0, max_z: 50.0})),

            // Set color to grey
            material: materials.add(Color::rgb(0.9, 0.9, 0.9).into()),

            // Use default values for rest
            ..default()
        })

        // Add a collider the same size as the box
        .insert(Collider::cuboid(1.0, 75.0, 100.0))

        // Add component to find easier
        .insert(Wall)

        // Move into position
        .insert_bundle(TransformBundle::from(Transform::from_xyz(-50.0, 0.0, 0.0)));
}


// Add light
pub fn spawn_light(
    mut commands: Commands
) {
    // Add bright white light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 1.0
    })
}

// Check for config file and read it in if present
pub fn update_sim_settings(mut commands: Commands) {

    // Get CLI args
    let args: Vec<String> = std::env::args().collect();

    // Initialize path to a default value
    let mut path = "config.txt".to_string();

    // Check if a different path is supplied after a -c flag
    if args.len() >= 3 {
        if args[1] == "-c" {
            path = args[2].clone();
        } else {
            println!("Only valid arg is -c and then the filepath to the config file.")
        }
    }

    // Read in file
    let res = fs::read_to_string(path);

    // If file does not exist or is undreadable return default data
    if res.is_err() {
        println!("Error");
        let lin_vel: Vec3 = Vec3::new(7.0, 2.0, 7.0);
        let ang_vel: Vec3 = Vec3::new(1.0, 2.0, 1.0);
        commands.insert_resource(SimSettings {
            fuse_time: 5.0,
            lin_vel,
            ang_vel,
            friction: 10.0,
            restitution: 0.9,
            initial_height: 5.0,
            fragment_count: 50,
            explosion_vel: 480.0,
            csv_location: "data.csv".to_string(),
        });
    // If file exists then parse file
    } else {
        // Initialize all variables to default values.
        let mut fuse_time: f32 = 5.0;
        let mut friction: f32 = 10.0;
        let mut restitution: f32 = 0.9;
        let mut initial_height: f32 = 5.0;
        let mut fragment_count: u32 = 100;
        let mut explosion_vel: f32 = 480.0;
        let mut csv_location: String = "data.csv".to_string();
        let mut lin_vel: Vec3 = Vec3::new(7.0, 2.0, 7.0);
        let mut ang_vel: Vec3 = Vec3::new(1.0, 2.0, 1.0);

        // Get valid file
        let valid_file = res.expect("Could not open.");

        // Parse file by line
        let lines = valid_file.lines();

        // If no lines then use default data
        if lines.clone().count() == 0 {
            commands.insert_resource(SimSettings {
                fuse_time,
                lin_vel,
                ang_vel,
                friction,
                restitution,
                initial_height,
                fragment_count,
                explosion_vel,
                csv_location: csv_location.clone()
            });
        }

        // Iterate over each line
        for line in lines {

            // Configurable delimeter. Must be set below not in file.
            let delimeter = ":";
            
            // Split the line on the delimeter
            let config_vals = line.split(delimeter).collect::<Vec<&str>>();
            
            // Check if there are at least 2 pieces
            if config_vals.len() >= 2 {

                // Match the value of the key
                match config_vals[0].to_lowercase().as_str() {
                    // Set fuse_time
                    "fuse_time" => {
                        let parsed: Result<f32, _> = config_vals[1].trim().parse();
                        if parsed.is_ok() {
                            fuse_time = parsed.expect("Not a f32");
                        }
                    },

                    // Set friction
                    "friction" => {
                        let parsed: Result<f32, _> = config_vals[1].trim().parse();
                        if parsed.is_ok() {
                            friction = parsed.expect("Not a f32");
                        }
                    },

                    // Set restitution
                    "restitution" => {
                        let parsed: Result<f32, _> = config_vals[1].trim().parse();
                        if parsed.is_ok() {
                            restitution = parsed.expect("Not a f32");
                        }
                    },

                    // Set initial height
                    "initial_height" => {
                        let parsed: Result<f32, _> = config_vals[1].trim().parse();
                        if parsed.is_ok() {
                            initial_height = parsed.expect("Not a f32");
                        }
                    },

                    // Set fragment_count
                    "fragment_count" => {
                        let parsed: Result<u32, _> = config_vals[1].trim().parse();
                        if parsed.is_ok() {
                            fragment_count = parsed.expect("Not a u32");
                        }
                    },

                    // Set explosion velocity
                    "explosion_vel" => {
                        let parsed: Result<f32, _> = config_vals[1].trim().parse();
                        if parsed.is_ok() {
                            explosion_vel = parsed.expect("Not a f32");
                        }
                    },

                    // Set output file location
                    "csv_location" => {
                        csv_location = config_vals[1].trim().to_string();
                    },

                    // Set initial linear velocity
                    "lin_vel" => {
                        // Parse out < and >
                        let cleaned_string = config_vals[1].trim().replace("<", "").replace(">", "");

                        // Split the remainder on , to get individual values
                        let cleaned_arr = cleaned_string.split(",");

                        // Collect values into a vector
                        let vec: Vec<&str> = cleaned_arr.collect();

                        // Initialize default x, y, z
                        let mut x: f32 = 7.0;
                        let mut y: f32 = 2.0;
                        let mut z: f32 = 7.0;
                        
                        // Ensure that it is a 3-dimensional vector
                        if vec.len() == 3 {
                            
                            // Parse x
                            let parsed_x: Result<f32, _> = vec[0].trim().parse();
                            if parsed_x.is_ok() {
                                x = parsed_x.expect("Not a f32");
                            }

                            // Parse y
                            let parsed_y: Result<f32, _> = vec[1].trim().parse();
                            if parsed_y.is_ok() {
                                y = parsed_y.expect("Not a f32");
                            }

                            // Parse z
                            let parsed_z: Result<f32, _> = vec[2].trim().parse();
                            if parsed_z.is_ok() {
                                z = parsed_z.expect("Not a f32");
                            }
                            // Set linear velocity
                            lin_vel = Vec3::new(x, y, z);
                        } else {
                            // Alert user to a invalid configuration
                            println!("Invalid vector for lin_vel.")
                        }
                    },
                    "ang_vel" => {
                        // Parse out < and >
                        let cleaned_string = config_vals[1].trim().replace("<", "").replace(">", "");

                        // Split the remainder on , to get individual values
                        let cleaned_arr = cleaned_string.split(",");

                        // Collect split values into an vector
                        let vec: Vec<&str> = cleaned_arr.collect();

                        // Initialize default x, y, z
                        let mut x: f32 = 7.0;
                        let mut y: f32 = 2.0;
                        let mut z: f32 = 7.0;

                        // Ensure that it is a 3-dimensional vector
                        if vec.len() == 3 {

                            // Parse x
                            let parsed_x: Result<f32, _> = vec[0].trim().parse();
                            if parsed_x.is_ok() {
                                x = parsed_x.expect("Not a f32");
                            }

                            // Parse y
                            let parsed_y: Result<f32, _> = vec[1].trim().parse();
                            if parsed_y.is_ok() {
                                y = parsed_y.expect("Not a f32");
                            }

                            // Parse z
                            let parsed_z: Result<f32, _> = vec[2].trim().parse();
                            if parsed_z.is_ok() {
                                z = parsed_z.expect("Not a f32");
                            }

                            // Set angular velocity
                            ang_vel = Vec3::new(x, y, z);
                        } else {
                            // Alert user to an invalid configuration
                            println!("Invalid vector for ang_vel.")
                        }
                    },

                    // Ignore all other keys
                    _ => {}
                }
                // Insert all updated or default data
                commands.insert_resource(SimSettings {
                    fuse_time,
                    lin_vel,
                    ang_vel,
                    friction,
                    restitution,
                    initial_height,
                    fragment_count,
                    explosion_vel,
                    csv_location: csv_location.clone()
                });
            // If there are more or less than just a key and value
            } else {

                // Insert default data
                commands.insert_resource(SimSettings {
                    fuse_time,
                    lin_vel,
                    ang_vel,
                    friction,
                    restitution,
                    initial_height,
                    fragment_count,
                    explosion_vel,
                    csv_location: csv_location.clone()
                });
            }
        }
    }
}