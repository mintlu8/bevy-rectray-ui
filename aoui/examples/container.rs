//! This is a demo most layout types for `bevy_aoui`'s container.
#![allow(clippy::type_complexity)]
use std::f32::consts::PI;

use bevy_aoui::{*, bundles::*, layout::*};
use bevy_egui::{self, EguiContexts, egui::{self, ComboBox, Grid, Slider}};
use bevy::prelude::*;
use rand::Rng;

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: bevy::window::PresentMode::AutoNoVsync,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins(bevy_egui::EguiPlugin)
        .add_systems(Startup, init)
        .add_plugins(AoUIPlugin)
        .add_systems(Update, egui_window)
        .insert_resource(ChildSize(Vec2::splat(30.0)))
        .run();
}

#[derive(Component)]
pub struct Root;

#[derive(Component)]
pub struct RootFlex;

pub fn init(mut commands: Commands, assets: Res<AssetServer>) {
    let texture = assets.load::<Image>("square.png");
    commands.spawn(Camera2dBundle::default());

    let container = commands.spawn((AoUISpriteBundle {
        sprite: Sprite {
            color: Color::rgb_linear(0.8, 0.8, 0.8),
            ..Default::default()
        },
        dimension: Dimension::INHERIT,
        ..Default::default()
    }, Container {
        layout: Box::new(SpanLayout {
            direction: FlexDir::LeftToRight,
            stretch: false,
        }),
        margin: Size2::pixels(2.0, 2.0),
    }, RootFlex)).id();

    commands.spawn((AoUISpriteBundle {
        sprite: Sprite {
            color: Color::WHITE,
            ..Default::default()
        },
        dimension: Dimension { 
            dim: DimensionSize::Owned(Size2::pixels(600.0, 100.0)), 
            ..Default::default()
        },
        texture: texture.clone(),
        ..Default::default()
    }, Root)).add_child(container);

    
}

pub fn random_color() -> Color {
    let mut rng = rand::thread_rng();
    Color::Hsla { hue: rng.gen_range(0.0..=360.0), saturation: 1.0, lightness: 0.5, alpha: 1.0 }
}

pub fn spawn(commands: &mut Commands, anchor: Anchor, size: Vec2, flexbox: Entity, assets: &Res<AssetServer>){
    let child = commands.spawn(AoUISpriteBundle {
        sprite: Sprite {
            color: random_color(),
            ..Default::default()
        },
        dimension: Dimension::pixels(size),
        texture: assets.load::<Image>("square.png"),
        transform: Transform2D {
            anchor,
            ..Default::default()
        },
        ..Default::default()
    }).id();
    commands.entity(flexbox).add_child(child);
}

#[derive(Resource)]
pub struct ChildSize(Vec2);

pub fn egui_window(mut commands: Commands, mut ctx: EguiContexts, 
    mut root: Query<(&mut Transform2D, &mut Dimension), With<Root>>, 
    mut container: Query<(Entity, &mut Container, &mut Transform2D), (With<RootFlex>, Without<Root>)>,
    spawned: Query<Entity, (With<Transform2D>,  Without<Root>, Without<RootFlex>)>, 
    assets: Res<AssetServer>,
    mut child_size: ResMut<ChildSize>,
) {
    let (mut transform, mut dimension) = root.single_mut();
    let (flexbox, mut container, mut transform2) = container.single_mut();
    let mut layout_type = match &container.layout {
        x if x.downcast_ref::<CompactLayout>().is_some() => "compact",
        x if x.downcast_ref::<SpanLayout>().is_some() => "span",
        x if x.downcast_ref::<ParagraphLayout>().is_some() => "paragraph",
        x if x.downcast_ref::<FixedGridLayout>().is_some() => "fixed grid",
        x if x.downcast_ref::<SizedGridLayout>().is_some() => "sized grid",
        x if x.downcast_ref::<DynamicTableLayout>().is_some() => "table",
        _ => unimplemented!(),
    };

    egui::Window::new("Console").show(ctx.ctx_mut(), |ui| {
        
        ui.label("AoUI Container");

        let Vec2 { x, y } = dimension.raw_mut();
        ui.add(Slider::new(x, 0.0..=2000.0).text("width"));
        ui.add(Slider::new(y, 0.0..=2000.0).text("height"));

        ui.add(Slider::new(&mut transform.rotation, -PI * 2.0..=PI * 2.0).text("rotate"));
        let Vec2 { x, y } = &mut transform.scale;

        ui.add(Slider::new(x, 0.0..=4.0).text("scale x"));
        ui.add(Slider::new(y, 0.0..=4.0).text("scale y"));

        let mut anc = transform2.anchor.str_name();

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

        
            transform2.anchor = match anc {
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

        ComboBox::from_label("Layout Type")
            .selected_text(layout_type)
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut layout_type, "compact", "compact");
                ui.selectable_value(&mut layout_type, "span", "span");
                ui.selectable_value(&mut layout_type, "paragraph", "paragraph");
                ui.selectable_value(&mut layout_type, "grid", "grid");
                ui.selectable_value(&mut layout_type, "table", "table");
            });
        match layout_type {
            "compact" => {
                if let Some(CompactLayout { direction }) = container.layout.downcast_mut() {
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
                } else {
                    container.layout = Box::new(CompactLayout { 
                        direction: FlexDir::LeftToRight
                    })
                }
            }
            "span" => {
                if let Some(SpanLayout { direction, stretch }) = container.layout.downcast_mut() {
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
                    ui.checkbox(stretch, "Stretch");
                } else {
                    container.layout = Box::new(SpanLayout { 
                        direction: FlexDir::LeftToRight, 
                        stretch: false 
                    })
                }
            }
            "paragraph" => {
                if let Some(ParagraphLayout { direction, stack, stretch }) = container.layout.downcast_mut() {
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
                } else {
                    container.layout = Box::new(ParagraphLayout { 
                        direction: FlexDir::LeftToRight, 
                        stack: FlexDir::TopToBottom, 
                        stretch: false
                    })
                }
            }
            "sized grid" => {
                if let Some(SizedGridLayout { cell_size, row_dir, column_dir, alignment, stretch }) = container.layout.downcast_mut() {
                    let Vec2 { x, y } = cell_size.raw_mut();
                    ui.add(Slider::new(x, 0.0..=200.0).text("width"));
                    ui.add(Slider::new(y, 0.0..=200.0).text("height"));
                    ComboBox::from_label("Row Direction")
                        .selected_text(match row_dir {
                            FlexDir::LeftToRight => "left to right",
                            FlexDir::RightToLeft => "right to left",
                            FlexDir::BottomToTop => "bottom to top",
                            FlexDir::TopToBottom => "top to bottom",
                        })
                        .show_ui(ui, |ui| {
                            ui.selectable_value(row_dir, FlexDir::LeftToRight, "left to right");
                            ui.selectable_value(row_dir, FlexDir::RightToLeft, "right to left");
                            ui.selectable_value(row_dir, FlexDir::BottomToTop, "bottom to top");
                            ui.selectable_value(row_dir, FlexDir::TopToBottom, "top to bottom");
                        });
                    match row_dir {
                        FlexDir::LeftToRight|FlexDir::RightToLeft => {
                            ComboBox::from_label("Column Direction")
                                .selected_text(match column_dir {
                                    FlexDir::TopToBottom => "top to bottom",
                                    FlexDir::BottomToTop => "bottom to top",
                                    _ => {
                                        *column_dir = FlexDir::TopToBottom;
                                        "top to bottom"
                                    }
                                })
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(column_dir, FlexDir::TopToBottom, "top to bottom");
                                    ui.selectable_value(column_dir, FlexDir::BottomToTop, "bottom to top");
                                });
                            ComboBox::from_label("Row Alignment")
                                .selected_text(match alignment {
                                    Alignment::Left => "left",
                                    Alignment::Center => "center",
                                    Alignment::Right => "right",
                                    _ => {
                                        *alignment = Alignment::Left;
                                        "left"
                                    }
                                })
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(alignment, Alignment::Left, "left");
                                    ui.selectable_value(alignment, Alignment::Center, "center");
                                    ui.selectable_value(alignment, Alignment::Right, "right");
                                });
                        }
                        FlexDir::BottomToTop|FlexDir::TopToBottom => {
                            ComboBox::from_label("Column Direction")
                                .selected_text(match column_dir {
                                    FlexDir::LeftToRight => "left to right",
                                    FlexDir::RightToLeft => "right to left",
                                    _ => {
                                        *column_dir = FlexDir::LeftToRight;
                                        "left to right"
                                    }
                                })
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(column_dir, FlexDir::LeftToRight, "left to right");
                                    ui.selectable_value(column_dir, FlexDir::RightToLeft, "right to left");
                                });
                            ComboBox::from_label("Row Alignment")
                                .selected_text(match alignment {
                                    Alignment::Top => "top",
                                    Alignment::Center => "center",
                                    Alignment::Bottom => "bottom",
                                    _ => {
                                        *alignment = Alignment::Top;
                                        "top"
                                    }
                                })
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(alignment, Alignment::Top, "top");
                                    ui.selectable_value(alignment, Alignment::Center, "center");
                                    ui.selectable_value(alignment, Alignment::Bottom, "bottom");
                                });
                        }
                    }
                    
                    ui.checkbox(stretch, "Stretch");
                } else {
                    container.layout = Box::new(SizedGridLayout { 
                        cell_size: Vec2::splat(40.0).into(),
                        row_dir: FlexDir::LeftToRight, 
                        column_dir: FlexDir::TopToBottom, 
                        alignment: Alignment::Left,
                        stretch: false, 
                    })
                }
            }
            "fixed grid" => {
                if let Some(FixedGridLayout { cells, row_dir, column_dir, alignment }) = container.layout.downcast_mut() {
                    let UVec2 { x, y } = cells;
                    ui.add(Slider::new(x, 1..=50).text("width"));
                    ui.add(Slider::new(y, 1..=50).text("height"));
                    ComboBox::from_label("Row Direction")
                        .selected_text(match row_dir {
                            FlexDir::LeftToRight => "left to right",
                            FlexDir::RightToLeft => "right to left",
                            FlexDir::BottomToTop => "bottom to top",
                            FlexDir::TopToBottom => "top to bottom",
                        })
                        .show_ui(ui, |ui| {
                            ui.selectable_value(row_dir, FlexDir::LeftToRight, "left to right");
                            ui.selectable_value(row_dir, FlexDir::RightToLeft, "right to left");
                            ui.selectable_value(row_dir, FlexDir::BottomToTop, "bottom to top");
                            ui.selectable_value(row_dir, FlexDir::TopToBottom, "top to bottom");
                        });
                    match row_dir {
                        FlexDir::LeftToRight|FlexDir::RightToLeft => {
                            ComboBox::from_label("Column Direction")
                                .selected_text(match column_dir {
                                    FlexDir::TopToBottom => "top to bottom",
                                    FlexDir::BottomToTop => "bottom to top",
                                    _ => {
                                        *column_dir = FlexDir::TopToBottom;
                                        "top to bottom"
                                    }
                                })
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(column_dir, FlexDir::TopToBottom, "top to bottom");
                                    ui.selectable_value(column_dir, FlexDir::BottomToTop, "bottom to top");
                                });
                            ComboBox::from_label("Row Alignment")
                                .selected_text(match alignment {
                                    Alignment::Left => "left",
                                    Alignment::Center => "center",
                                    Alignment::Right => "right",
                                    _ => {
                                        *alignment = Alignment::Left;
                                        "left"
                                    }
                                })
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(alignment, Alignment::Left, "left");
                                    ui.selectable_value(alignment, Alignment::Center, "center");
                                    ui.selectable_value(alignment, Alignment::Right, "right");
                                });
                        }
                        FlexDir::BottomToTop|FlexDir::TopToBottom => {
                            ComboBox::from_label("Column Direction")
                                .selected_text(match column_dir {
                                    FlexDir::LeftToRight => "left to right",
                                    FlexDir::RightToLeft => "right to left",
                                    _ => {
                                        *column_dir = FlexDir::LeftToRight;
                                        "left to right"
                                    }
                                })
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(column_dir, FlexDir::LeftToRight, "left to right");
                                    ui.selectable_value(column_dir, FlexDir::RightToLeft, "right to left");
                                });
                            ComboBox::from_label("Row Alignment")
                                .selected_text(match alignment {
                                    Alignment::Top => "top",
                                    Alignment::Center => "center",
                                    Alignment::Bottom => "bottom",
                                    _ => {
                                        *alignment = Alignment::Top;
                                        "top"
                                    }
                                })
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(alignment, Alignment::Top, "top");
                                    ui.selectable_value(alignment, Alignment::Center, "center");
                                    ui.selectable_value(alignment, Alignment::Bottom, "bottom");
                                });
                        }
                    }
                    } else {
                    container.layout = Box::new(FixedGridLayout  { 
                        cells: UVec2 { x: 5, y: 5 }, 
                        row_dir: FlexDir::LeftToRight, 
                        column_dir: FlexDir::TopToBottom, 
                        alignment: Alignment::Left,
                    })
                }
                
            }
            "table" => {
                if let Some(DynamicTableLayout { columns, row_dir, column_dir, stretch }) = container.layout.downcast_mut() {
                    ui.label("Checkout another example for a demo on fixed columns.");
                    ui.add(Slider::new(columns, 1..=20).text("columns"));
                    ComboBox::from_label("Row Direction")
                        .selected_text(match row_dir {
                            FlexDir::LeftToRight => "left to right",
                            FlexDir::RightToLeft => "right to left",
                            FlexDir::BottomToTop => "bottom to top",
                            FlexDir::TopToBottom => "top to bottom",
                        })
                        .show_ui(ui, |ui| {
                            ui.selectable_value(row_dir, FlexDir::LeftToRight, "left to right");
                            ui.selectable_value(row_dir, FlexDir::RightToLeft, "right to left");
                            ui.selectable_value(row_dir, FlexDir::BottomToTop, "bottom to top");
                            ui.selectable_value(row_dir, FlexDir::TopToBottom, "top to bottom");
                        });
                    match row_dir {
                        FlexDir::LeftToRight|FlexDir::RightToLeft => {
                            ComboBox::from_label("Column Direction")
                                .selected_text(match column_dir {
                                    FlexDir::TopToBottom => "top to bottom",
                                    FlexDir::BottomToTop => "bottom to top",
                                    _ => {
                                        *column_dir = FlexDir::TopToBottom;
                                        "top to bottom"
                                    }
                                })
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(column_dir, FlexDir::TopToBottom, "top to bottom");
                                    ui.selectable_value(column_dir, FlexDir::BottomToTop, "bottom to top");
                                });
                        }
                        FlexDir::BottomToTop|FlexDir::TopToBottom => {
                            ComboBox::from_label("Column Direction")
                                .selected_text(match column_dir {
                                    FlexDir::LeftToRight => "left to right",
                                    FlexDir::RightToLeft => "right to left",
                                    _ => {
                                        *column_dir = FlexDir::LeftToRight;
                                        "left to right"
                                    }
                                })
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(column_dir, FlexDir::LeftToRight, "left to right");
                                    ui.selectable_value(column_dir, FlexDir::RightToLeft, "right to left");
                                });
                        }
                    }
                    ui.checkbox(stretch, "Stretch");
                } else {
                    container.layout = Box::new(DynamicTableLayout{ 
                        columns: 5, 
                        row_dir: FlexDir::LeftToRight, 
                        column_dir: FlexDir::TopToBottom, 
                        stretch: false, 
                    })
                }
            }
            _ => unreachable!()
        }
    
        let Vec2 { x, y } = container.margin.raw_mut();
        ui.add(Slider::new(x, 0.0..=50.0).text("margin x"));
        ui.add(Slider::new(y, 0.0..=50.0).text("margin y"));

        ui.label("Add Children with dimension:");
        let ChildSize(child_size) = child_size.as_mut();
        ui.add(Slider::new(&mut child_size.x, 0.0..=100.0).text("x"));
        ui.add(Slider::new(&mut child_size.y, 0.0..=100.0).text("y"));
        ui.label("and anchor:");
        Grid::new("add").show(ui, |ui| {
            if ui.button("TopLeft").clicked() {
                spawn(&mut commands, Anchor::TopLeft, *child_size, flexbox, &assets,)
            };
            if ui.button("TopCenter").clicked() {
                spawn(&mut commands, Anchor::TopCenter, *child_size, flexbox, &assets)
            };
            if ui.button("TopRight").clicked() {
                spawn(&mut commands, Anchor::TopRight, *child_size, flexbox, &assets)
            };
            ui.end_row();
            if ui.button("CenterLeft").clicked() {
                spawn(&mut commands, Anchor::CenterLeft, *child_size, flexbox, &assets)
            };
            if ui.button("Center").clicked() {
                spawn(&mut commands, Anchor::Center, *child_size, flexbox, &assets)
            };
            if ui.button("CenterRight").clicked() {
                spawn(&mut commands, Anchor::CenterRight, *child_size, flexbox, &assets)
            };
            ui.end_row();
            if ui.button("BottomLeft").clicked() {
                spawn(&mut commands, Anchor::BottomLeft, *child_size, flexbox, &assets)
            };
            if ui.button("BottomCenter").clicked() {
                spawn(&mut commands, Anchor::BottomCenter, *child_size, flexbox, &assets)
            };
            if ui.button("BottomRight").clicked() {
                spawn(&mut commands, Anchor::BottomRight, *child_size, flexbox, &assets)
            };
            ui.end_row();
        });

        if ui.button("Linebreak").clicked() {
            let child = commands.spawn(LinebreakBundle::new(*child_size)).id();
            commands.entity(flexbox).add_child(child);
        }

        if ui.button("Clear Children").clicked() {
            for entity in spawned.iter() {
                commands.entity(entity).despawn();
            }
        };
        
    });
}
