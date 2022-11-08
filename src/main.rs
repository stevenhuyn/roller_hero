use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    sprite::MaterialMesh2dBundle,
    time::{FixedTimestep, FixedTimesteps},
    window::PresentMode,
};

const LABEL: &str = "my_fixed_timestep";

#[derive(Component)]
struct Garbage;

fn main() {
    App::new()
        // .add_plugin(LogDiagnosticsPlugin::default())
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .insert_resource(WindowDescriptor {
            width: 1000.0,
            height: 1000.0,
            canvas: Some("#gamecanvas".to_string()),
            present_mode: PresentMode::AutoVsync,
            ..default()
        })
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.5).with_label(LABEL))
                .with_system(fixed_hello),
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(2.))
                .with_system(diamond_spawner),
        )
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(sprite_movement)
        .add_system(diamond_mover)
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

fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
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

#[derive(Component)]
struct Diamond;

const DIAMOND_XVEL: f32 = -200.;

fn diamond_mover(
    mut commands: Commands,
    mut query: Query<(Entity, &Diamond, &mut Transform)>,
    time: Res<Time>,
) {
    for (entity, _, mut transform) in query.iter_mut() {
        transform.translation.x += DIAMOND_XVEL * time.delta_seconds();
        if transform.translation.x <= -300. {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn diamond_spawner(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>,
) {
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
            transform: Transform::from_translation(Vec3::new(
                650. + (DIAMOND_XVEL * time.delta_seconds()),
                0.,
                0.,
            ))
            .with_scale(Vec3::splat(100.)),
            material: materials.add(ColorMaterial::from(Color::PURPLE)),
            ..default()
        })
        .insert(Diamond);
}

fn cleanup_system<T: Component>(mut commands: Commands, q: Query<Entity, With<T>>) {
    for e in q.iter() {
        commands.entity(e).despawn_recursive();
    }
}
