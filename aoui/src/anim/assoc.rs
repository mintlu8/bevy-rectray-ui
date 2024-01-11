use bevy::{render::color::Color, text::Text, math::Vec2};
use bevy::ecs::{component::Component, system::Query};
use bevy::sprite::{TextureAtlasSprite, Sprite};
use bevy::ecs::query::{WorldQuery, ReadOnlyWorldQuery, Without};
use crate::{Transform2D, Dimension, Opacity, widgets::TextFragment};
use super::{Interpolation, Interpolate, Offset, Rotation, Scale, Index};


/// Associate a component with an interpolation.
pub trait InterpolateAssociation {
    type Component: Component;
    type Interpolation: Interpolation;
    type Condition: ReadOnlyWorldQuery;

    fn set(component: &mut Self::Component, value: <Self::Interpolation as Interpolation>::FrontEnd);
    fn get(component: &Self::Component) -> <Self::Interpolation as Interpolation>::FrontEnd;

    fn system(mut query: Query<(&mut Self::Component, &Interpolate<Self::Interpolation>), Self::Condition>) {
        query.iter_mut().for_each(|(mut comp, inter)| {
            if Self::get(comp.as_ref()) != inter.get() {
                Self::set(comp.as_mut(), inter.get())
            }
        })
    }

}

impl InterpolateAssociation for (Transform2D, Offset) {
    type Component = Transform2D;
    type Interpolation = Offset;
    type Condition = ();

    fn set<'t>(component: &mut Self::Component, value: <Self::Interpolation as Interpolation>::FrontEnd) {
        component.offset.edit_raw(|x| *x = value);
    }

    fn get(component: &Self::Component) -> <Self::Interpolation as Interpolation>::FrontEnd {
        component.offset.raw()
    }
}

impl InterpolateAssociation for (Transform2D, Rotation) {
    type Component = Transform2D;
    type Interpolation = Rotation;
    type Condition = ();

    fn set<'t>(component: &mut Self::Component, value: <Self::Interpolation as Interpolation>::FrontEnd) {
        component.rotation = value;
    }

    fn get(component: &Self::Component) -> <Self::Interpolation as Interpolation>::FrontEnd {
        component.rotation
    }
}

impl InterpolateAssociation for (Transform2D, Scale) {
    type Component = Transform2D;
    type Interpolation = Scale;
    type Condition = ();

    fn set<'t>(component: &mut Self::Component, value: <Self::Interpolation as Interpolation>::FrontEnd) {
        component.scale = value;
    }

    fn get(component: &Self::Component) -> <Self::Interpolation as Interpolation>::FrontEnd {
        component.scale
    }
}

impl InterpolateAssociation for (Dimension, Dimension) {
    type Component = Dimension;
    type Interpolation = Dimension;
    type Condition = ();

    fn set<'t>(component: &mut Self::Component, value: <Self::Interpolation as Interpolation>::FrontEnd) {
        component.edit_raw(|x| *x = value);
    }

    fn get(component: &Self::Component) -> <Self::Interpolation as Interpolation>::FrontEnd {
        match component.dimension {
            crate::DimensionType::Copied =>
                panic!("Cannot interpolate `copied` dimension."),
            crate::DimensionType::Dynamic =>
                panic!("Cannot interpolate `dynamic` dimension."),
            crate::DimensionType::Owned(v) => v.raw(),
        }
    }
}

impl InterpolateAssociation for (TextureAtlasSprite, Index) {
    type Component = TextureAtlasSprite;
    type Interpolation = Index;
    type Condition = ();

    fn set<'t>(component: &mut Self::Component, value: <Self::Interpolation as Interpolation>::FrontEnd) {
        component.index = value
    }

    fn get(component: &Self::Component) -> <Self::Interpolation as Interpolation>::FrontEnd {
        component.index
    }
}

impl InterpolateAssociation for (Opacity, Opacity) {
    type Component = Opacity;
    type Interpolation = Opacity;
    type Condition = ();

    fn set<'t>(component: &mut Self::Component, value: <Self::Interpolation as Interpolation>::FrontEnd) {
        component.opacity = value
    }

    fn get(component: &Self::Component) -> <Self::Interpolation as Interpolation>::FrontEnd {
        component.opacity
    }
}

impl InterpolateAssociation for (Opacity, Color) {
    type Component = Opacity;
    type Interpolation = Color;
    type Condition = ();

    fn set<'t>(component: &mut Self::Component, value: <Self::Interpolation as Interpolation>::FrontEnd) {
        component.opacity = value.a()
    }

    fn get(component: &Self::Component) -> <Self::Interpolation as Interpolation>::FrontEnd {
        let o = component.opacity;
        Color::rgba_linear(o, o, o, o)
    }
}


impl InterpolateAssociation for (Sprite, Color) {
    type Component = Sprite;
    type Interpolation = Color;
    type Condition = Without<TextFragment>;

    fn set<'t>(component: &mut Self::Component, value: <Self::Interpolation as Interpolation>::FrontEnd) {
        component.color = value
    }

    fn get(component: &Self::Component) -> <Self::Interpolation as Interpolation>::FrontEnd {
        component.color
    }
}

impl InterpolateAssociation for (TextureAtlasSprite, Color) {
    type Component = TextureAtlasSprite;
    type Interpolation = Color;
    type Condition = Without<TextFragment>;

    fn set<'t>(component: &mut Self::Component, value: <Self::Interpolation as Interpolation>::FrontEnd) {
        component.color = value
    }

    fn get(component: &Self::Component) -> <Self::Interpolation as Interpolation>::FrontEnd {
        component.color
    }
}

impl InterpolateAssociation for (Text, Color) {
    type Component = Text;
    type Interpolation = Color;
    type Condition = Without<TextFragment>;

    fn set<'t>(component: &mut Self::Component, value: <Self::Interpolation as Interpolation>::FrontEnd) {
        for section in &mut component.sections {
            section.style.color = value;
        }
    }

    fn get(component: &Self::Component) -> <Self::Interpolation as Interpolation>::FrontEnd {
        component.sections.first().map(|x| x.style.color).unwrap_or(Color::NONE)
    }
}

impl InterpolateAssociation for (TextFragment, Color) {
    type Component = TextFragment;
    type Interpolation = Color;
    type Condition = ();

    fn set<'t>(component: &mut Self::Component, value: <Self::Interpolation as Interpolation>::FrontEnd) {
        component.color = value;
    }

    fn get(component: &Self::Component) -> <Self::Interpolation as Interpolation>::FrontEnd {
        component.color
    }
}

/// Query for either setting a field or setting its associated interpolation.
#[derive(Debug, WorldQuery)]
#[world_query(mutable)]
pub struct Attr<A: Component, B: Interpolation> where (A, B): InterpolateAssociation<Component = A, Interpolation = B> {
    pub component: &'static mut A,
    pub interpolate: Option<&'static mut Interpolate<B>>,
}

impl<A: Component, B: Interpolation> AttrItem<'_, A, B>
        where (A, B): InterpolateAssociation<Component = A, Interpolation = B> {

    /// Set the value or move the interpolation.
    pub fn set(&mut self, value: B::FrontEnd) {
        if let Some(interpolate) = &mut self.interpolate {
            interpolate.interpolate_to(value);
        } else {
            <(A, B)>::set(&mut self.component, value);
        }
    }

    /// This will move the interpolation without interpolating.
    pub fn force_set(&mut self, value: B::FrontEnd) {
        if let Some(interpolate) = &mut self.interpolate {
            interpolate.set(value);
        }
        <(A, B)>::set(&mut self.component, value);
    }

    pub fn get(&self) -> B::FrontEnd {
        if let Some(interpolate) = &self.interpolate {
            interpolate.get()
        } else {
            <(A, B)>::get(&self.component)
        }
    }

    pub fn take(&mut self) -> B::FrontEnd {
        if let Some(interpolate) = &mut self.interpolate {
            interpolate.take_target()
        } else {
            <(A, B)>::get(&self.component)
        }
    }
}


impl<A: Component, B: Interpolation> AttrReadOnlyItem<'_, A, B>
        where (A, B): InterpolateAssociation<Component = A, Interpolation = B> {

    pub fn get(&self) -> B::FrontEnd {
        if let Some(interpolate) = &self.interpolate {
            interpolate.get()
        } else {
            <(A, B)>::get(self.component)
        }
    }
}


impl AttrItem<'_, Transform2D, Offset> {
    pub fn get_pixels(&self, parent: Vec2, em: f32, rem: f32) -> Vec2 {
        if let Some(interpolate) = &self.interpolate {
            interpolate.get()
        } else {
            self.component.offset.as_pixels(parent, em, rem)
        }
    }

    pub fn force_set_pixels(&mut self, value: Vec2) {
        if let Some(interpolate) = &mut self.interpolate {
            interpolate.set(value);
        }
        self.component.offset = value.into()
    }
}

impl AttrReadOnlyItem<'_, Transform2D, Offset> {
    pub fn get_pixels(&self, parent: Vec2, em: f32, rem: f32) -> Vec2 {
        if let Some(interpolate) = &self.interpolate {
            interpolate.get()
        } else {
            self.component.offset.as_pixels(parent, em, rem)
        }
    }
}
