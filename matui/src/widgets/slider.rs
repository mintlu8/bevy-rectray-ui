use std::ops::Range;

use bevy::{render::texture::Image, window::CursorIcon, ecs::entity::Entity};
use bevy_aoui::dsl::{Widget, AouiCommands};
use bevy_aoui::{widget_extension, build_frame, material_sprite, layout::Axis, events::EvPositionFactor, Anchor, dsl::HandleOrString};
use bevy_aoui::events::Handlers;

use crate::shapes::RoundedRectangleMaterial;

use super::{util::{OptionM, ShadowInfo}, toggle::DialPalette};


pub trait SliderData {}


impl SliderData for i32 {
    
}

impl SliderData for f32 {
    
}

widget_extension!(
    pub struct MSliderBuilder[T: SliderData] {
        pub direction: Axis,
        /// Sets the CursorIcon when hovering this button, default is `Hand`
        pub cursor: Option<CursorIcon>,
        pub range: Range<T>,
        /// Sends a `bool` signal whenever the button is clicked.
        pub signal: Handlers<EvPositionFactor>,
        /// Sets whether the default value is checked or not.
        pub checked: bool,

        /// The length the dial travels in em, default is 1.25 em.
        pub length: Option<f32>,
        
        pub palette: DialPalette,
        pub hover_palette: Option<DialPalette>,
        pub drag_palette: Option<DialPalette>,

        pub thickness: Option<f32>,
        pub background_size: Option<f32>,
        pub background_texture: HandleOrString<Image>,
        pub background_stroke: f32,

        /// Size of the dial, default is 1.4 em.
        pub dial_size: Option<f32>,
        pub dial_texture: HandleOrString<Image>,
        pub dial_stroke: f32,

        /// Shadow for background.
        pub background_shadow: OptionM<ShadowInfo>,
        /// Shadow for the dial.
        pub dial_shadow: OptionM<ShadowInfo>,
    }
);

impl<T: SliderData> Widget for MSliderBuilder<T> {
    fn spawn(mut self, commands: &mut AouiCommands) -> (Entity, Entity) {
        use bevy_aoui::dsl::prelude::*;

        let palette = self.palette;
        let hover_palette = self.hover_palette.unwrap_or(palette);
        let drag_palette = self.drag_palette.unwrap_or(hover_palette);
        

        let horiz_len = self.length.unwrap_or(5.0);
        self.dimension = Some(Size2::em(2.0 + horiz_len, 2.0));
        if let Some(event) = &mut self.event {
            *event |= EventFlags::LeftClick | EventFlags::Hover;
        } else {
            self.event = Some(EventFlags::LeftClick | EventFlags::Hover);
        }
        let mut frame = build_frame!(commands, self);

        frame.insert((
            PropagateFocus,
        ));

        let frame = frame.id();

        let thickness = self.thickness.unwrap_or(0.4);
        let active_background: Entity;
        let background = material_sprite!(commands {
            dimension: Size2::em(horiz_len + thickness, thickness),
            z: 0.01,
            material: RoundedRectangleMaterial::capsule(palette.background)
                .with_stroke((palette.background_stroke, self.background_stroke)),
            child: material_sprite!{
                entity: active_background,
                anchor: Anchor::CenterLeft,
                dimension: Size2::em(horiz_len + thickness, thickness),
                z: 0.01,
                material: RoundedRectangleMaterial::capsule(palette.background)
                    .with_stroke((palette.background_stroke, self.background_stroke))
            }
        });
        if let OptionM::Some(shadow) = self.background_shadow {
            let shadow = shadow.build_capsule(commands);
            commands.entity(background).add_child(shadow);
        }
        commands.entity(frame).add_child(background);
        let dial_size = self.dial_size.unwrap_or(1.4);

        let (drag_send, drag_recv) = commands.signal();
        let dial;
        let core_slider = frame!(commands {
            dimension: size2!({horiz_len} em, 0.0),
            child: frame! {
                anchor: Left,
                dimension: [0, 0],
                extra: DragX.with_recv(drag_recv)
                    .with_handler(self.signal),
                child: material_sprite! {
                    entity: dial,
                    dimension: Size2::em(dial_size, dial_size),
                    z: 0.01,
                    material: RoundedRectangleMaterial::capsule(palette.dial)
                        .with_stroke((palette.dial_stroke, self.dial_stroke)),
                    event: EventFlags::LeftDrag | EventFlags::Hover,
                    hitbox: Ellipse(1),
                    extra: Handlers::<EvMouseDrag>::new(drag_send),
                    extra: SetCursor { 
                        flags: EventFlags::LeftDrag | EventFlags::Hover, 
                        icon: CursorIcon::Hand,
                    }
                }
            },
        });
        if let OptionM::Some(shadow) = self.background_shadow {
            let shadow = shadow.build_capsule(commands);
            commands.entity(background).add_child(shadow);
        }
        if let OptionM::Some(shadow) = self.dial_shadow {
            let shadow = shadow.build_capsule(commands);
            commands.entity(dial).add_child(shadow);
        }
        commands.entity(frame).add_child(background);
        commands.entity(frame).add_child(core_slider);
        (frame, frame)
    }
}


#[macro_export]
macro_rules! mslider {
    ($ctx: tt {$($tt: tt)*}) => {
        $crate::aoui::meta_dsl!($ctx [$crate::widgets::MSliderBuilder] {
            $($tt)*
        })
    };
}