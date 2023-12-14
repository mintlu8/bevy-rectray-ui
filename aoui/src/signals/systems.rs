use bevy::{ecs::{system::Query, query::Without}, text::Text, log::warn, render::color::Color, sprite::{Sprite, TextureAtlasSprite}, math::Vec2};

use crate::{Transform2D, Dimension, Opacity, dsl::prelude::{Interpolate, Rotation, Offset, Scale}};

use super::{Receiver, types::*};

pub fn signal_receive_text(mut query: Query<(&Receiver<SigText>, &mut Text)>) {
    query.par_iter_mut().for_each(|(sig, mut text)| {
        let string = match sig.poll::<String>() {
            Some(string) => string,
            None => { 
                let Some(str) = sig.poll::<&str>() else {return};
                str.to_owned()
            },
        };
        let Some(section) = text.sections.first_mut() else { 
            // Since we don't have a style, writing it makes no sense.
            warn!("'SigText' received by a 'Text' component with no sections. Requires at least an empty section for styling.");
            return;
        }; 
        section.value = string;
    })
}

pub fn signal_receive_offset(mut query: Query<(&Receiver<SigOffset>, &mut Transform2D, Option<&mut Interpolate<Offset>>)>) {
    query.par_iter_mut().for_each(|(sig, mut transform, interpolate)| {
        let Some(offset) = sig.poll() else {return};
        if let Some(mut interpolate) = interpolate {
            interpolate.interpolate_to_or_reverse(offset);
        } else {
            transform.offset.edit_raw(|v| *v = offset)
        }
    })
}

pub fn signal_receive_offset_x(mut query: Query<(&Receiver<SigOffsetX>, &mut Transform2D, Option<&mut Interpolate<Offset>>)>) {
    query.par_iter_mut().for_each(|(sig, mut transform, interpolate)| {
        let Some(offset) = sig.poll() else {return};
        if let Some(mut interpolate) = interpolate {
            let y = interpolate.target().y;
            interpolate.interpolate_to_or_reverse(Vec2::new(offset, y));
        } else {
            transform.offset.edit_raw(|v| v.x = offset)
        }
    })
}

pub fn signal_receive_offset_y(mut query: Query<(&Receiver<SigOffsetY>, &mut Transform2D, Option<&mut Interpolate<Offset>>)>) {
    query.par_iter_mut().for_each(|(sig, mut transform, interpolate)| {
        let Some(offset) = sig.poll() else {return};
        if let Some(mut interpolate) = interpolate {
            let x = interpolate.target().x;
            interpolate.interpolate_to_or_reverse(Vec2::new(x, offset));
        } else {
            transform.offset.edit_raw(|v| v.y = offset)
        }
    })
}

pub fn signal_receive_rotation(mut query: Query<(&Receiver<SigRotation>, &mut Transform2D, Option<&mut Interpolate<Rotation>>)>) {
    query.par_iter_mut().for_each(|(sig, mut transform, interpolate)| {
        let Some(rotation) = sig.poll() else {return};
        if let Some(mut interpolate) = interpolate {
            interpolate.interpolate_to_or_reverse(rotation);
        } else {
            transform.rotation = rotation;
        }
    })
}

pub fn signal_receive_scale(mut query: Query<(&Receiver<SigScale>, &mut Transform2D, Option<&mut Interpolate<Scale>>)>) {
    query.par_iter_mut().for_each(|(sig, mut transform, interpolate)| {
        let Some(scale) = sig.poll() else {return};
        if let Some(mut interpolate) = interpolate {
            interpolate.interpolate_to_or_reverse(scale);
        } else {
            transform.scale = scale;
        }
    })
}

pub fn signal_receive_scale_x(mut query: Query<(&Receiver<SigScaleX>, &mut Transform2D, Option<&mut Interpolate<Scale>>)>) {
    query.par_iter_mut().for_each(|(sig, mut transform, interpolate)| {
        let Some(scale) = sig.poll() else {return};
        if let Some(mut interpolate) = interpolate {
            let y = interpolate.target().y;
            interpolate.interpolate_to_or_reverse(Vec2::new(scale, y));
        } else {
            transform.scale.x = scale;
        }
    })
}

pub fn signal_receive_scale_y(mut query: Query<(&Receiver<SigScaleY>, &mut Transform2D, Option<&mut Interpolate<Scale>>)>) {
    query.par_iter_mut().for_each(|(sig, mut transform, interpolate)| {
        let Some(scale) = sig.poll() else {return};
        if let Some(mut interpolate) = interpolate {
            let x = interpolate.target().x;
            interpolate.interpolate_to_or_reverse(Vec2::new(x, scale));
        } else {
            transform.scale.y = scale;
        }
    })
}

pub fn signal_receive_dimension(mut query: Query<(&Receiver<SigDimension>, &mut Dimension, Option<&mut Interpolate<Dimension>>)>) {
    query.par_iter_mut().for_each(|(sig, mut dimension, interpolate)| {
        let Some(dim) = sig.poll() else {return};
        if let Some(mut interpolate) = interpolate {
            interpolate.interpolate_to_or_reverse(dim);
        } else {
            dimension.edit_raw(|v| *v = dim);
        }
    })
}

pub fn signal_receive_dimension_x(mut query: Query<(&Receiver<SigDimensionX>, &mut Dimension, Option<&mut Interpolate<Dimension>>)>) {
    query.par_iter_mut().for_each(|(sig, mut dimension, interpolate)| {
        let Some(dim) = sig.poll() else {return};
        if let Some(mut interpolate) = interpolate {
            let y = interpolate.target().y;
            interpolate.interpolate_to_or_reverse(Vec2::new(dim, y));
        } else {
            dimension.edit_raw(|v| v.y = dim);
        }
    })
}

pub fn signal_receive_dimension_y(mut query: Query<(&Receiver<SigDimensionY>, &mut Dimension, Option<&mut Interpolate<Dimension>>)>) {
    query.par_iter_mut().for_each(|(sig, mut dimension, interpolate)| {
        let Some(dim) = sig.poll() else {return};
        if let Some(mut interpolate) = interpolate {
            let x = interpolate.target().x;
            interpolate.interpolate_to_or_reverse(Vec2::new(x, dim));
        } else {
            dimension.edit_raw(|v| v.y = dim);
        }
    })
}

pub fn signal_receive_opacity(mut query: Query<(&Receiver<SigOpacity>, &mut Opacity, Option<&mut Interpolate<Opacity>>)>) {
    query.par_iter_mut().for_each(|(sig, mut opacity, interpolate)| {
        let Some(op) = sig.poll() else {return};
        if let Some(mut interpolate) = interpolate {
            interpolate.interpolate_to_or_reverse(op)
        } else {
            opacity.opacity = op;
        }
    })
}

pub fn signal_receive_color_sprite(
    mut sprites: Query<(&Receiver<SigColor>, &mut Sprite), Without<Interpolate<Color>>>,
) {
    sprites.par_iter_mut().for_each(|(sig, mut sprite)| {
        let Some(color) = sig.poll() else {return};
        sprite.color = color;
    });
}

pub fn signal_receive_color_atlas(
    mut atlases: Query<(&Receiver<SigColor>, &mut TextureAtlasSprite), Without<Interpolate<Color>>>,
) {
    atlases.par_iter_mut().for_each(|(sig, mut sprite)| {
        let Some(color) = sig.poll() else {return};
        sprite.color = color;
    });
}

pub fn signal_receive_color_text(
    mut texts: Query<(&Receiver<SigColor>, &mut Text), Without<Interpolate<Color>>>,
) {
    texts.par_iter_mut().for_each(|(sig, mut text)| {
        let Some(color) = sig.poll() else {return};
        text.sections.iter_mut().for_each(|x| x.style.color = color);
    })
}

pub fn signal_receive_color_interpolate(
    mut texts: Query<(&Receiver<SigColor>, &mut Interpolate<Color>)>,
) {
    texts.par_iter_mut().for_each(|(sig, mut inter)| {
        let Some(color) = sig.poll::<Color>() else {return};
        inter.interpolate_to(color.into())
    })
}