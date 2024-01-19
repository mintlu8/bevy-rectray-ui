use bevy::{prelude::*, diagnostic::FrameTimeDiagnosticsPlugin};
use bevy_aoui::{AouiPlugin, widgets::richtext::RichTextBuilder, util::AouiCommands};

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
        .run();
}


pub fn init(mut commands: AouiCommands) {
    use bevy_aoui::dsl::prelude::*;
    commands.spawn_bundle(Camera2dBundle::default());

    text!(commands {
        anchor: TopRight,
        text: "FPS: 0.00",
        color: color!(gold),
        extra: async_systems!(|fps: FPS, text: Ac<Text>| {
            let fps = fps.get().await;
            text.set(move |text| format_widget!(text, "FPS: {:.2}", fps)).await?;
        })
    });
    
    let (target_in, target_out) = commands.render_target([800, 800]);

    camera_frame!(commands {
        dimension: [400, 400],
        offset: [-200, 0],
        render_target: target_in,
        layer: 1,
    });

    sprite!(commands {
        dimension: [400, 400],
        offset: [-200, 0],
        sprite: target_out
    });
    

    scrolling!(commands {
        dimension: [400, 400],
        offset: [-200, 0],
        scroll: Scrolling::Y,
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

    let (target_in, target_out) = commands.render_target([800, 800]);
    
    camera_frame!(commands {
        dimension: [400, 400],
        offset: [200, 0],
        render_target: target_in,
        layer: 1,
    });

    sprite!(commands {
        dimension: [400, 400],
        offset: [200, 0],
        sprite: target_out
    });
    
    let font = commands.load("ComicNeue-Regular.ttf");
    let entity = {
        let mut builder = RichTextBuilder::new(&mut commands, font.clone())
            .configure_size(font, 32.0)
            .with_color(Color::WHITE)
            .with_layer(1);

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
    };
    scrolling!(commands {
        dimension: [400, 400],
        offset: [200, 0],
        scroll: Scrolling::Y,
        layer: 2,
        child: entity
    });
}
