use bevy::prelude::*;
use bevy_aoui::{AouiPlugin, WorldExtension, dsl::AouiCommands};


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
        .register_cursor_default(CursorIcon::Arrow)
        .add_plugins(AouiPlugin)
        .run();
}


pub fn init(mut commands: AouiCommands) {
    use bevy_aoui::dsl::prelude::*;
    commands.spawn_bundle(Camera2dBundle::default());
    let (submit_sender, recv_s1, recv_s2) = commands.signal();
    let (change_sender, recv_c1, recv_c2) = commands.signal();
    inputbox! (commands {
        dimension: size2!(800, 1 em),
        offset: [0, 200],
        font_size: em(4),
        hitbox: Hitbox::rect(1),
        text: "Type here and press Enter.",
        font: commands.load::<Font>("RobotoCondensed.ttf"),
        color: color!(red),
        cursor_bar: rectangle! {
            color: color!(gold),
            z: 0.1,
            dimension: size2!(2, 1 em),
        },
        cursor_area: rectangle! {
            color: color!(green) * 0.5,
            z: -0.1,
            dimension: size2!(12, 1 em),
        },
        on_submit: submit_sender,
        on_change: change_sender,
        child: rectangle! {
            dimension: Size2::FULL,
            color: color!(darkgrey),
            z: -0.2
        }
    });

    text!(commands {
        text: "This is a receiver.",
        offset: [-200, 0],
        font: "RobotoCondensed.ttf",
        extra: recv_s1.recv(|s: String, text: &mut Text| format_widget!(text, "{}", s))
    });

    text!(commands {
        text: "This is a formatter.",
        offset: [-200, -200],
        font: "RobotoCondensed.ttf",
        extra: recv_s2.recv(|s: String, text: &mut Text| format_widget!(text, "Received string \"{}\"!", s))
    });

    text!(commands {
        text: "This is a change detector.",
        offset: [200, 0],
        font: "RobotoCondensed.ttf",
        extra: recv_c1.recv(|s: String, text: &mut Text| format_widget!(text, "{}", s))
    });

    text!(commands {
        text: "This is a change detecting formatter.",
        offset: [200, -200],
        font: "RobotoCondensed.ttf",
        extra: recv_c2.recv(|s: String, text: &mut Text| format_widget!(text, "Received string \"{}\"!", s))
    });
}
