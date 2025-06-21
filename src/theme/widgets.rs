use std::borrow::Cow;

use bevy::{
    ecs::{spawn::SpawnWith, system::IntoObserverSystem},
    prelude::*,
    text::LineHeight,
};

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
        Name::new("Card"),
        Node {
            width: Val::Vw(50.0),
            height: Val::Vh(80.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::srgb(0.0, 0.5, 0.5)),
    )
}

pub fn header(content: impl Into<String>) -> impl Bundle {
    (
        Name::new("Header"),
        Text::new(content),
        TextFont {
            font_size: 48.0,
            line_height: LineHeight::RelativeToFont(1.6),
            ..default()
        },
        TextColor::WHITE,
    )
}

pub fn button<E, B, M, I>(label: impl Into<String>, action: I) -> impl Bundle
where
    E: Event,
    B: Bundle,
    I: IntoObserverSystem<E, B, M>,
{
    let label = label.into();
    let action = IntoObserverSystem::into_system(action);

    (
        Name::new("Button"),
        Node::default(),
        Children::spawn(SpawnWith(|parent: &mut ChildSpawner| {
            parent
                .spawn((
                    Name::new("Button inner"),
                    Button::default(),
                    BackgroundColor(Color::srgb(0.5, 0.5, 0.0)),
                    children![(
                        Name::new("Button text"),
                        Text::new(label),
                        TextFont::from_font_size(40.0),
                        TextColor::WHITE,
                        Pickable::IGNORE,
                    )],
                ))
                .observe(action);
        })),
    )
}

pub fn list() -> impl Bundle {
    Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),

        display: Display::Flex,
        flex_direction: FlexDirection::Column,
        justify_content: JustifyContent::SpaceEvenly,
        align_items: AlignItems::Center,

        ..default()
    }
}
