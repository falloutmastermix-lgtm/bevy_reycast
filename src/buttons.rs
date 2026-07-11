
use bevy::prelude::*;

pub fn spawn_ios_button(
    parent: &mut ChildBuilder, 
    text: &str, 
    width: f32, 
    font: Handle<Font>
) {
    parent.spawn(ButtonBundle {
        style: Style {
            width: Val::Px(width),
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
            text,
            TextStyle {
                font,
                font_size: 24.0,
                color: Color::WHITE,
            },
        ));
    });
}