use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, ios_button_system)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    // Загружаем шрифт из папки assets/fonts/
    let font = asset_server.load("fonts/arial_bolditalicmt.ttf");

    commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            
            align_items: AlignItems::Center,
            ..default()
        },
        ..default()
    })
    .with_children(|parent| {
        parent.spawn(ButtonBundle {
            style: Style {
                width: Val::Px(240.0),
                height: Val::Px(56.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
               
                ..default()
            },
            border_radius: BorderRadius::all(Val::Px(16.0)),
            background_color: BackgroundColor(Color::srgb(0.0, 0.48, 1.0)),
            ..default()
        })
        .with_children(|button| {
            button.spawn(TextBundle::from_section(
                "Продолжить",
                TextStyle {
                    font: font.clone(), // Применяем загруженный шрифт
                    font_size: 24.0,
                    color: Color::WHITE,
                },
            ));
        });
    });
}

fn ios_button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut Transform),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, mut transform) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = BackgroundColor(Color::srgb(0.0, 0.35, 0.85));
                transform.scale = Vec3::splat(0.95);
            }
            Interaction::Hovered => {
                *color = BackgroundColor(Color::srgb(0.1, 0.55, 1.0));
                transform.scale = Vec3::splat(1.0);
            }
            Interaction::None => {
                *color = BackgroundColor(Color::srgb(0.0, 0.48, 1.0));
                transform.scale = Vec3::splat(1.0);
            }
        }
    }
}