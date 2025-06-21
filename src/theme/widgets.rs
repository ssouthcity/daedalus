use std::borrow::Cow;

use bevy::prelude::*;

pub fn root(name: impl Into<Cow<'static, str>>) -> impl Bundle {
    (
        Name::new(name),
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
    )
}

pub fn background_dimmed() -> impl Bundle {
    BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.9))
}

pub fn card() -> impl Bundle {
    (
        Name::new("card"),
        Node {
            width: Val::Vw(50.0),
            height: Val::Vh(80.0),
            justify_content: JustifyContent::Center,
            ..default()
        },
        BackgroundColor(Color::srgb(0.0, 0.5, 0.5)),
    )
}

pub fn header(content: impl Into<String>) -> impl Bundle {
    (
        Name::new("header"),
        Text::new(content),
        TextFont {
            font_size: 48.0,
            ..default()
        },
        TextColor::WHITE,
    )
}
