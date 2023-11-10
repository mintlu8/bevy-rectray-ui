use bevy_aoui::{*, bundles::*};
use bevy::{prelude::*, sprite::Anchor, text::Text2dBounds};

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, init)
        .add_plugins(AoUIPlugin)
        .run();
}

pub fn style(color: Color) -> TextStyle{
    TextStyle{
        color,
        ..Default::default()
    }
}

pub fn init(mut commands: Commands, assets: Res<AssetServer>) {

    commands.spawn(Camera2dBundle::default());

    let textbox = commands.spawn((AoUISpriteBundle {
        transform: Transform2D::UNIT,
        sprite: Sprite { 
            anchor: Anchor::Center,
            color: Color::DARK_GRAY,
            ..Default::default()
        },
        dimension: Dimension::pixels(Vec2::new(300.0, 700.0)),
        texture: assets.load("square.png"),
        ..Default::default()
    }, Container {
        layout: Layout::Paragraph { 
            direction: FlexDir::LeftToRight, 
            stack: FlexDir::TopToBottom,
            stretch: false,
        },
        margin: Size2::pixels(4.0, 0.0),
    })).id();
    let mut words = Vec::new();
    words.push(commands.spawn(AoUITextBundle {
        text: Text::from_section(
            "Thunderbolt", 
            style(Color::WHITE)
        ),
        transform: Transform2D::UNIT.with_anchor(Anchor::TopLeft),
        ..Default::default()
    }).id());

    words.push(commands.spawn(AoUITextBundle {
        text: Text::from_section(
            "Special Attack", 
            style(Color::WHITE)
        ),
        transform: Transform2D::UNIT.with_anchor(Anchor::TopRight),
        ..Default::default()
    }).id());

    words.push(commands.spawn(LinebreakBundle::default()).id());

    words.push(commands.spawn(AoUISpriteBundle {
        texture: assets.load("electric_type.png"),
        transform: Transform2D::UNIT.with_anchor(Anchor::TopLeft),
        dimension: Dimension::owned(Size2::em(0.8, 0.8)),
        ..Default::default()
    }).id());

    words.push(commands.spawn(AoUITextBundle {
        
        text: Text::from_section(
            "Electric Type", 
            style(Color::WHITE)
        ),
        transform: Transform2D::UNIT.with_anchor(Anchor::TopLeft),
        dimension: Dimension::COPIED.with_em(SetEM::Ems(0.8)),
        ..Default::default()
    }).id());


    words.push(commands.spawn(AoUITextBundle {
        text: Text::from_section(
            "90 bp", 
            style(Color::WHITE)
        ),
        transform: Transform2D::UNIT.with_anchor(Anchor::TopRight),
        dimension: Dimension::COPIED.with_em(SetEM::Ems(0.8)),
        ..Default::default()
    }).id());

    words.push(commands.spawn(LinebreakBundle::default()).id());
    words.push(commands.spawn(LinebreakBundle::ems(Vec2::ONE)).id());


    words.push(commands.spawn(AoUITextBundle {
        text: Text::from_section(
            "The user attacks the target with a strong electric blast. This may also leave the target with paralysis.", 
            style(Color::WHITE)
        ),
        text_bounds: Text2dBounds{
            size: Vec2::new(300.0, 999999.0),
        },
        transform: Transform2D::UNIT.with_anchor(Anchor::TopLeft),
        ..Default::default()
    }).id());

    commands.entity(textbox).push_children(&words);
}
