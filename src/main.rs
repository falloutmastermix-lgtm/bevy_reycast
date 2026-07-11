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
    commands.spawn(Camera2dBundle::default());

    // 1. Пол
    commands.spawn((
        RigidBody::Static,
        Collider::cuboid(2000.0, 50.0),
        TransformBundle::from(Transform::from_xyz(0.0, -250.0, 0.0)),
    ));

    // 2. Кузов (центр машины)
    let body_entity = commands.spawn((
        RigidBody::Dynamic,
        // Коллайдер меньше спрайта, чтобы колеса не "взрывали" его
        Collider::rectangle(80.0, 30.0),
        TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)),
    )).id();

    // 3. Колеса
    let wheel_offsets = [Vec2::new(-50.0, -40.0), Vec2::new(50.0, -40.0)];

    for offset in wheel_offsets {
        let wheel = commands.spawn((
            RigidBody::Dynamic,
            Collider::circle(15.0), // Колеса чуть меньше
            ExternalTorque::default(),
            CarMotor,
            TransformBundle::from(Transform::from_xyz(offset.x, offset.y, 0.0)),
        )).id();

        // Шарнир: привязка к кузову
        let joint = RevoluteJoint::new(body_entity, wheel)
            .with_local_anchor_1(offset)
            .with_local_anchor_2(Vec2::ZERO);
        
        commands.entity(wheel).insert(joint);
    }
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