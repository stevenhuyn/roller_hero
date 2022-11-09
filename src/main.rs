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
#[derive(Component)]

struct ScoreText;

#[derive(Default)]
struct Score(usize);

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
                .with_run_criteria(FixedTimestep::step(0.5))
                .with_system(diamond_spawner),
        )
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(sprite_movement)
        .add_system(diamond_mover)
        .add_system(diamond_deleter)
        .add_system(update_score)
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

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn_bundle(Camera2dBundle::default());
    commands.spawn_bundle(MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
        transform: Transform::from_translation(Vec3::new(-400., 0., 1.))
            .with_scale(Vec3::new(10., 1000., 1.)),
        material: materials.add(ColorMaterial::from(Color::GOLD)),
        ..default()
    });

    // Score stuff
    commands.insert_resource(Score(0));
    let font = asset_server.load("Roboto-Regular.ttf");
    let text_style = TextStyle {
        font,
        font_size: 60.0,
        color: Color::WHITE,
    };
    let text_alignment = TextAlignment::CENTER_RIGHT;
    commands
        .spawn_bundle(Text2dBundle {
            text: Text::from_section("0", text_style).with_alignment(text_alignment),
            transform: Transform::from_translation(Vec3::new(400., 400., 0.)),
            ..default()
        })
        .insert(ScoreText);
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

const DIAMOND_XVEL: f32 = -400.;

fn update_score(score: Res<Score>, mut query: Query<&mut Text, With<ScoreText>>) {
    for mut text in query.iter_mut() {
        text.sections[0].value = format!("{}", score.0);
    }
}

fn diamond_mover(
    mut commands: Commands,
    mut query: Query<(Entity, &Diamond, &mut Transform)>,
    time: Res<Time>,
) {
    for (entity, _, mut transform) in query.iter_mut() {
        transform.translation.x += DIAMOND_XVEL * time.delta_seconds();
        if transform.translation.x <= -551. {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn diamond_deleter(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(Entity, &Diamond, &Transform)>,
    mut score: ResMut<Score>,
) {
    for (entity, _, transform) in query.iter_mut() {
        if transform.translation.x - 50. <= -410.
            && transform.translation.x + 50. >= -390.
            && keyboard_input.pressed(KeyCode::Space)
        {
            commands.entity(entity).despawn_recursive();
            score.0 += 100;
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
                550. + (DIAMOND_XVEL * time.delta_seconds()),
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
