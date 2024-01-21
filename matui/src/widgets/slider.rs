use std::ops::Range;

use bevy::ecs::component::Component;
use bevy::math::Vec2;
use bevy::{render::texture::Image, window::CursorIcon, ecs::entity::Entity};
use bevy_aoui::sync::TypedSignal;
use bevy_aoui::util::ComposeExtension;
use bevy_aoui::util::{Widget, AouiCommands, convert::{OptionEx, IntoAsset}};
use bevy_aoui::{frame_extension, build_frame, layout::Axis, Anchor};

use crate::shaders::RoundedRectangleMaterial;
use crate::style::Palette;
use crate::widgets::states::ButtonColors;

use super::util::ShadowInfo;

#[derive(Debug, Clone, Component)]
pub struct SliderRebase(TypedSignal<Vec2>);

pub trait SliderData {}


impl SliderData for i32 {

}

impl SliderData for f32 {

}

frame_extension!(
    pub struct MSliderBuilder[T: SliderData] {
        pub direction: Axis,
        /// Sets the CursorIcon when hovering this button, default is `Hand`
        pub cursor: Option<CursorIcon>,
        pub range: Range<T>,
        /// Sends a `bool` signal whenever the button is clicked.
        pub signal: TypedSignal<f32>,
        /// Sets whether the default value is checked or not.
        pub checked: bool,

        /// The length the dial travels in em, default is 1.25 em.
        pub length: Option<f32>,

        pub palette: Palette,
        pub hover_palette: Option<Palette>,
        pub drag_palette: Option<Palette>,
        pub disabled_palette: Option<Palette>,

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

        //let (fac_send, fac_recv) = signal();
        let rebase_send = signal();
        let (drag_send_root, drag_send_dial) = signal();


        let mut frame = build_frame!(commands, self);

        frame.add_sender::<Dragging>(drag_send_root)
            .insert((
                PropagateFocus,
                SliderRebase(rebase_send),
                // Handlers::<EvLeftDown>::new(Mutation::with_context(
                //     |res: Res<CursorState>| {res.down_position()},
                //     |down: Vec2, rect: &RotatedRect, fac: &mut SliderRebase| {
                //         let hdim = rect.half_dim();
                //         fac.0.send(rect.local_space(down) + Vec2::new(hdim.x - hdim.y, 0.0));
                //     }
                // )),
        ));

        let frame = frame.id();


        let thickness = self.thickness.unwrap_or(0.4);
        let background = frame!(commands {
            dimension: Size2::em(horiz_len + thickness, thickness),
            z: 0.01,
            extra: RoundedRectangleMaterial::capsule(palette.background())
                .with_stroke((palette.stroke(), self.background_stroke))
                .into_bundle(commands),
            child: frame!{
                anchor: Anchor::CENTER_LEFT,
                dimension: size2!(0%, thickness em),
                z: 0.01,
                extra: RoundedRectangleMaterial::capsule(palette.foreground())
                    .with_stroke((palette.stroke(), self.background_stroke))
                    .into_bundle(commands),
                // extra: fac_recv.recv(|fac: f32, dim: &mut Dimension| {
                //     dim.edit_raw(|v| v.x = fac);
                // }),
                extra: transition!(Color 0.2 CubicInOut default {palette.foreground()}),
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
                extra: DragX.with_constraints(),
                    // .with_recv(drag_recv)
                    // .with_handler(self.signal.and(fac_send)),
                // extra: rebase_recv.recv(|pos: Vec2, state: &mut Dragging|{
                //     state.drag_start.x = pos.x
                // }),
                child: frame! {
                    entity: dial,
                    dimension: Size2::em(dial_size, dial_size),
                    z: 0.01,
                    extra: RoundedRectangleMaterial::capsule(palette.foreground())
                        .with_stroke((palette.foreground_stroke(), self.dial_stroke))
                        .into_bundle(commands),
                    event: EventFlags::LeftDrag | EventFlags::Hover,
                    hitbox: Hitbox::ellipse(1),
                    extra: ButtonColors {
                        idle: palette.foreground(),
                        hover: hover_palette.foreground(),
                        pressed: drag_palette.foreground(),
                        disabled: disabled_palette.foreground(),
                    },
                    extra: transition!(Color 0.2 CubicInOut default {palette.foreground()}),
                    signal: sender::<Dragging>(drag_send_dial),
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
