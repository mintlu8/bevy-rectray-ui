use bevy::{prelude::*, diagnostic::FrameTimeDiagnosticsPlugin};
use bevy_aoui::{AouiPlugin, dsl::AouiCommands};

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: bevy::window::PresentMode::AutoNoVsync,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_systems(Startup, init)
        .add_plugins(AouiPlugin)
        // classic macos stuff
        .run();
}


pub fn init(mut commands: AouiCommands) {
    use bevy_aoui::dsl::prelude::*;
    commands.spawn_bundle(Camera2dBundle::default());

    text!(commands {
        anchor: TopRight,
        text: "FPS: 0.00",
        color: color!(gold),
        extra: fps_channel(|fps: f32, text: &mut Text| {
            format_widget!(text, "FPS: {:.2}", fps);
        })
    });

    let (send1, recv1) = commands.signal();

    text! (commands {
        offset: [-400, 200],
        color: color!(gold),
        text: "Scroll this! =>",
        extra: recv1.recv(|x: f32, text: &mut Text| format_widget!(text, "This has value {:.2}! =>", x))
    });
    
    sprite! (commands {
        dimension: [200, 60],
        offset: [-200, 200],
        hitbox: Rect(1),
        sprite: commands.load("square.png"),
        event: EventFlags::MouseWheel,
        extra: Scrolling::X.with_handler(send1),
        child: frame! {
            dimension: size2!(100%, 100%),
            child: sprite! {
                dimension: [60, 60],
                sprite: commands.load("square.png"),
                color: color!(red),
            }
        }
    });
    sprite! (commands {
        dimension: [100, 100],
        offset: [0, 200],
        hitbox: Rect(1),
        sprite: commands.load("square.png"),
        event: EventFlags::MouseWheel,
        extra: Scrolling::BOTH.with_constraints(),
        child: frame! {
            dimension: size2!(100%, 100%),
            child: sprite! {
                dimension: [60, 150],
                sprite: commands.load("square.png"),
                color: color!(red),
            }
        }
    });
    sprite! (commands {
        dimension: [60, 100],
        offset: [200, 200],
        hitbox: Rect(1),
        sprite: commands.load("square.png"),
        event: EventFlags::MouseWheel,
        extra: Scrolling::X.with_constraints(),
        child: frame! {
            dimension: size2!(100%, 100%),
            child: sprite! {
                dimension: [200, 60],
                sprite: commands.load("square.png"),
                color: color!(red),
            }
        }
    });
    sprite! (commands {
        dimension: [200, 200],
        offset: [-200, 0],
        hitbox: Rect(1),
        sprite: commands.load("square.png"),
        event: EventFlags::MouseWheel,
        extra: Scrolling::BOTH.with_constraints(),
        child: frame! {
            dimension: size2!(100%, 100%),
            child: sprite! {
                dimension: [60, 60],
                sprite: commands.load("square.png"),
                color: color!(red),
            }
        }
    });
    sprite! (commands {
        dimension: [200, 200],
        hitbox: Rect(1),
        sprite: commands.load("square.png"),
        event: EventFlags::MouseWheel,
        extra: Scrolling::BOTH.with_constraints(),
        child: frame! {
            dimension: size2!(100%, 100%),
            child: sprite! {
                dimension: [60, 60],
                offset: [-40, 0],
                sprite: commands.load("square.png"),
                color: color!(red),
            },
            child: sprite! {
                dimension: [60, 60],
                offset: [40, 20],
                sprite: commands.load("square.png"),
                color: color!(red),
            },
            child: sprite! {
                dimension: [60, 60],
                offset: [30, -50],
                sprite: commands.load("square.png"),
                color: color!(red),
            }
        }
    });
    sprite! (commands {
        dimension: [100, 100],
        offset: [200, 0],
        hitbox: Rect(1),
        sprite: commands.load("square.png"),
        extra: EventFlags::MouseWheel,
        extra: Scrolling::BOTH.with_constraints(),
        child: frame! {
            dimension: size2!(100%, 100%),
            child: sprite! {
                dimension: [200, 200],
                z: -1,
                sprite: commands.load("square.png"),
                color: color!(red),
            }
        }
    });

    let (send2, recv2) = commands.signal();

    text! (commands {
        offset: [-400, -200],
        color: color!(gold),
        text: "Scroll this! =>",
        extra: recv2.recv(|x: f32, text: &mut Text| format_widget!(text, "This has value {:.2}! =>", x))
    });

    sprite! (commands {
        dimension: [60, 200],
        offset: [-200, -200],
        hitbox: Rect(1),
        sprite: commands.load("square.png"),
        event: EventFlags::MouseWheel,
        extra: Scrolling::Y.with_handler(send2),
        child: frame! {
            dimension: size2!(100%, 100%),
            child: sprite! {
                dimension: [60, 60],
                sprite: commands.load("square.png"),
                color: color!(red),
            }
        }
    });

    sprite! (commands {
        dimension: [100, 100],
        offset: [0, -200],
        hitbox: Rect(1),
        sprite: commands.load("square.png"),
        event: EventFlags::MouseWheel,
        extra: Scrolling::BOTH.with_constraints(),
        child: frame! {
            dimension: size2!(100%, 100%),
            child: sprite! {
                dimension: [150, 60],
                sprite: commands.load("square.png"),
                color: color!(red),
            }
        }
    });

    sprite! (commands {
        dimension: [100, 60],
        offset: [200, -200],
        hitbox: Rect(1),
        sprite: commands.load("square.png"),
        event: EventFlags::MouseWheel,
        extra: Scrolling::Y.with_constraints(),
        child: frame! {
            dimension: size2!(100%, 100%),
            child: sprite! {
                dimension: [60, 200],
                sprite: commands.load("square.png"),
                color: color!(red),
            }
        }
    });
}
