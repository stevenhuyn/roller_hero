use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    sprite::MaterialMesh2dBundle,
    time::{FixedTimestep, FixedTimesteps},
    window::PresentMode,
};

const LABEL: &str = "my_fixed_timestep";

fn main() {
    App::new()
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .insert_resource(WindowDescriptor {
            width: 400.0,
            height: 400.0,
            canvas: Some("#gamecanvas".to_string()),
            present_mode: PresentMode::AutoVsync,
            ..default()
        })
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.5).with_label(LABEL))
                .with_system(fixed_hello),
        )
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(sprite_movement)
        .run();
}

fn fixed_hello(fixed_timesteps: Res<FixedTimesteps>) {
    println!(
        "hello world: {}",
        fixed_timesteps.get(LABEL).unwrap().overstep_percentage()
    );
}

#[derive(Component)]
enum Direction {
    Up,
    Down,
}

#[derive(Component)]
struct Note {}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_bundle(Camera2dBundle::default());
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("dog.png"),
            transform: Transform::from_xyz(100., 0., 0.),
            ..default()
        })
        .insert(Direction::Up);
    commands.spawn_bundle(MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
        transform: Transform::default().with_scale(Vec3::splat(128.)),
        material: materials.add(ColorMaterial::from(Color::PURPLE)),
        ..default()
    });
}

fn sprite_movement(time: Res<Time>, mut sprite_position: Query<(&mut Direction, &mut Transform)>) {
    for (mut logo, mut transform) in &mut sprite_position {
        match *logo {
            Direction::Up => transform.translation.y += 150. * time.delta_seconds(),
            Direction::Down => transform.translation.y -= 150. * time.delta_seconds(),
        }

        if transform.translation.y > 200. {
            *logo = Direction::Down;
        } else if transform.translation.y < -200. {
            *logo = Direction::Up;
        }
    }
}
