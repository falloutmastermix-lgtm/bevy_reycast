use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<()>::default())
        // .add_plugins(RapierDebugRenderPlugin::default()) // Раскомментируйте, если хотите видеть границы коллайдеров
        .add_systems(Startup, setup_scene)
        .add_systems(Update, update_tracking_system) // Указываем правильную систему обновления
        .run();
}

#[derive(Component)]
struct Target;

#[derive(Component)]
struct Tracker;

fn setup_scene(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    // 1. Создаем цель (Target)
    commands.spawn((
        SpriteBundle {
            sprite: Sprite { 
                color: Color::srgb(0.0, 0.0, 1.0), 
                custom_size: Some(Vec2::new(30.0, 30.0)), 
                ..default() 
            },
            transform: Transform::from_xyz(200.0, 0.0, 0.0),
            ..default()
        },
        Target,
        Collider::cuboid(15.0, 15.0), // ОБЯЗАТЕЛЬНО: коллайдер, чтобы луч попадал
    ));

    // 2. Создаем преследователя (Tracker)
    commands.spawn((
        SpriteBundle {
            sprite: Sprite { 
                color: Color::srgb(1.0, 1.0, 0.0), 
                custom_size: Some(Vec2::new(30.0, 30.0)), 
                ..default() 
            },
            transform: Transform::from_xyz(-200.0, 0.0, 0.0),
            ..default()
        },
        Tracker,
        Collider::cuboid(15.0, 15.0), // ОБЯЗАТЕЛЬНО: коллайдер
    ));
}

fn update_tracking_system(
    rapier_context: Res<RapierContext>,
    mut gizmos: Gizmos,
    time: Res<Time>,
    mut query_target: Query<&mut Transform, (With<Target>, Without<Tracker>)>,
    mut query_tracker: Query<&mut Transform, With<Tracker>>,
) {
    let Ok(mut target_transform) = query_target.get_single_mut() else { return; };
    let Ok(mut tracker_transform) = query_tracker.get_single_mut() else { return; };

    // 1. Движение цели по вертикали
    let time_seconds = time.elapsed_seconds();
    target_transform.translation.y = (time_seconds.sin()) * 100.0;

    // 2. Вычисление направления
    let target_pos = target_transform.translation.truncate();
    let tracker_pos = tracker_transform.translation.truncate();
    let direction = (target_pos - tracker_pos).normalize_or_zero();
    let max_toi = 500.0;

    // 3. Raycast: проверяем попадание луча
    let hit = rapier_context.cast_ray(
        tracker_pos,
        direction,
        max_toi,
        true, // solid
        QueryFilter::default().exclude_sensors(), // игнорируем сенсоры
    );

    // 4. Отрисовка луча (зеленый если попали, красный если мимо)
    let color = if hit.is_some() { Color::srgb(0.0, 1.0, 0.0) } else { Color::srgb(1.0, 0.0, 0.0) };
    gizmos.line_2d(tracker_pos, tracker_pos + direction * max_toi, color);
}