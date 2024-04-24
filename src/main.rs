//! Shows how to render simple primitive shapes with a single color.

mod ui;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle, window::PrimaryWindow};
use bevy::input::keyboard::KeyboardInput;
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use rand::Rng;
use crate::ui::spawn_top_bar;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Startup, spawn_top_bar)
        .add_systems(Update, (spawn_enemies, update_enemies))
        .add_systems(Update, spawn_tower)
        .add_systems(Update, camera_controls)
        .run();
}

#[derive(Component)]
struct Enemy {}

#[derive(Component)]
struct Tower {}

fn spawn_enemies(
    time: Res<Time>,
    mut timer: ResMut<SpawnTimer>,
    mut commands: Commands,
    asset_set: Res<AssetSet>,
) {
    timer.0.tick(time.delta()).just_finished().then(|| {
        let enemy_material = asset_set.enemy_material.clone();
        let enemy_mesh = asset_set.enemy_mesh.clone();
        let mut rng = rand::thread_rng();
        let e = commands
            .spawn(MaterialMesh2dBundle {
                material: enemy_material,
                mesh: enemy_mesh.into(),
                transform: Transform::from_translation(Vec3::new(
                    500.0 + rng.gen_range(-10.0..10.0),
                    200.0 + rng.gen_range(-10.0..10.0),
                    0.0,
                )),
                ..Default::default()
            })
            .id();

        commands.entity(e).insert(Enemy {});
    });
}

#[derive(Resource)]
struct AssetSet {
    enemy_material: Handle<ColorMaterial>,
    enemy_mesh: Handle<Mesh>,
    tower_material: Handle<ColorMaterial>,
    tower_mesh: Handle<Mesh>,
}

fn spawn_tower(
    buttons: Res<Input<MouseButton>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    asset_set: Res<AssetSet>,
    mut commands: Commands,
) {
    if !buttons.just_pressed(MouseButton::Left) {
        return;
    }

    let (camera, camera_transform) = q_camera.single();
    // Games typically only have one window (the primary window)
    if let Some(position) = q_windows.single().cursor_position() {
        let position = camera
            .viewport_to_world_2d(camera_transform, position)
            .unwrap();
        println! {"Creating tower at: {:?}", position}
        commands
            .spawn(MaterialMesh2dBundle {
                material: asset_set.tower_material.clone(),
                mesh: asset_set.tower_mesh.clone().into(),
                transform: Transform::from_translation(Vec3::new(position.x, position.y, 10.0)),
                ..Default::default()
            })
            .insert(Tower {});
    } else {
        println!("Cursor is not in the game window.");
    }
}

fn update_enemies(
    time: Res<Time>,
    mut commands: Commands,
    mut enemies: Query<(Entity, &mut Transform), With<Enemy>>,
) {
    for (e, mut t) in enemies.iter_mut() {
        t.translation.x -= 50.0 * time.delta_seconds();
        t.translation.y = (t.translation.x * 0.01).sin() * 200.0;
        if t.translation.x < 0.0 {
            commands.entity(e).despawn();
        }
    }
}

#[derive(Resource)]
struct SpawnTimer(Timer);

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.insert_resource(AssetSet {
        enemy_mesh: meshes.add(shape::Circle::new(5.).into()).into(),
        enemy_material: materials.add(ColorMaterial::from(Color::PURPLE)),
        tower_material: materials.add(ColorMaterial::from(Color::LIME_GREEN)),
        tower_mesh: meshes
            .add(shape::Quad::new(Vec2::new(20., 20.)).into())
            .into(),
    });
    commands.insert_resource(SpawnTimer(Timer::from_seconds(1.0, TimerMode::Repeating)));

    let mut camera_bundle = Camera2dBundle {
        transform: Transform::from_translation(Vec3::new(100.0, 100.0, 10.0)),
        ..default()
    };
    // camera_bundle.projection.scale = 0.8;
    commands.spawn(camera_bundle);
}

fn camera_controls(
    mut scroll_evr: EventReader<MouseWheel>,
    keys: Res<Input<KeyCode>>,
    mut camera: Query<(&mut OrthographicProjection, &mut Transform)>,
) {
    let (mut projection, mut transform) = camera.single_mut();

    if keys.pressed(KeyCode::Left) {
        transform.translation.x -= 10.0 * projection.scale
    }
    if keys.pressed(KeyCode::Right) {
        transform.translation.x += 10.0 * projection.scale
    }
    if keys.pressed(KeyCode::Up) {
        transform.translation.y += 10.0 * projection.scale
    }
    if keys.pressed(KeyCode::Down) {
        transform.translation.y -= 10.0 * projection.scale
    }

    for ev in scroll_evr.read() {
        match ev.unit {
            MouseScrollUnit::Line => {
                projection.scale *= 1.0 + ev.y * 0.0001;
            }
            MouseScrollUnit::Pixel => {
                projection.scale *= 1.0 + ev.y * 0.001;
            }
        }
    }
}