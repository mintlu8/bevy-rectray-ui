use bevy::{window::CursorIcon, ecs::{entity::Entity, component::Component, query::Changed, system::Query}, render::color::Color, hierarchy::BuildChildren, sprite::Mesh2dHandle, transform::components::GlobalTransform, asset::Handle, text::Font};
use bevy_aoui::{widget_extension, widgets::button::{RadioButton, Payload}, dsl::{OptionX, Widget, AouiCommands, mesh_rectangle}, signals::SignalBuilder, build_frame, events::EventFlags, layout::StackLayout, BuildMeshTransform, Anchor};

use crate::{shapes::RoundedRectangleMaterial, divider};

use super::{frame::FramePalette, util::{ShadowInfo, OptionM}};

#[derive(Debug, Clone, Default, Component)]
pub struct DropdownItems(Vec<String>);

#[derive(Debug, Clone, Component, Default)]
pub struct DropdownData {
    font: Handle<Font>,
    anchor: Anchor,
    divider: bool,
    divider_width: Option<f32>,
    divider_inset: f32,
    divider_color: Color,
}

pub fn rebuild_dropdown_children(
    mut commands: AouiCommands,
    query: Query<(Entity, &DropdownItems, &DropdownData), Changed<DropdownItems>>
) {
    for (entity, items, builder) in query.iter() {
        let mut first = true;
        commands.despawn_descendants(entity);
        for text in &items.0 {
            if !first && builder.divider {
                let div = divider!(commands{
                    width: builder.divider_width,
                    inset: builder.divider_inset,
                    color: builder.divider_color,
                });
                commands.entity(entity).add_child(div);
            }
            first = false;
            let text = bevy_aoui::text!(commands{
                text: text.clone(),
                font: builder.font.clone(),
                anchor: builder.anchor,
            });
            commands.entity(entity).add_child(text);
        }
    }
}

widget_extension!(
    pub struct MDropDownMenuBuilder {
        /// Sets the CursorIcon when hovering this menu, default is `Hand`
        pub cursor: Option<CursorIcon>,
        /// The context for the dropdown radio_button value.
        pub context: Option<RadioButton>,
        /// If true, behave like a `CheckButton` and set context to `None` if already checked.
        pub cancellable: bool,
        /// Discriminant for this button's value, must be comparable.
        pub value: OptionX<Payload>,
        /// Signal to open the dropdown menu.
        pub open_signal: Option<SignalBuilder<bool>>,
        /// Width of the divider.
        pub divider: Option<f32>,
        /// Selected
        pub selected: Option<String>,
        /// Items.
        pub items: Vec<String>,
        /// Palette
        pub palette: FramePalette,
        /// Palette for hovering.
        pub hover_palette: FramePalette,
        /// Shadow.
        pub shadow: OptionM<ShadowInfo>,
        pub stroke: f32,
        pub radius: f32,
    }
);

impl Widget for MDropDownMenuBuilder {
    fn spawn(mut self, commands: &mut AouiCommands) -> (Entity, Entity) {
        bevy_aoui::inject_events!(self.event, EventFlags::ClickOutside);
        if self.layout.is_none() {
            self.layout = Some(Box::new(StackLayout::VSTACK));
        }
        let mesh = commands.add(mesh_rectangle());
        let material = commands.add(
            RoundedRectangleMaterial::new(self.palette.background, self.radius)
                .with_stroke((self.palette.stroke, self.stroke)));
        let mut frame = build_frame!(commands, self);
        let frame = frame.insert((
            Mesh2dHandle(mesh),
            material,
            GlobalTransform::IDENTITY,
            BuildMeshTransform,
            DropdownItems(self.items),
        )).id();
        self.z += 0.01;
        self.event = Some(EventFlags::BlockAll);
        if let OptionM::Some(shadow) = self.shadow {
            let shadow = shadow.build_rect(commands, self.radius);
            commands.entity(frame).add_child(shadow);
        }
        (frame, frame)
    }
}
