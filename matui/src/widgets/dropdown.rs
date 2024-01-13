

use bevy::ecs::component::Component;
use bevy::{window::CursorIcon, hierarchy::BuildChildren};
use bevy::ecs::{entity::Entity, query::Changed, system::Query};
use bevy_aoui::{Anchor, size2, frame};
use bevy_aoui::dsl::prelude::em;
use bevy_aoui::layout::BoundsLayout;
use bevy_aoui::signals::Object;
use bevy_aoui::{widget_extension, events::EventFlags, layout::StackLayout};
use bevy_aoui::widgets::button::{RadioButton, Payload, radio_button_group};
use bevy_aoui::dsl::{Widget, AouiCommands, WidgetBuilder, OptionEx};

use crate::{mframe, mdivider};

use super::{frame::FramePalette, util::ShadowInfo};

#[derive(Debug, Default, Component)]
pub struct DropdownItems(Vec<MenuItem>);

#[derive(Debug, Component)]
pub struct DropdownBuilder {
    width: Option<f32>,
    radio: RadioButton,
    divider: WidgetBuilder<()>,
    text: WidgetBuilder<String>,
}

#[derive(Debug, Default)]
pub enum MenuItem {
    #[default]
    Divider,
    Text {
        key: Object,
        value: String,
        left: Option<WidgetBuilder<()>>,
        right: Option<WidgetBuilder<()>>,
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
            left: None,
            right: None,
        }
    };
    (($key: expr, $value: expr)) => {
        $crate::widgets::MenuItem::Text{
            key: $bevy_aoui::dsl::parse($key),
            value: $value.to_string(),
            left: None,
            right: None,
        }
    };
    (($key: expr, $icon: expr, $value: expr)) => {
        $crate::widgets::MenuItem::Text{
            key: $bevy_aoui::dsl::parse($key),
            value: $value.to_string(),
            left: $bevy_aoui::dsl::parse(|commands| $crate::aoui::sprite!(commands {
                sprite: $icon,
                dimension: $crate::aoui::size2!(1.2 em, 1.2 em),
            })),
            right: None,
        }
    };
    (($key: expr, _, $value: expr, $right: expr)) => {
        $crate::widgets::MenuItem::Text{
            key: $bevy_aoui::dsl::parse($key),
            value: $value.to_string(),
            left: None,
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
    query: Query<(Entity, &DropdownItems, &DropdownBuilder), Changed<DropdownItems>>
) {
    for (entity, items, builder) in query.iter() {
        dbg!("Ahhh");
        commands.despawn_descendants(entity);
        let Some(width) = builder.width else {continue};
        for item in &items.0 {
            match item {
                MenuItem::Divider => {
                    let div = builder.divider.build(&mut commands, ());
                    commands.entity(entity).add_child(div);
                },
                MenuItem::Text { key, value, left, right } => {
                    let entity = frame!(commands {
                        dimension: size2!(width em, 2.2 em),
                    });
                    let text = bevy_aoui::radio_button!(commands {
                        context: builder.radio.clone(),
                        value: key.clone(),
                        layout: BoundsLayout::PADDING,
                        child: builder.text.build(&mut commands, value.clone())
                    });
                    commands.entity(entity).add_child(text);
                    if let Some(left) = left {
                        let icon = left.build(&mut commands, ());
                        commands.entity(entity).add_child(icon);
                    }
                    if let Some(right) = right {
                        let right = right.build(&mut commands, ());
                        commands.entity(entity).add_child(right);
                    }
                },
            }
        }
    }
}

widget_extension!(
    pub struct MMenuBuilder {
        /// Sets the CursorIcon when hovering this menu, default is `Hand`
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
        /// Palette
        pub palette: FramePalette,
        /// Palette for hovering.
        pub hover_palette: FramePalette,
        /// Shadow.
        pub shadow: OptionEx<ShadowInfo>,
        pub stroke: f32,
        pub radius: f32,
        pub width: Option<f32>,

        pub default_value: Object,

        /// Widget builder for text dropdown.
        pub text_builder: Option<WidgetBuilder<String>>,

        pub divider_width: Option<f32>,
        pub divider_inset: f32,
        /// Widget builder for text divider.
        pub dropdown_divider: Option<WidgetBuilder<()>>
    }
);

impl Widget for MMenuBuilder {
    fn spawn(self, commands: &mut AouiCommands) -> (Entity, Entity) {
        let radio = radio_button_group(self.selected);
        let frame = mframe!(commands {
            anchor: self.anchor,
            parent_anchor: self.parent_anchor,
            rotation: self.rotation,
            scale: self.scale,
            z: self.z,
            event: EventFlags::BlockAll,
            layout: StackLayout::VSTACK,
            margin: em(1),
            padding: em(0.25),
            radius: self.radius,
            shadow: self.shadow,
            extra: DropdownItems(self.items),
            extra: DropdownBuilder {
                width: self.width,
                radio,
                divider: WidgetBuilder::new(move |commands: &mut AouiCommands| {
                    bevy_aoui::padding!(commands {
                        padding: size2!(0, 0.3 em),
                        child: mdivider!{
                            color: self.palette.stroke,
                            width: self.divider_width,
                            inset: self.divider_inset,
                        }
                    })
                }),
                text: self.text_builder.unwrap_or_else(||
                    WidgetBuilder::new(move |commands: &mut AouiCommands, text: String| {
                        bevy_aoui::text!(commands {
                            anchor: Anchor::CENTER_LEFT,
                            text: text,
                            color: self.hover_palette.stroke,
                        })
                    })
                ),
            }
        });
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
