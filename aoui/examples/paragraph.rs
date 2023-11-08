use bevy_aoui::*;
use bevy::{prelude::*, sprite::Anchor, diagnostic::{LogDiagnosticsPlugin, FrameTimeDiagnosticsPlugin}};


static LOREM_IPSUM: &'static str = 
r#"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Ut condimentum nunc luctus erat tristique facilisis. Nullam nulla dolor, suscipit id feugiat in, vestibulum ut purus. Etiam erat magna, suscipit at felis nec, molestie dignissim tellus. Nullam id eros vitae nisl fermentum accumsan. Donec vitae ante ut dolor accumsan pellentesque eu a sapien. Vivamus dapibus augue lectus, quis hendrerit dui sollicitudin non. Cras enim ante, fermentum eu lectus a, pellentesque efficitur mauris. Integer non sapien metus. Phasellus eget mi condimentum, vestibulum eros et, porta nisl. Cras suscipit egestas tincidunt. Donec id sodales orci.
 
Nunc ac convallis augue. Vivamus vel nisl et eros euismod scelerisque. Sed nec leo eu lacus eleifend pulvinar a quis risus. Duis metus ex, facilisis nec augue nec, aliquam euismod nibh. Integer sit amet tincidunt neque, vel ultrices diam. Donec efficitur malesuada scelerisque. Proin id tincidunt justo.
 
In nec finibus metus, ac hendrerit augue. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer tincidunt, velit consequat luctus aliquam, velit nunc faucibus dui, at convallis enim diam a ligula. Proin molestie eros in suscipit fringilla. Duis eget metus cursus, tristique libero sit amet, malesuada ante. Sed mattis, augue a facilisis luctus, orci velit tristique purus, sit amet finibus neque augue non lectus. Maecenas consectetur urna in odio dictum, in maximus sem tempus. Sed varius est vitae egestas scelerisque. In vitae mattis est. In hac habitasse platea dictumst."#;

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: bevy::window::PresentMode::AutoNoVsync,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_systems(Startup, init)
        .add_systems(Update, controls)
        .add_plugins(AoUIPlugin)
        .run();
}

pub fn init(mut commands: Commands, assets: Res<AssetServer>) {

    commands.spawn(Camera2dBundle::default());

    let textbox = commands.spawn((AoUISpriteBundle {
        transform: Transform2D::DEFAULT,
        sprite: Sprite { 
            anchor: Anchor::Center,
            custom_size: Some(Vec2::new(700.0, 700.0)),
            color: Color::DARK_GRAY,
            ..Default::default()
        },
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

    LOREM_IPSUM.split('\n').for_each(|x| {
        x.split([' ']).filter(|x| !x.is_empty()).for_each(|w| {
            words.push(
                commands.spawn(AoUITextBundle {
                    text: Text::from_section(w, TextStyle{
                        font: assets.load("Noto.ttf"),
                        font_size: 1.0,
                        color: Color::WHITE,
                    }),
                    text_anchor: Anchor::TopLeft,
                    ..Default::default()
                }).id()
            );
        });
        words.push(commands.spawn(LinebreakBundle::new(Size2::em(1.0, 1.0))).id());
    });
    commands.entity(textbox).push_children(&words[0..words.len()-1]);
}

pub fn spin_anc(anc: &Anchor) -> Anchor {
    match anc {
        Anchor::BottomLeft => Anchor::BottomCenter,
        Anchor::BottomCenter => Anchor::BottomRight,
        Anchor::BottomRight => Anchor::CenterLeft,
        Anchor::CenterLeft => Anchor::Center,
        Anchor::Center => Anchor::CenterRight,
        Anchor::CenterRight => Anchor::TopLeft,
        Anchor::TopLeft => Anchor::TopCenter,
        Anchor::TopCenter => Anchor::TopRight,
        Anchor::TopRight => Anchor::BottomLeft,
        Anchor::Custom(_) => unreachable!(),
    }
}

/// Controls: 
/// 1: Spin FlexDir
/// 2: Spin WrapTo
/// 3: Spin Alignment
/// 4: Spin TextAnchor
/// 5: Spin Main Anchor
/// Q, E: Change font size
/// WSAD: Change FlexContainer size
pub fn controls(
    mut flex: Query<&mut Container>, 
    mut text: Query<&mut Anchor, With<Text>>, 
    mut main: Query<&mut Sprite>, 
    mut text_size: Query<&mut Dimension>, 
    keys: Res<Input<KeyCode>>) {
    if keys.just_pressed(KeyCode::Key1) {
        for mut sp in flex.iter_mut() {
            if let Layout::Paragraph { direction, stack, .. } = &mut sp.layout {
                *direction = match direction {
                    FlexDir::LeftToRight => FlexDir::RightToLeft,
                    FlexDir::RightToLeft => {
                        *stack = stack.transpose();
                        FlexDir::BottomToTop
                    },
                    FlexDir::BottomToTop => FlexDir::TopToBottom,
                    FlexDir::TopToBottom => {
                        *stack = stack.transpose();
                        FlexDir::LeftToRight
                    },
                }
            }
        }
    }
    if keys.just_pressed(KeyCode::Key2) {
        for mut sp in flex.iter_mut() {
            if let Layout::Paragraph { stack, .. } = &mut sp.layout {
                *stack = stack.flip()
            }
        }
    }
    if keys.just_pressed(KeyCode::Key4) {
        for mut sp in text.iter_mut() {
            *sp = spin_anc(&sp)
        }
    }
    if keys.just_pressed(KeyCode::Key5) {
        for mut sp in main.iter_mut() {
            sp.anchor = spin_anc(&sp.anchor);
        }
    }
    if keys.just_pressed(KeyCode::Key5) {
        for mut sp in main.iter_mut() {
            sp.anchor = spin_anc(&sp.anchor);
        }
    }

    if keys.just_pressed(KeyCode::Q) {
        for mut sp in text_size.iter_mut() {
            match &mut sp.set_em {
                SetEM::Ems(em) => *em = (*em - 0.1).max(0.1),
                em => *em = SetEM::Ems(1.0),
            }
        }
    }

    if keys.just_pressed(KeyCode::E) {
        for mut sp in text_size.iter_mut() {
            match &mut sp.set_em {
                SetEM::Ems(em) => *em = (*em + 0.1).max(0.1),
                em => *em = SetEM::Ems(1.0),
            }
        }
    }

    if keys.just_pressed(KeyCode::A) {
        for mut sp in main.iter_mut() {
            if let Some(size) = &mut sp.custom_size {
                size.x = (size.x - 10.0).max(10.0);
            }
        }
    }
    if keys.just_pressed(KeyCode::D) {
        for mut sp in main.iter_mut() {
            if let Some(size) = &mut sp.custom_size {
                size.x = (size.x + 10.0).max(10.0);
            }
        }
    }
    if keys.just_pressed(KeyCode::S) {
        for mut sp in main.iter_mut() {
            if let Some(size) = &mut sp.custom_size {
                size.y = (size.y - 10.0).max(10.0);
            }
        }
    }
    if keys.just_pressed(KeyCode::W) {
        for mut sp in main.iter_mut() {
            if let Some(size) = &mut sp.custom_size {
                size.y = (size.y + 10.0).max(10.0);
            }
        }
    }
}
