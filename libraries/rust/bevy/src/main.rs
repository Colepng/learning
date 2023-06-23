use std::f32::consts::PI;

use bevy::ecs::bundle::BundleInfo;
use bevy::prelude::*;
use bevy::app::CoreSchedule::FixedUpdate;
use bevy::sprite::collide_aabb::{collide, self};
use bevy::utils::tracing::Instrument;
use bevy::sprite::collide_aabb::Collision;

const G: f64 = 9.8;
const GROUND: f32 = -300.;
const WALL: f32 = 550.;
const PIXEL_SIZE: Vec2 = Vec2 {x: 6.25, y: 6.25};
const DELTA_TIME: f32 = 1./120.;
fn main() {

    App::new()
        .add_plugins(DefaultPlugins.set(
            // This sets image filtering to nearest
            // This is done to prevent textures with low resolution (e.g. pixel art) from being blurred
            // by linear filtering.
            ImagePlugin::default_nearest(),
        ))
        .insert_resource(FixedTime::new_from_secs(DELTA_TIME)).add_system(physics.in_schedule(FixedUpdate))
        .add_startup_system(setup)
        .add_system(keyboard_input_system)
        .add_system(bevy::window::close_on_esc)
        //.add_system(physics_2.in_schedule(FixedUpdate))
        .run();
}

#[derive(Component)]
enum Direction {
    Left,
    Right,
}

#[derive(Component)]
struct Physics {
    acceleration: Vec2,
    velocity: Vec2,
    force: Vec2,
    // in kg
    mass: f64,
}

#[derive(Resource)]
struct Id(usize);
//
// #[derive(Bundle)]
// struct Pixel {
//     physics: Physics,
//     transfrom: Transform,
// }

// impl Pixel {
//     fn new(xyz: Vec3, mass: f64) -> Self {
//         Pixel {
//             physics: Physics {
//                 acceleration: V,
//                 velocity: 0.,
//                 mass: mass,
//             },
//             transfrom: Transform::from_translation(xyz),
//         }
//     }
// }
//
static mut id: usize = 0;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    // commands.spawn((
    //     SpriteBundle {
    //         texture: asset_server.load("bevy_pixel_dark.png"),
    //         transform: Transform::from_xyz(0., 300., 0.),
    //         ..default()
    //     },
    //     Direction::Right,
    //     Physics { acceleration: G, velocity: 0., mass: 10. }
    // ));
    //commands.spawn()
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.94, 0.97, 1.0),
                custom_size: Some(PIXEL_SIZE),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(
                rand::random::<f32>(),
                rand::random::<f32>(),
                0.,
            )),
            ..default()
        },
        Physics {
            acceleration: Vec2::new(0., G as f32),
            velocity: Vec2::new(0., 0.),
            mass: 5.,
            force: Vec2::new(0., 0.,),
        },
        // unsafe {
        //     Id(id)
        // }
    ));
    // unsafe {
    //     id += 1;
    // }
    commands.spawn(
 SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.1, 0.9, 1.0),
                custom_size: Some(Vec2::new(1000., 100.)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(
                0.,
                GROUND,
                0.,
            )),
            ..default()
        });
    // commands.spawn((
    //         Pixel::new(Vec3::new(0., 100., 0.), 5.),
    //         Sprite {
    //             color: Color::rgb(0.25, 0.25, 0.25),
    //             custom_size: Some(Vec2::new(6.25, 6.25)),
    //         }
    // ));
}

fn keyboard_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut sprite_position: Query<(&mut Transform, &mut Physics)>,
    mut commands: Commands,
    collider_query: Query<(&Transform, &Sprite), Without<Physics>>,
) {
    if keyboard_input.just_pressed(KeyCode::E) {
        info!("'A' currently pressed");
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                color: Color::rgb(0.94, 0.97, 1.0),
                    custom_size: Some(PIXEL_SIZE),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(
                    0.,
                    80.,
                    0.,
                )),
                ..default()
            },
        Physics {
            acceleration: Vec2::new(0., G as f32),
            velocity: Vec2::new(1., 0.),
            mass: 5.,
            force: Vec2::new(0., 0.,),
        },
        // unsafe {
        //     Id(id)
        // }
        ));
        // unsafe {
        //     id += 1;
        // }
    }

    if keyboard_input.pressed(KeyCode::A) {
        for (_, mut physics) in &mut sprite_position {
            physics.acceleration.x -= -10.;
        }
    }

    if keyboard_input.pressed(KeyCode::D) {
        for (_, mut physics) in &mut sprite_position {
            physics.acceleration.x -= 10.;
        }
    }

    if keyboard_input.just_pressed(KeyCode::Space) {
        for (transform, mut physics) in &mut sprite_position {
            for (floor, sprite) in &collider_query {
                if (transform.translation.y - PIXEL_SIZE.y/2. + 0.001).round() == (GROUND + sprite.custom_size.unwrap().y/2.).round() {
                    physics.velocity.y -= 10.;
                }
            }
        }
    }

    if keyboard_input.just_released(KeyCode::A) {
        info!("'A' just released");
    }
}

fn physics(mut sprite_position: Query<(&mut Transform, &mut Physics)>, collider_query: Query<(&Transform, &Sprite), Without<Physics>>) {
    // let mut temp2 = sprite_position.iter_combinations_mut();
    // while let Some([(mut b_transfrom, mut b_physics), (mut a_transfrom, mut a_physics)]) = temp2.fetch_next() {
    //     match collide(b_transfrom.translation, PIXEL_SIZE, a_transfrom.translation, PIXEL_SIZE) {
    //         Some(_) => {
    //         b_physics.velocity.y = 0.;
    //         let move_by_y = b_transfrom.translation.y - a_transfrom.translation.y;
    //         b_transfrom.translation.y -= (PIXEL_SIZE.y - move_by_y)/2.;
    //         a_transfrom.translation.y += (PIXEL_SIZE.y - move_by_y)/2.;
    //         }
    //         None => {}
    //     }
    // }

    for (mut transform, mut physics) in &mut sprite_position {

        physics.force.y;

        physics.velocity.y += physics.acceleration.y * DELTA_TIME;
        physics.velocity.x += physics.acceleration.x * DELTA_TIME;
        //info!("{}", physics.velocity);

        transform.translation.y -= physics.velocity.y;
        transform.translation.x -= physics.velocity.x;

        // if transform.translation.y < GROUND {
        //     physics.velocity.y = 0.;
        //     transform.translation.y = GROUND;
        // }

        if physics.acceleration.x > 10. {
            physics.acceleration.x = 10.;
        } else if physics.acceleration.x < -10. {
            physics.acceleration.x = -10.;
        }
        if transform.translation.x > WALL {
            transform.translation.x = WALL;
            physics.velocity.x *= -1.;
            physics.acceleration.x *= -1.;
        } else if transform.translation.x < -WALL {
            transform.translation.x = -WALL;
            physics.velocity.x *= -1.;
            physics.acceleration.x *= -1.;
        }

        // for (pixel) in &pixel_query {
        //     // if temp.0 != entity.0 {
        //         match collide(transform.translation, Vec2::new(6.25, 6.25), pixel.translation, Vec2::new(6.25, 6.25)) {
        //             Some(Collision::Top) => {
        //                 transform.translation.y = pixel.translation.y + 6.25/2.;
        //             }
        //             _ => {}
        //         }
        //     // }
        // }

        for (floor, sprite) in &collider_query {
            match collide(transform.translation, PIXEL_SIZE, floor.translation, sprite.custom_size.unwrap()) {
                Some(Collision::Top) | Some(Collision::Inside)=> {
                    info!("{}", physics.acceleration);
                    // bouncing
                    //physics.velocity.y *= -0.5;

                    // no bouncing
                    physics.velocity.y = 0.;
                    transform.translation.y = floor.translation.y + sprite.custom_size.unwrap().y/2. + PIXEL_SIZE.y/2.; 
                    //physics.acceleration.x *= 0.95;
                    //physics.velocity.x *= 0.95;
                }
                _ => {}
            }
            physics.acceleration.x *= 0.9;
            physics.velocity.x *= 0.9;
        }
        if transform.translation.y == GROUND {
            //physics.velocity.x *= 0.1;
        }
    }

        //physics_2(sprite_position);
}

fn physics_2(mut temp: Query<(&mut Transform, &mut Physics), With<Physics>>) {
    let mut temp2 = temp.iter_combinations_mut();
    while let Some([(mut b_transfrom, mut b_physics), (mut a_transfrom, mut a_physics)]) = temp2.fetch_next() {
        match collide(b_transfrom.translation, PIXEL_SIZE, a_transfrom.translation, PIXEL_SIZE) {
            Some(_) => {
            b_physics.velocity.y = 0.;
            let move_by_y = b_transfrom.translation.y - a_transfrom.translation.y;
            b_transfrom.translation.y -= (PIXEL_SIZE.y - move_by_y)/2.;
            a_transfrom.translation.y += (PIXEL_SIZE.y - move_by_y)/2.;
            }
            None => {}
        }
    }
}
