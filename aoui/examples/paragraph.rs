use std::f32::consts::PI;

use bevy_aoui::{*, bundles::*, layout::*};
use bevy::{prelude::*, diagnostic::{LogDiagnosticsPlugin, FrameTimeDiagnosticsPlugin}};
use bevy_egui::{EguiContexts, egui::{self, Slider, ComboBox}, EguiPlugin};


static LOREM_IPSUM: &str = 
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
        .add_plugins(EguiPlugin)
        .add_plugins(AoUIPlugin)
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_systems(Startup, init)
        .add_systems(Update, egui_window)
        .run();
}

pub fn init(mut commands: Commands, assets: Res<AssetServer>) {

    commands.spawn(Camera2dBundle::default());

    let textbox = commands.spawn((AoUISpriteBundle {
        transform: Transform2D::UNIT,
        dimension: Dimension::pixels(Vec2::new(700.0, 700.0)).with_em(SetEM::Ems(1.0)),
        sprite: Sprite { 
            color: Color::DARK_GRAY,
            ..Default::default()
        },
        texture: assets.load("square.png"),
        ..Default::default()
    }, Container {
        layout: Box::new(ParagraphLayout { 
            direction: FlexDir::LeftToRight, 
            stack: FlexDir::TopToBottom,
            stretch: false,
        }),
        margin: Size2::em(0.4, 0.0),
    })).id();
    let mut words = Vec::new();

    LOREM_IPSUM.split('\n').for_each(|x| {
        x.split([' ']).filter(|x| !x.is_empty()).for_each(|w| {
            words.push(
                commands.spawn(AoUITextBundle {
                    text: Text::from_section(w, TextStyle{
                        color: Color::WHITE,
                        ..Default::default()
                    }),
                    transform: Transform2D {
                        anchor: Anchor::TopLeft,
                        ..Default::default()
                    },
                    ..Default::default()
                }).id()
            );
        });
        words.push(commands.spawn(LinebreakBundle::new(Size2::em(1.0, 1.0))).id());
    });
    commands.entity(textbox).push_children(&words[0..words.len()-1]);
}


pub fn egui_window(mut ctx: EguiContexts, 
    mut container: Query<(&mut Container, &mut Transform2D, &mut Dimension)>,
    mut spawned: Query<&mut Transform2D, (Without<Container>, With<Text>)>,
) {
    let (mut container, mut transform, mut dimension) = container.single_mut();

    egui::Window::new("Console").show(ctx.ctx_mut(), |ui| {
        
        ui.label("Paragraph");

        let Vec2 { x, y } = dimension.raw_mut();
        ui.add(Slider::new(x, 0.0..=2000.0).text("width"));
        ui.add(Slider::new(y, 0.0..=2000.0).text("height"));

        ui.add(Slider::new(&mut transform.rotation, -PI * 2.0..=PI * 2.0).text("rotate"));
        let Vec2 { x, y } = &mut transform.scale;

        ui.add(Slider::new(x, 0.0..=4.0).text("scale x"));
        ui.add(Slider::new(y, 0.0..=4.0).text("scale y"));

        let mut anc = spawned.iter().next().unwrap().anchor.str_name();

        ComboBox::from_label("Anchor")
            .selected_text(anc)
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut anc, "BottomLeft", "BottomLeft");
                ui.selectable_value(&mut anc, "BottomCenter", "BottomCenter");
                ui.selectable_value(&mut anc, "BottomRight", "BottomRight");
                ui.selectable_value(&mut anc, "CenterLeft", "CenterLeft");
                ui.selectable_value(&mut anc, "Center", "Center");
                ui.selectable_value(&mut anc, "CenterRight", "CenterRight");
                ui.selectable_value(&mut anc, "TopLeft", "TopLeft");
                ui.selectable_value(&mut anc, "TopCenter", "TopCenter");
                ui.selectable_value(&mut anc, "TopRight", "TopRight");
            });
        
        let result_anchor = match anc {
            "BottomLeft" => Anchor::BottomLeft,
            "BottomCenter" =>  Anchor::BottomCenter,
            "BottomRight" => Anchor::BottomRight,
            "CenterLeft" => Anchor::CenterLeft,
            "Center" => Anchor::Center,
            "CenterRight" => Anchor::CenterRight,
            "TopLeft" => Anchor::TopLeft,
            "TopCenter" => Anchor::TopCenter,
            "TopRight" => Anchor::TopRight,
            _ => unreachable!(),
        };

        spawned.iter_mut().for_each(|mut x| x.anchor = result_anchor);


        let font_size = dimension.set_em.raw_mut();
        ui.add(Slider::new(font_size, 0.0..=12.0).text("font size (em)"));

        let Some(ParagraphLayout { direction, stack, stretch }) = container.layout.downcast_mut() else {return};

        ComboBox::from_label("Direction")
            .selected_text(match direction {
                FlexDir::LeftToRight => "left to right",
                FlexDir::RightToLeft => "right to left",
                FlexDir::BottomToTop => "bottom to top",
                FlexDir::TopToBottom => "top to bottom",
            })
            .show_ui(ui, |ui| {
                ui.selectable_value(direction, FlexDir::LeftToRight, "left to right");
                ui.selectable_value(direction, FlexDir::RightToLeft, "right to left");
                ui.selectable_value(direction, FlexDir::BottomToTop, "bottom to top");
                ui.selectable_value(direction, FlexDir::TopToBottom, "top to bottom");
            });
        match direction {
            FlexDir::LeftToRight|FlexDir::RightToLeft => {
                ComboBox::from_label("Stack")
                    .selected_text(match stack {
                        FlexDir::BottomToTop => "bottom to top",
                        FlexDir::TopToBottom => "top to bottom",
                        _ => {
                            *stack = FlexDir::TopToBottom;
                            "bottom to top"
                        }
                    })
                    .show_ui(ui, |ui| {
                        ui.selectable_value(stack, FlexDir::BottomToTop, "bottom to top");
                        ui.selectable_value(stack, FlexDir::TopToBottom, "top to bottom");
                    });
            },
            FlexDir::BottomToTop|FlexDir::TopToBottom => {
                ComboBox::from_label("Stack")
                    .selected_text(match stack {
                        FlexDir::LeftToRight => "left to right",
                        FlexDir::RightToLeft => "right to left",
                        _ => {
                            *stack = FlexDir::LeftToRight;
                            "left to right"
                        }
                    })
                    .show_ui(ui, |ui| {
                        ui.selectable_value(stack, FlexDir::LeftToRight, "left to right");
                        ui.selectable_value(stack, FlexDir::RightToLeft, "right to left");
                    });
            },
        }
        ui.checkbox(stretch, "Stretch");
        
    
        let Vec2 { x, y } = container.margin.raw_mut();
        ui.add(Slider::new(x, 0.0..=10.0).text("margin x (em)"));
        ui.add(Slider::new(y, 0.0..=10.0).text("margin y (em)"));
    });
}
