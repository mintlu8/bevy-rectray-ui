use bevy::prelude::*;
use bevy_aoui::{AoUIPlugin, widgets::scroll::{Scrolling, ScrollDirection}};

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
        .add_plugins(AoUIPlugin)
        // classic macos stuff
        .insert_resource(ScrollDirection::INVERTED)
        .run();
}


pub fn init(mut commands: Commands, assets: Res<AssetServer>) {
    use bevy_aoui::dsl::prelude::*;
    commands.spawn(Camera2dBundle::default());
    sprite! (commands {
        dimension: [200, 60],
        offset: [-200, 200],
        hitbox: Rect(1),
        sprite: assets.load("square.png"),
        extra: EventFlags::MouseWheel,
        extra: Scrolling::X,
        child: frame! {
            dimension: size2!([100%, 100%]),
            child: sprite! {
                dimension: [60, 60],
                sprite: assets.load("square.png"),
                color: color!(red),
            }
        }
    });
    sprite! (commands {
        dimension: [100, 100],
        offset: [0, 200],
        hitbox: Rect(1),
        sprite: assets.load("square.png"),
        extra: EventFlags::MouseWheel,
        extra: Scrolling::BOTH,
        child: frame! {
            dimension: size2!([100%, 100%]),
            child: sprite! {
                dimension: [60, 150],
                sprite: assets.load("square.png"),
                color: color!(red),
            }
        }
    });
    sprite! (commands {
        dimension: [60, 100],
        offset: [200, 200],
        hitbox: Rect(1),
        sprite: assets.load("square.png"),
        extra: EventFlags::MouseWheel,
        extra: Scrolling::X,
        child: frame! {
            dimension: size2!([100%, 100%]),
            child: sprite! {
                dimension: [200, 60],
                sprite: assets.load("square.png"),
                color: color!(red),
            }
        }
    });
    sprite! (commands {
        dimension: [200, 200],
        offset: [-200, 0],
        hitbox: Rect(1),
        sprite: assets.load("square.png"),
        extra: EventFlags::MouseWheel,
        extra: Scrolling::BOTH,
        child: frame! {
            dimension: size2!([100%, 100%]),
            child: sprite! {
                dimension: [60, 60],
                sprite: assets.load("square.png"),
                color: color!(red),
            }
        }
    });
    sprite! (commands {
        dimension: [200, 200],
        hitbox: Rect(1),
        sprite: assets.load("square.png"),
        extra: EventFlags::MouseWheel,
        extra: Scrolling::BOTH,
        child: frame! {
            dimension: size2!([100%, 100%]),
            child: sprite! {
                dimension: [60, 60],
                offset: [-40, 0],
                sprite: assets.load("square.png"),
                color: color!(red),
            },
            child: sprite! {
                dimension: [60, 60],
                offset: [40, 20],
                sprite: assets.load("square.png"),
                color: color!(red),
            },
            child: sprite! {
                dimension: [60, 60],
                offset: [30, -50],
                sprite: assets.load("square.png"),
                color: color!(red),
            }
        }
    });
    sprite! (commands {
        dimension: [100, 100],
        offset: [200, 0],
        hitbox: Rect(1),
        sprite: assets.load("square.png"),
        extra: EventFlags::MouseWheel,
        extra: Scrolling::BOTH,
        child: frame! {
            dimension: size2!([100%, 100%]),
            child: sprite! {
                dimension: [200, 200],
                z: -1,
                sprite: assets.load("square.png"),
                color: color!(red),
            }
        }
    });

    sprite! (commands {
        dimension: [60, 200],
        offset: [-200, -200],
        hitbox: Rect(1),
        sprite: assets.load("square.png"),
        extra: EventFlags::MouseWheel,
        extra: Scrolling::Y,
        child: frame! {
            dimension: size2!([100%, 100%]),
            child: sprite! {
                dimension: [60, 60],
                sprite: assets.load("square.png"),
                color: color!(red),
            }
        }
    });
    sprite! (commands {
        dimension: [100, 100],
        offset: [0, -200],
        hitbox: Rect(1),
        sprite: assets.load("square.png"),
        extra: EventFlags::MouseWheel,
        extra: Scrolling::BOTH,
        child: frame! {
            dimension: size2!([100%, 100%]),
            child: sprite! {
                dimension: [150, 60],
                sprite: assets.load("square.png"),
                color: color!(red),
            }
        }
    });
    sprite! (commands {
        dimension: [100, 60],
        offset: [200, -200],
        hitbox: Rect(1),
        sprite: assets.load("square.png"),
        extra: EventFlags::MouseWheel,
        extra: Scrolling::Y,
        child: frame! {
            dimension: size2!([100%, 100%]),
            child: sprite! {
                dimension: [60, 200],
                sprite: assets.load("square.png"),
                color: color!(red),
            }
        }
    });
}
