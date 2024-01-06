//! examples using richtext
//! 
//! Note this just compiles to bevy_text, nothing fancy here.

use bevy::{prelude::*, utils::HashMap};
use bevy_aoui::{AouiPlugin, widgets::richtext::{RichTextBuilder, FontStyle}, dsl::AouiCommands};

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
        .add_plugins(AouiPlugin)
        .run();
}


pub fn init(mut commands: AouiCommands) {
    use bevy_aoui::dsl::prelude::*;
    commands.spawn_bundle(Camera2dBundle::default());
    let rich = paragraph! (commands {
        dimension: [600, 600],
        //margin: [8, 0],
        font_size: 32,
    });
    let fonts = HashMap::from([
        (("comicneue", FontStyle::None), commands.load("ComicNeue-Regular.ttf")),
        (("comicneue", FontStyle::Bold), commands.load("ComicNeue-Bold.ttf")),
        (("comicneue", FontStyle::Italic), commands.load("ComicNeue-Italic.ttf")),
        (("comicneue", FontStyle::Bold|FontStyle::Italic), commands.load("ComicNeue-BoldItalic.ttf")),
        (("roboto", FontStyle::None), commands.load("RobotoCondensed.ttf")),
        (("roboto", FontStyle::Bold), commands.load("RobotoCondensed-Bold.ttf")),
        (("roboto", FontStyle::Italic), commands.load("RobotoCondensed-Italic.ttf")),
        (("roboto", FontStyle::Bold|FontStyle::Italic), commands.load("RobotoCondensed-BoldItalic.ttf")),
    ]);
    let default = commands.load("ComicNeue-Regular.ttf");
    let mut builder = RichTextBuilder::new(&mut commands, fonts)
        .configure_size(default.clone(), 32.0)
        .with_font("roboto")
        .with_color(Color::WHITE);

    builder.push_str(r#"
Hello, {orange:Rustaceans!}

Let's make our text {red:red} and {blue:blue.}

We can make it **bold** or *italic,* even {red:red and ***bold and italic.***}


We can use a different {@comicneue: font}.

We can align our text

left, {center:center,} {right:or right}

We can make our font {*2:bigger} or {*0.5:smaller}.

Let's permanently change our font to {@comicneue} ComicNeue,
and use the color {green} green,
{right} and right align everything.

Awesome {br} right?
"#).unwrap();

    let children = builder.build();
    commands.entity(rich).push_children(&children);


    let zip_test = paragraph! (commands {
        dimension: [10, 600],
        anchor: Left,
        //margin: [8, 0],
        font_size: 32,
    });


    let mut builder = RichTextBuilder::new(&mut commands, default.clone())
        .configure_size(default, 32.0)
        .with_color(Color::WHITE);
        //.with_ignore_space(true);

    builder.push_str(r#"
    ZipTest

    {red:ThisDoesn'tWork},

    {zip:{red:ButThisDoes},}

    {zip:{red:AndThisToo}!!}
"#).unwrap();
    let children = builder.build();
    commands.entity(zip_test).push_children(&children);
}
