use bevy::{prelude::*, diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin}};
use bevy_aoui::{AouiPlugin, widgets::richtext::RichTextBuilder};

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
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_systems(Startup, init)
        .add_plugins(AouiPlugin)
        .run();
}


pub fn init(mut commands: Commands, assets: Res<AssetServer>) {
    use bevy_aoui::dsl::prelude::*;
    commands.spawn(Camera2dBundle::default());
    clipping_layer!(commands {
        dimension: [400, 400],
        offset: [-200, 0],
        buffer: [800, 800],
        scroll: Scrolling::Y,
        layer: 1,
        child: text! {
            anchor: TopLeft,
            bounds: [390, 999999],
            color: color!(gold),
            wrap: true,
            layer: 1,
            text: "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Mauris semper magna nibh, nec tincidunt metus fringilla id. Phasellus viverra elit volutpat orci lacinia, non suscipit odio egestas. Praesent urna ipsum, viverra non dui id, auctor sodales sem. Quisque ut mi sit amet quam ultricies cursus at vitae justo. Morbi egestas pulvinar dui id elementum. Aliquam non aliquam eros. Nam euismod in lectus sit amet blandit. Aenean mauris diam, auctor ut massa sed, convallis congue leo. Maecenas non nibh semper, tempor velit sit amet, facilisis lacus. Curabitur nec leo nisl. Proin vitae fringilla nisl. Sed vel hendrerit mi. Donec et cursus risus, at euismod justo.
Ut luctus tellus mi. Donec non lacus ex. Vivamus non rutrum quam. Curabitur in bibendum tellus. Fusce eu gravida massa. Ut viverra vestibulum convallis. Morbi ullamcorper gravida fringilla. Morbi ullamcorper sem eget eleifend sagittis. Mauris interdum odio eget luctus pretium. In non dapibus risus.
Quisque id odio urna. Maecenas aliquam ullamcorper semper. Duis eu pulvinar magna, vel malesuada odio. Morbi lobortis porttitor metus sit amet pellentesque. In convallis feugiat sem, eget varius risus vulputate eget. Ut nec massa cursus, tempor quam nec, vulputate lorem. Nullam nec nisl et odio blandit vulputate. Morbi porta risus dui, nec efficitur sem euismod quis. Integer vel elit massa. Mauris ornare sollicitudin nunc venenatis laoreet. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Aenean aliquet egestas ipsum.
Aenean fringilla faucibus augue, at commodo lectus vestibulum placerat. Fusce et placerat velit. Suspendisse vel risus leo. Aenean in justo nec mauris porta lobortis a vitae nisi. Fusce fermentum ipsum et aliquet varius. Proin vel aliquam risus, et ornare libero. Proin non luctus dui.",
        }
    });


    // This showcases bulk inserting multiple entities with the `with_layer` scope.
    // This will changed the default layer of all macros invoked in the closure.
    // For the `clipped_frame` widget, only `original_layer` is affected by this.
    let entity = with_layer(2, || {
        let mut builder = RichTextBuilder::new(&mut commands, assets.load("ComicNeue-Regular.ttf"))
            .configure_size(assets.load("ComicNeue-Regular.ttf"), 32.0)
            .with_color(Color::WHITE);
            // `.with_layer(2)` also works, but we are showcasing bulk insertion here.

        builder.push_str(r#"{red:Lorem ipsum dolor sit amet, consectetur adipiscing elit. Mauris semper magna nibh, nec tincidunt metus fringilla id. Phasellus viverra elit volutpat orci lacinia, non suscipit odio egestas. Praesent urna ipsum, viverra non dui id, auctor sodales sem. Quisque ut mi sit amet quam ultricies cursus at vitae justo. Morbi egestas pulvinar dui id elementum. Aliquam non aliquam eros. Nam euismod in lectus sit amet blandit. Aenean mauris diam, auctor ut massa sed, convallis congue leo. Maecenas non nibh semper, tempor velit sit amet, facilisis lacus. Curabitur nec leo nisl. Proin vitae fringilla nisl. Sed vel hendrerit mi. Donec et cursus risus, at euismod justo.}
        
        {green:Ut luctus tellus mi. Donec non lacus ex. Vivamus non rutrum quam. Curabitur in bibendum tellus. Fusce eu gravida massa. Ut viverra vestibulum convallis. Morbi ullamcorper gravida fringilla. Morbi ullamcorper sem eget eleifend sagittis. Mauris interdum odio eget luctus pretium. In non dapibus risus.}
        
        {blue:Quisque id odio urna. Maecenas aliquam ullamcorper semper. Duis eu pulvinar magna, vel malesuada odio. Morbi lobortis porttitor metus sit amet pellentesque. In convallis feugiat sem, eget varius risus vulputate eget. Ut nec massa cursus, tempor quam nec, vulputate lorem. Nullam nec nisl et odio blandit vulputate. Morbi porta risus dui, nec efficitur sem euismod quis. Integer vel elit massa. Mauris ornare sollicitudin nunc venenatis laoreet. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Aenean aliquet egestas ipsum.}
        
        {gold:Aenean fringilla faucibus augue, at commodo lectus vestibulum placerat. Fusce et placerat velit. Suspendisse vel risus leo. Aenean in justo nec mauris porta lobortis a vitae nisi. Fusce fermentum ipsum et aliquet varius. Proin vel aliquam risus, et ornare libero. Proin non luctus dui.}"#).unwrap();
        let children = builder.build();

        let para = paragraph!(commands {
            anchor: TopLeft,
            dimension: [400, 400],
        });
        commands.entity(para).push_children(&children).id()
    });
    clipping_layer!(commands {
        dimension: [400, 400],
        offset: [200, 0],
        buffer: [800, 800],
        scroll: Scrolling::Y,
        layer: 2,
        child: entity
    });
}
