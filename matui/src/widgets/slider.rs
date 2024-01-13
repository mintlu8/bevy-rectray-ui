use std::ops::Range;

use bevy::ecs::component::Component;
use bevy::ecs::system::Res;
use bevy::math::Vec2;
use bevy::{render::texture::Image, window::CursorIcon, ecs::entity::Entity};
use bevy_aoui::signals::SignalSender;
use bevy_aoui::widgets::drag::Dragging;
use bevy_aoui::RotatedRect;
use bevy_aoui::dsl::{Widget, AouiCommands, OptionEx, IntoAsset};
use bevy_aoui::{widget_extension, build_frame, material_sprite, layout::Axis, events::EvPositionFactor, Anchor};
use bevy_aoui::events::{Handlers, CursorState};

use crate::shapes::RoundedRectangleMaterial;
use crate::widgets::button::CursorStateColors;

use super::{util::ShadowInfo, toggle::DialPalette};

#[derive(Debug, Clone, Component)]
pub struct SliderRebase(SignalSender<Vec2>);

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
        pub disabled_palette: Option<DialPalette>,

        pub thickness: Option<f32>,
        pub background_size: Option<f32>,
        pub background_texture: IntoAsset<Image>,
        pub background_stroke: f32,

        /// Size of the dial, default is 1.4 em.
        pub dial_size: Option<f32>,
        pub dial_texture: IntoAsset<Image>,
        pub dial_stroke: f32,

        /// Shadow for background.
        pub background_shadow: OptionEx<ShadowInfo>,
        /// Shadow for the dial.
        pub dial_shadow: OptionEx<ShadowInfo>,
    }
);

impl<T: SliderData> Widget for MSliderBuilder<T> {
    fn spawn(mut self, commands: &mut AouiCommands) -> (Entity, Entity) {
        use bevy_aoui::dsl::prelude::*;

        let palette = self.palette;
        let hover_palette = self.hover_palette.unwrap_or(palette);
        let drag_palette = self.drag_palette.unwrap_or(hover_palette);
        let disabled_palette = self.disabled_palette.unwrap_or(palette);

        let horiz_len = self.length.unwrap_or(5.0);
        self.dimension = Size2::em(2.0 + horiz_len, 2.0).dinto();
        self.event |= EventFlags::Hover | EventFlags::LeftDrag;

        let (fac_send, fac_recv) = commands.signal();
        let (rebase_send, rebase_recv) = commands.signal();
        let (drag_send_root, drag_send_dial, drag_recv) = commands.signal();


        let mut frame = build_frame!(commands, self);

        frame.insert((
            PropagateFocus,
            SliderRebase(rebase_send.send()),
            Handlers::<EvMouseDrag>::new(drag_send_root),
            Handlers::<EvLeftDown>::new(Mutation::with_context(
                |res: Res<CursorState>| {res.down_position()},
                |down: Vec2, rect: &RotatedRect, fac: &mut SliderRebase| {
                    let hdim = rect.half_dim();
                    fac.0.send(rect.local_space(down) + Vec2::new(hdim.x - hdim.y, 0.0));
                }
            )),
        ));

        let frame = frame.id();


        let thickness = self.thickness.unwrap_or(0.4);
        let background = material_sprite!(commands {
            dimension: Size2::em(horiz_len + thickness, thickness),
            z: 0.01,
            material: RoundedRectangleMaterial::capsule(palette.background)
                .with_stroke((palette.background_stroke, self.background_stroke)),
            child: material_sprite!{
                anchor: Anchor::CENTER_LEFT,
                dimension: size2!(0%, thickness em),
                z: 0.01,
                material: RoundedRectangleMaterial::capsule(palette.dial)
                    .with_stroke((palette.background_stroke, self.background_stroke)),
                extra: fac_recv.recv(|fac: f32, dim: &mut Dimension| {
                    dim.edit_raw(|v| v.x = fac);
                }),
                extra: transition!(Color 0.2 CubicInOut default {palette.dial}),
            }
        });
        if let OptionEx::Some(shadow) = self.background_shadow {
            let shadow = shadow.build_capsule(commands);
            commands.entity(background).add_child(shadow);
        }
        commands.entity(frame).add_child(background);
        let dial_size = self.dial_size.unwrap_or(1.4);

        let dial;
        let core_slider = frame!(commands {
            dimension: size2!({horiz_len} em, 0.0),
            child: frame! {
                anchor: Left,
                dimension: [0, 0],
                extra: DragX.with_recv(drag_recv)
                    .with_handler(self.signal.and(fac_send)),
                extra: rebase_recv.recv(|pos: Vec2, state: &mut Dragging|{
                    state.drag_start.x = pos.x
                }),
                child: material_sprite! {
                    entity: dial,
                    dimension: Size2::em(dial_size, dial_size),
                    z: 0.01,
                    material: RoundedRectangleMaterial::capsule(palette.dial)
                        .with_stroke((palette.dial_stroke, self.dial_stroke)),
                    event: EventFlags::LeftDrag | EventFlags::Hover,
                    hitbox: Hitbox::ellipse(1),
                    extra: CursorStateColors {
                        idle: palette.dial,
                        hover: hover_palette.dial,
                        pressed: drag_palette.dial,
                        disabled: disabled_palette.dial,
                    },
                    extra: transition!(Color 0.2 CubicInOut default {palette.dial}),
                    extra: Handlers::<EvMouseDrag>::new(drag_send_dial),
                    extra: SetCursor {
                        flags: EventFlags::LeftDrag | EventFlags::Hover | EventFlags::LeftDown,
                        icon: CursorIcon::Hand,
                    }
                }
            },
        });
        if let OptionEx::Some(shadow) = self.background_shadow {
            let shadow = shadow.build_capsule(commands);
            commands.entity(background).add_child(shadow);
        }
        if let OptionEx::Some(shadow) = self.dial_shadow {
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
