

use bevy::asset::Handle;
use bevy::ecs::component::Component;
use bevy::render::color::Color;
use bevy::render::texture::Image;
use bevy::{window::CursorIcon, hierarchy::BuildChildren};
use bevy::ecs::{entity::Entity, query::Changed, system::Query};
use bevy_aoui::anim::Interpolate;
use bevy_aoui::events::EventFlags;
use bevy_aoui::widgets::util::DisplayIf;
use bevy_aoui::{Anchor, size2, Size2, markers, frame, transition, Opacity, rectangle, color};
use bevy_aoui::signals::{Object, SignalBuilder};
use bevy_aoui::layout::StackLayout;
use bevy_aoui::widgets::button::{RadioButton, Payload, radio_button_group};
use bevy_aoui::util::{Widget, AouiCommands, WidgetBuilder};

use crate::style::Palette;
use crate::{mdivider, mframe_extension, build_mframe, mcapsule, palette};


#[derive(Debug, Default, Component)]
pub struct DropdownItems(Vec<MenuItem>);

#[derive(Debug, Component)]
pub struct MenuBuilder {
    width: f32,
    radio: RadioButton,
    divider: WidgetBuilder<()>,
    text: WidgetBuilder<String>,
    icon: Option<WidgetBuilder<Handle<Image>>>,
    right_icon: Option<WidgetBuilder<Handle<Image>>>,
    hover_background: Color,
    hover_capsule: bool,
}

#[derive(Debug, Default, Clone)]
pub enum MenuItem {
    #[default]
    Divider,
    Text {
        key: Object,
        value: String,
        icon: Option<Handle<Image>>,
        right: Option<WidgetBuilder<()>>,
    },
    Nest {
        key: Object,
        value: String,
        left: Option<WidgetBuilder<()>>,
        right: Option<WidgetBuilder<()>>,
        nest: WidgetBuilder<()>,
    },
}

#[doc(hidden)]
#[macro_export]
macro_rules! menu_item {
    (|) => {
        $crate::widgets::MenuItem::Divider
    };
    ($name: expr) => {
        $crate::widgets::MenuItem::Text{
            key: $crate::aoui::signals::Object::new($name.to_string()),
            value: $name.to_string(),
            icon: None,
            right: None,
        }
    };
    (($key: expr, $value: expr)) => {
        $crate::widgets::MenuItem::Text{
            key: $bevy_aoui::dsl::parse($key),
            value: $value.to_string(),
            icon: None,
            right: None,
        }
    };
    (($key: expr, $icon: expr, $value: expr)) => {
        $crate::widgets::MenuItem::Text{
            key: $bevy_aoui::dsl::parse($key),
            value: $value.to_string(),
            icon: $icon,
            right: None,
        }
    };
    (($key: expr, _, $value: expr, $right: expr)) => {
        $crate::widgets::MenuItem::Text{
            key: $bevy_aoui::dsl::parse($key),
            value: $value.to_string(),
            icon: None,
            right: $bevy_aoui::dsl::parse($right),
        }
    };
    (($key: expr, $icon: expr, $value: expr, $right: expr)) => {
        $crate::widgets::MenuItem::Text{
            key: $bevy_aoui::dsl::parse($key),
            value: $value.to_string(),
            left: $bevy_aoui::dsl::parse(|commands| $crate::aoui::sprite!(commands {
                sprite: $icon,
                dimension: $crate::aoui::size2!(1.2 em, 1.2 em),
            })),
            right: $bevy_aoui::dsl::parse($right),
        }
    };
    ($tt: tt) => {
        $tt
    }
}

#[macro_export]
macro_rules! menu_items {
    ($($expr: tt),* $(,)?) => {
        vec![$($crate::menu_item!($expr)),*]
    };
}

pub fn rebuild_dropdown_children(
    mut commands: AouiCommands,
    query: Query<(Entity, &DropdownItems, &MenuBuilder), Changed<DropdownItems>>
) {
    for (entity, items, builder) in query.iter() {
        markers!(MenuItemMarker);
        commands.despawn_children_with::<MenuItemMarker>(entity);
        let width = builder.width;
        for item in &items.0 {
            match item {
                MenuItem::Divider => {
                    let div = builder.divider.build(&mut commands, ());
                    commands.entity(div).insert(MenuItemMarker);
                    commands.entity(entity).add_child(div);
                },
                MenuItem::Text { key, value, icon, right } => {
                    let item = bevy_aoui::radio_button!(commands {
                        dimension: size2!(width em, 2.2 em),
                        context: builder.radio.clone(),
                        value: key.clone(),
                        child: builder.text.build(&mut commands, value.clone()),
                        child: builder.icon.as_ref().and_then(|x| Some(x.build(&mut commands, icon.clone()?))),
                        extra: MenuItemMarker,
                    });
                    if builder.hover_background.a() > 0.0 && builder.hover_capsule{
                        let background = mcapsule!(commands {
                            dimension: Size2::FULL,
                            palette: palette!(
                                background: grey300,
                                stroke: grey300,
                            ),
                            extra: DisplayIf(EventFlags::Hover|EventFlags::LeftPressed),
                            extra: transition!(Opacity 0.15 Linear default 0.0),
                            z: 0.005,
                        });
                        commands.entity(item).add_child(background);
                    } else if builder.hover_background.a() > 0.0 {
                        let background = rectangle!(commands {
                            dimension: Size2::FULL,
                            color: color!(grey300),
                            extra: DisplayIf(EventFlags::Hover|EventFlags::LeftPressed),
                            extra: transition!(Opacity 0.15 Linear default 0.0),
                            z: 0.005,
                        });
                        commands.entity(item).add_child(background);
                    }
                    if let Some(right) = right {
                        let right = right.build(&mut commands, ());
                        commands.entity(item).add_child(right);
                    }
                    commands.entity(entity).add_child(item);
                },
                MenuItem::Nest { key, value, left, right, nest:_ } => {
                    let item = bevy_aoui::radio_button!(commands {
                        dimension: size2!(width em, 2.2 em),
                        context: builder.radio.clone(),
                        value: key.clone(),
                        child: builder.text.build(&mut commands, value.clone()),
                        extra: MenuItemMarker,
                    });
                    if let Some(left) = left {
                        let icon = left.build(&mut commands, ());
                        commands.entity(item).add_child(icon);
                    }
                    if let Some(right) = right {
                        let right = right.build(&mut commands, ());
                        commands.entity(item).add_child(right);
                    }
                    commands.entity(entity).add_child(item);
                }
            }
        }
    }
}

mframe_extension!(
    #[derive(Clone)]
    pub struct MMenuBuilder {
        pub cursor: Option<CursorIcon>,
        /// The context for the dropdown radio_button value.
        pub context: Option<RadioButton>,
        /// If true, behave like a `CheckButton` and set context to `None` if already checked.
        pub cancellable: bool,
        /// Discriminant for this button's value, must be comparable.
        pub value: Option<Payload>,
        /// Width of the divider.
        pub divider: Option<f32>,
        /// Selected
        pub selected: Object,
        /// Items.
        pub items: Vec<MenuItem>,
        /// Palette for hovering.
        pub hover_palette: Option<Palette>,

        pub width: Option<f32>,

        pub default_value: Object,

        /// Widget builder for text dropdown.
        pub text_builder: Option<WidgetBuilder<String>>,

        pub divider_width: Option<f32>,
        pub divider_inset: f32,

        pub hover_capsule: bool,

        /// Signal for building bools.
        pub open_signal: Option<SignalBuilder<bool>>,
    }
);

impl Widget for MMenuBuilder {
    fn spawn(mut self, commands: &mut AouiCommands) -> (Entity, Entity) {
        let radio = radio_button_group(self.selected);
        let palette = self.palette;
        let hover_palette = self.hover_palette.unwrap_or(self.palette);
        if self.layout.is_none() {
            self.layout = Some(StackLayout::VSTACK.into());
        }
        let divider_width = self.divider_width.unwrap_or(0.1);
        let frame = build_mframe!(commands, self)
            .insert((
                transition!(Opacity 0.15 Linear default self.opacity.opacity),
                DropdownItems(self.items),
                MenuBuilder {
                    hover_background: Color::GRAY,
                    hover_capsule: self.hover_capsule,
                    width: self.width.unwrap_or(10.0),
                    radio,
                    divider: WidgetBuilder::new(move |commands: &mut AouiCommands| {
                        frame!(commands {
                            dimension: size2!(100%, {divider_width + 0.3} em),
                            child: mdivider!{
                                color: palette.stroke,
                                width: divider_width,
                                inset: self.divider_inset,
                                z: 0.01,
                            }
                        })
                    }),
                    text: self.text_builder.unwrap_or_else(||
                        WidgetBuilder::new(move |commands: &mut AouiCommands, text: String| {
                            bevy_aoui::text!(commands {
                                anchor: Anchor::CENTER_LEFT,
                                offset: size2!(2.2 em, 0),
                                text: text,
                                color: hover_palette.stroke(),
                                z: 0.01,
                            })
                        })
                    ),
                    icon: None,
                    right_icon: None,
                },
            )).id();
        if let Some(signal) = self.open_signal {
            commands.entity(frame).insert(
                signal.recv_filter(
                    |op: &mut Interpolate<Opacity>| op.interpolate_to(1.0),
                    |op: &mut Interpolate<Opacity>| op.interpolate_to(0.0),
                ),
            );
        }
        (frame, frame)
    }
}

#[macro_export]
macro_rules! mmenu {
    ($ctx: tt {$($tt: tt)*}) => {
        $crate::aoui::meta_dsl!($ctx [$crate::widgets::MMenuBuilder] {
            $($tt)*
        })
    };
}
