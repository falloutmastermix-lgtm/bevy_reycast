use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(PhysicsDebugPlugin::default())
        .insert_resource(Gravity(Vec2::new(0.0, -9.81)))
        .add_systems(Startup, setup)
        .add_systems(Update, drive_system)
        .run();
}

#[derive(Component)]
struct CarMotor;

fn setup(mut commands: Commands) {
    // ВАЖНО: Добавляем камеру, иначе ничего не увидите
    commands.spawn(Camera2dBundle::default());

    // 1. Пол
    commands.spawn((
        RigidBody::Static,
        Collider::rectangle(2000.0, 50.0), // Исправил cuboid на rectangle по предупреждению
        TransformBundle::from(Transform::from_xyz(0.0, -250.0, 0.0)),
    ));

    // 2. Составное тело (кузов + 2 колеса)
    commands.spawn((
        RigidBody::Dynamic,
        Collider::compound(vec![
            (Vec2::ZERO, 0.0, Collider::rectangle(100.0, 40.0)),
            (Vec2::new(-50.0, -40.0), 0.0, Collider::circle(15.0)),
            (Vec2::new(50.0, -40.0), 0.0, Collider::circle(15.0)),
        ]),
        TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)),
    ));
}

fn drive_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut wheels: Query<&mut ExternalTorque, With<CarMotor>>,
) {
    for mut torque in wheels.iter_mut() {
        if keyboard.pressed(KeyCode::KeyD) {
            torque.apply_torque(-100.0);
        } else if keyboard.pressed(KeyCode::KeyA) {
            torque.apply_torque(100.0);
        } else {
            // Ленивое торможение
            torque.apply_torque(0.0);
        }
    }
}