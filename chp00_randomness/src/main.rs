use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    window::WindowResolution,
};
use noise::{self, NoiseFn};
use rand::Rng;

const WIDTH: f32 = 1280.0;
const HEIGHT: f32 = 640.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set({
            WindowPlugin {
                primary_window: Some(Window {
                    resolution: WindowResolution::new(WIDTH, HEIGHT),
                    ..default()
                }),
                ..default()
            }
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, (walk, change_walk_mode, close_on_esc))
        .run();
}
#[derive(Component)]
enum WalkType {
    Random,
    Noise,
}

#[derive(Component)]
struct Walker {
    x: f64,
    y: f64,
    tx: f64,
    ty: f64,
    walk_type: WalkType,
}

impl Walker {
    fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            tx: 100.0,
            ty: 0.0,
            walk_type: WalkType::Random,
        }
    }
}

#[derive(Component)]
struct ModeText;

#[derive(Component)]
struct DisplayText;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle { ..default() });

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Circle::new(8.0))),
            material: materials.add(Color::srgb(0.0, 1.0, 0.5)),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        Walker::new(),
    ));

    let roboto_handle = asset_server.load("fonts/Roboto-Regular.ttf");

    commands.spawn((
        TextBundle::from_section(
            "Press space to change the bug's movement mode :)",
            TextStyle {
                font: roboto_handle.clone(),
                font_size: 20.0,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            left: Val::Px(10.0),
            top: Val::Px(10.0),
            ..default()
        }),
        DisplayText,
    ));

    commands.spawn((
        TextBundle::from_section(
            "Random",
            TextStyle {
                font: roboto_handle.clone(),
                font_size: 20.0,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        }),
        ModeText,
    ));
}

fn walk(mut query: Query<(&mut Walker, &mut Transform)>) {
    let (mut walker, mut transform) = query.single_mut();

    match walker.walk_type {
        WalkType::Random => {
            random_walk(&mut walker);
            transform.translation = (walker.x as f32, walker.y as f32, 0.0).into();
        }
        WalkType::Noise => {
            noise_walk(&mut walker);
            transform.translation = (walker.x as f32, walker.y as f32, 0.0).into();
        }
    }
}

// mutates walker state
fn random_walk(walker: &mut Walker) {
    let step = rand::thread_rng().gen_range(0.0..1.1);

    // long ugly if else chain to allow small chance for periodic larger "jumps" in walkers movement
    if step < 0.25 {
        walker.x += 2.0
    } else if step < 0.5 {
        walker.x -= 2.0
    } else if step < 0.75 {
        walker.y += 2.0
    } else if step < 1.0 {
        walker.y -= 2.0
    } else if step < 1.025 {
        walker.x += 10.0
    } else if step < 1.05 {
        walker.x -= 10.0
    } else if step < 1.075 {
        walker.y += 10.0
    } else {
        walker.y -= 10.0
    }
}

fn noise_walk(walker: &mut Walker) {
    walker.tx += 0.005;
    walker.ty += 0.005;
    let noise = noise::Perlin::new(1);

    // map output to larger range to have quicker movement in noise mode
    walker.x += map(noise.get([walker.tx]), -1.0, 1.0, -5.0, 5.0);
    walker.y += map(noise.get([walker.ty]), -1.0, 1.0, -5.0, 5.0);
}

/// Primitive function to map floats from one range to floats in another
fn map(n: f64, input_start: f64, input_end: f64, output_start: f64, output_end: f64) -> f64 {
    ((output_end - output_start) / (input_end - input_start)) * (n - input_start) + output_start
}

fn change_walk_mode(
    input: Res<ButtonInput<KeyCode>>,
    mut query_walker: Query<&mut Walker>,
    mut query_text: Query<&mut Text, With<ModeText>>,
) {
    let mut walker = query_walker.single_mut();
    let mut text = query_text.single_mut();
    if input.just_pressed(KeyCode::Space) {
        match walker.walk_type {
            WalkType::Random => {
                walker.walk_type = WalkType::Noise;
                text.sections[0].value = String::from("Noise");
            }
            WalkType::Noise => {
                walker.walk_type = WalkType::Random;
                text.sections[0].value = String::from("Random");
            }
        }
    }
}

fn close_on_esc(
    mut commands: Commands,
    focused_windows: Query<(Entity, &Window)>,
    input: Res<ButtonInput<KeyCode>>,
) {
    for (window_ent, _focus) in &focused_windows {
        if input.just_pressed(KeyCode::Escape) {
            commands.entity(window_ent).despawn();
        }
    }
}
