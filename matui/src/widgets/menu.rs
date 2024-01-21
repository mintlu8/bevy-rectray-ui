

use bevy::asset::Handle;
use bevy::ecs::component::Component;
use bevy::render::color::Color;
use bevy::render::texture::Image;
use bevy::{window::CursorIcon, hierarchy::BuildChildren};
use bevy::ecs::{entity::Entity, query::Changed, system::Query};
use bevy_aoui::dsl::prelude::sender;
use bevy_aoui::events::{EventFlags, FocusChange, FocusStateMachine};
use bevy_aoui::sync::TypedSignal;
use bevy_aoui::widgets::util::{DisplayIf, BlockPropagation};
use bevy_aoui::{Anchor, size2, Size2, markers, frame, transition, Opacity};
use bevy_aoui::util::{signal, ComposeExtension, Object};
use bevy_aoui::layout::StackLayout;
use bevy_aoui::widgets::button::{radio_button_group, Payload, RadioButton, ToggleChange};
use bevy_aoui::util::{Widget, AouiCommands, WidgetBuilder};

use crate::style::Palette;
use crate::widgets::states::ToggleOpacity;
use crate::{mdivider, mframe_extension, build_mframe, mcapsule, palette, mrectangle};


#[derive(Debug, Default, Component)]
pub struct DropdownItems(Vec<MenuItem>);

#[derive(Debug, Component)]
pub struct MenuBuilder {
    width: f32,
    radio: RadioButton,
    divider: WidgetBuilder<()>,
    text: WidgetBuilder<String>,
    icon: WidgetBuilder<Handle<Image>>,
    nested: Option<WidgetBuilder<Vec<MenuItem>>>,
    hover_background: Color,
    hover_capsule: bool,
}

#[doc(hidden)]
#[derive(Debug, Default)]
pub struct MenuItemText {
    pub key: Object,
    pub value: String,
    pub icon: Option<Handle<Image>>,
    pub right: Option<Handle<Image>>,
}

impl Into<MenuItem> for MenuItemText {
    fn into(self) -> MenuItem {
        MenuItem::Text { 
            key: self.key.or(self.value.clone()), 
            value: self.value, 
            icon: self.icon, 
            right: self.right 
        }
    }
}

#[doc(hidden)]
#[derive(Debug, Default)]
pub struct MenuItemNested {
    pub key: Object,
    pub value: String,
    pub icon: Option<Handle<Image>>,
    pub right: Option<Handle<Image>>,
    pub nested: Vec<MenuItem>
}

impl Into<MenuItem> for MenuItemNested {
    fn into(self) -> MenuItem {
        MenuItem::Nest { 
            key: self.key.or(self.value.clone()), 
            value: self.value, 
            icon: self.icon, 
            right: self.right,
            nest: self.nested
        }
    }
}

#[derive(Debug, Default, Clone)]
pub enum MenuItem {
    #[default]
    Divider,
    Text {
        key: Object,
        value: String,
        icon: Option<Handle<Image>>,
        right: Option<Handle<Image>>,
    },
    Nest {
        key: Object,
        value: String,
        icon: Option<Handle<Image>>,
        right: Option<Handle<Image>>,
        nest: Vec<MenuItem>,
    },
}

#[doc(hidden)]
#[macro_export]
macro_rules! menu_item {
    ($commands: tt |) => {
        $crate::widgets::MenuItem::Divider
    };
    ($commands: tt $name: literal) => {
        $crate::widgets::MenuItem::Text{
            key: $crate::aoui::util::Object::new($name.to_string()),
            value: $name.to_string(),
            icon: None,
            right: None,
        }
    };
    ($commands: tt ($key: expr, $value: expr)) => {
        $crate::widgets::MenuItem::Text{
            key: $bevy_aoui::dsl::parse($key),
            value: $value.to_string(),
            icon: None,
            right: None,
        }
    };
    ($commands: tt {$($field: ident: $value: expr),* $(,)?}) => {
        $crate::widgets::MenuItemText{
            $($field: $crate::aoui::dsl::parse($value),)*
            ..Default::default()
        }.into()
    };
    ($commands: tt $name: literal {$($nest:tt)*}) => {
        $crate::widgets::MenuItemNested{
            key: $crate::aoui::util::Object::new($name.to_string()),
            value: $name.to_string(),
            icon: None,
            right: None,
            nested: $crate::menu_items! ($commands {
                $($nest)*
            }),
        }.into()
    };
    ($commands: tt ($key: expr, $value: expr) {$($nest:tt)*}) => {
        $crate::widgets::MenuItemNested{
            key: $bevy_aoui::dsl::parse($key),
            value: $value.to_string(),
            icon: None,
            right: None,
            nested: $crate::menu_items! ($commands {
                $($nest)*
            }),
        }.into()
    };
    ($commands: tt {$($field: ident: $value: expr),* $(,)?} {$($nest:tt)*}) => {
        $crate::widgets::MenuItemNested{
            $($field: $crate::aoui::dsl::parse($value),)*
            nested: $crate::menu_items! ($commands {
                $($nest)*
            }),
            ..Default::default()
        }.into()
    };
    ($commands: tt $tt: tt) => {
        $tt
    }
}

#[macro_export]
macro_rules! menu_items {
    ($commands: tt {$($expr: tt $({$($nest:tt)*})?),* $(,)?}) => {
        vec![$($crate::menu_item!($commands $expr $({$($nest)*})?)),*]
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
                        child: icon.clone().map(|icon| commands.spawn_fn(&builder.icon, icon)),
                        child: right.clone().map(|icon| commands.spawn_fn(&builder.icon, icon)),
                        extra: MenuItemMarker,
                    });
                    if builder.hover_background.a() > 0.0 && builder.hover_capsule{
                        let background = mcapsule!(commands {
                            dimension: Size2::FULL,
                            palette: palette!(
                                background: grey300,
                            ),
                            extra: DisplayIf(EventFlags::Hover|EventFlags::LeftPressed),
                            extra: transition!(Opacity 0.15 Linear default 0.0),
                            z: 0.005,
                        });
                        commands.entity(item).add_child(background);
                    } else if builder.hover_background.a() > 0.0 {
                        let background = mrectangle!(commands {
                            dimension: Size2::FULL,
                            palette: palette!(
                                background: grey300,
                            ),
                            extra: DisplayIf(EventFlags::Hover|EventFlags::LeftPressed),
                            extra: transition!(Opacity 0.15 Linear default 0.0),
                            z: 0.005,
                        });
                        commands.entity(item).add_child(background);
                    }
                    commands.entity(entity).add_child(item);
                },
                MenuItem::Nest { key, value, icon, right, nest } => {
                    let (send, recv) = signal();
                    let item = bevy_aoui::radio_button!(commands {
                        dimension: size2!(width em, 2.2 em),
                        context: builder.radio.clone(),
                        value: key.clone(),
                        child: builder.text.build(&mut commands, value.clone()),
                        child: icon.clone().map(|icon| commands.spawn_fn(&builder.icon, icon)),
                        child: right.clone().map(|icon| commands.spawn_fn(&builder.icon, icon)),
                        extra: MenuItemMarker,
                        extra: FocusStateMachine::new(),
                        signal: sender::<FocusChange>(send),
                    });
                    let child = commands.spawn_fn(builder.nested.as_ref()
                        .expect("Expect menu builder."), 
                        nest.clone());
                    commands.entity(child)
                        .add_receiver::<ToggleChange>(recv)
                        .insert(ToggleOpacity::new(0.0, 1.0));
                    commands.entity(item).add_child(child);

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
                        let background = mrectangle!(commands {
                            dimension: Size2::FULL,
                            palette: palette!(
                                background: grey300,
                                stroke: grey300,
                            ),                            extra: DisplayIf(EventFlags::Hover|EventFlags::LeftPressed),
                            extra: transition!(Opacity 0.15 Linear default 0.0),
                            z: 0.005,
                        });
                        commands.entity(item).add_child(background);
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
        pub context: RadioButton,
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
        pub icon_builder: Option<WidgetBuilder<Handle<Image>>>,

        /// Widget builder for text dropdown.
        pub text_builder: Option<WidgetBuilder<String>>,
        pub nested_builder: Option<WidgetBuilder<Vec<MenuItem>>>,


        pub divider_width: Option<f32>,
        pub divider_inset: f32,

        pub hover_capsule: bool,

        /// Signal for building bools.
        pub open_signal: Option<TypedSignal<bool>>,
    }
);

impl Widget for MMenuBuilder {
    fn spawn(mut self, commands: &mut AouiCommands) -> (Entity, Entity) {
        let nested_builder = match self.nested_builder {
            Some(builder) => Some(builder),
            None => {
                if self.items.iter().any(|x| matches!(x, MenuItem::Nest { .. })) {
                    let mut cloned = self.clone();
                    cloned.opacity = Opacity::new(0.0);
                    cloned.anchor = Anchor::TOP_LEFT;
                    cloned.parent_anchor = Anchor::TOP_RIGHT.into();
                    Some(WidgetBuilder::new(
                        move |commands: &mut AouiCommands, items: Vec<MenuItem>| {
                            let mut cloned = cloned.clone();
                            cloned.items = items;
                            cloned.spawn(commands).0
                        }
                    ))
                } else {
                    None
                }
            },
        };

        let radio = radio_button_group(self.selected);
        let palette = self.palette;
        let hover_palette = self.hover_palette.unwrap_or(self.palette);
        if self.layout.is_none() {
            self.layout = Some(StackLayout::VSTACK.into());
        }
        let divider_width = self.divider_width.unwrap_or(0.1);
        let frame = build_mframe!(commands, self)
            .insert((
                BlockPropagation,
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
                    icon: self.icon_builder.unwrap_or_else(||
                        WidgetBuilder::new(move |commands: &mut AouiCommands, img: Handle<Image>| {
                            bevy_aoui::sprite!(commands {
                                anchor: Anchor::CENTER_RIGHT,
                                sprite: img,
                                offset: size2!(-2.2 em, 0),
                                dimension: size2!(2.2 em, 2.2 em),
                                color: hover_palette.stroke(),
                                z: 0.01,
                            })
                        })
                    ),
                    nested: nested_builder,
                },
            )).id();
        if let Some(signal) = self.open_signal {
            commands.entity(frame)
                .add_receiver::<ToggleChange>(signal)
                .insert(ToggleOpacity::new(0.0, 1.0));
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
