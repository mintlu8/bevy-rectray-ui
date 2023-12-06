use bevy::prelude::*;
use bevy_aoui::AoUIPlugin;
use bevy_aoui_widgets::{AoUIExtensionsPlugin, widgets::{CursorDefault, inputbox::SignalFormat}};
use bevy_prototype_lyon::prelude::*;


pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: bevy::window::PresentMode::AutoNoVsync,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_systems(Startup, init)
        .init_resource::<CursorDefault>()
        .add_plugins(AoUIPlugin)
        .add_plugins(AoUIExtensionsPlugin)
        .add_plugins(ShapePlugin)
        .run();
}


pub fn init(mut commands: Commands, assets: Res<AssetServer>) {
    use bevy_aoui_widgets::dsl::prelude::*;
    commands.spawn(Camera2dBundle::default());
    let (submit_sender, recv_s1, recv_s2) = signal();
    let (change_sender, recv_c1, recv_c2) = signal();
    inputbox! ((commands, assets) {
        dimension: size2!([400, 1 em]),
        offset: [0, 200],
        font_size: em(4),
        hitbox: Rect(1),
        text: "Type here and press Enter.",
        font: assets.load::<Font>("RobotoCondensed.ttf"),
        color: color!(red),
        cursor_bar: shape! {
            shape: Shapes::Rectangle,
            fill: color!(gold),
            z: -0.1,
            dimension: size2!([2, 1 em]),
        },
        cursor_area: shape! {
            shape: Shapes::Rectangle,
            fill: color!(green) * 0.5,
            z: -0.2,
            dimension: size2!([12, 1 em]),
        },
        submit: submit_sender,
        change: change_sender,
    });

    textbox!(commands {
        text: "This is a receiver.",
        offset: [-200, 0],
        font: assets.load::<Font>("RobotoCondensed.ttf"),
        extra: SignalFormat::<Submit>::COPY,
        extra: recv_s1.mark::<Submit>(),
    });

    textbox!(commands {
        text: "This is a formatter.",
        offset: [-200, -200],
        font: assets.load::<Font>("RobotoCondensed.ttf"),
        extra: SignalFormat::<Submit>::format("Received string \"{%}\"!"),
        extra: recv_s2.mark::<Submit>(),
    });

    textbox!(commands {
        text: "This is a change detector.",
        offset: [200, 0],
        font: assets.load::<Font>("RobotoCondensed.ttf"),
        extra: SignalFormat::<Change>::COPY,
        extra: recv_c1.mark::<Change>(),
    });

    textbox!(commands {
        text: "This is a change detecting formatter.",
        offset: [200, -200],
        font: assets.load::<Font>("RobotoCondensed.ttf"),
        extra: SignalFormat::<Change>::format("\"{%}\"!"),
        extra: recv_c2.mark::<Change>(),
    });
}
