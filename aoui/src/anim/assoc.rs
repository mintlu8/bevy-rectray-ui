use bevy::{ecs::{component::Component, query::WorldQuery, system::Query}, sprite::{TextureAtlasSprite, Sprite}, render::color::Color, text::Text};

use crate::{Transform2D, Dimension, Opacity};

use super::{Interpolation, Interpolate, Offset, Rotation, Scale, Index};


pub trait InterpolateAssociation {
    type Comp: Component;
    type Inter: Interpolation;

    fn set(component: &mut Self::Comp, value: <Self::Inter as Interpolation>::FrontEnd); 
    fn get(component: &Self::Comp) -> <Self::Inter as Interpolation>::FrontEnd; 

    fn system(mut query: Query<(&mut Self::Comp, &Interpolate<Self::Inter>)>) {
        query.par_iter_mut().for_each(|(mut comp, inter)| {
            Self::set(comp.as_mut(), inter.get())
        })
    }

}

impl InterpolateAssociation for (Transform2D, Offset) {
    type Comp = Transform2D;
    type Inter = Offset;

    fn set<'t>(component: &mut Self::Comp, value: <Self::Inter as Interpolation>::FrontEnd) {
        component.offset.edit_raw(|x| *x = value);
    }

    fn get(component: &Self::Comp) -> <Self::Inter as Interpolation>::FrontEnd {
        component.offset.raw()
    }
}

impl InterpolateAssociation for (Transform2D, Rotation) {
    type Comp = Transform2D;
    type Inter = Rotation;

    fn set<'t>(component: &mut Self::Comp, value: <Self::Inter as Interpolation>::FrontEnd) {
        component.rotation = value;
    }

    fn get(component: &Self::Comp) -> <Self::Inter as Interpolation>::FrontEnd {
        component.rotation
    }
}

impl InterpolateAssociation for (Transform2D, Scale) {
    type Comp = Transform2D;
    type Inter = Scale;

    fn set<'t>(component: &mut Self::Comp, value: <Self::Inter as Interpolation>::FrontEnd) {
        component.scale = value;
    }

    fn get(component: &Self::Comp) -> <Self::Inter as Interpolation>::FrontEnd {
        component.scale
    }
}

impl InterpolateAssociation for (Dimension, Dimension) {
    type Comp = Dimension;
    type Inter = Dimension;

    fn set<'t>(component: &mut Self::Comp, value: <Self::Inter as Interpolation>::FrontEnd) {
        component.edit_raw(|x| *x = value);
    }

    fn get(component: &Self::Comp) -> <Self::Inter as Interpolation>::FrontEnd {
        component.raw()
    }
}

impl InterpolateAssociation for (TextureAtlasSprite, Index) {
    type Comp = TextureAtlasSprite;
    type Inter = Index;

    fn set<'t>(component: &mut Self::Comp, value: <Self::Inter as Interpolation>::FrontEnd) {
        component.index = value
    }

    fn get(component: &Self::Comp) -> <Self::Inter as Interpolation>::FrontEnd {
        component.index
    }
}

impl InterpolateAssociation for (Opacity, Opacity) {
    type Comp = Opacity;
    type Inter = Opacity;

    fn set<'t>(component: &mut Self::Comp, value: <Self::Inter as Interpolation>::FrontEnd) {
        component.opacity = value
    }

    fn get(component: &Self::Comp) -> <Self::Inter as Interpolation>::FrontEnd {
        component.opacity
    }
}


impl InterpolateAssociation for (Sprite, Color) {
    type Comp = Sprite;
    type Inter = Color;

    fn set<'t>(component: &mut Self::Comp, value: <Self::Inter as Interpolation>::FrontEnd) {
        component.color = value
    }

    fn get(component: &Self::Comp) -> <Self::Inter as Interpolation>::FrontEnd {
        component.color
    }
}

impl InterpolateAssociation for (TextureAtlasSprite, Color) {
    type Comp = TextureAtlasSprite;
    type Inter = Color;

    fn set<'t>(component: &mut Self::Comp, value: <Self::Inter as Interpolation>::FrontEnd) {
        component.color = value
    }

    fn get(component: &Self::Comp) -> <Self::Inter as Interpolation>::FrontEnd {
        component.color
    }
}

impl InterpolateAssociation for (Text, Color) {
    type Comp = Text;
    type Inter = Color;

    fn set<'t>(component: &mut Self::Comp, value: <Self::Inter as Interpolation>::FrontEnd) {
        for section in &mut component.sections {
            section.style.color = value;
        }
    }

    fn get(component: &Self::Comp) -> <Self::Inter as Interpolation>::FrontEnd {
        component.sections.first().map(|x| x.style.color).unwrap_or(Color::NONE)
    }
}

#[derive(Debug, WorldQuery)]
#[world_query(mutable)]
pub struct MaybeAnim<A: Component, B: Interpolation> where (A, B): InterpolateAssociation {
    pub component: &'static mut A,
    pub interpolate: Option<&'static mut Interpolate<B>>,
}

impl<A: Component, B: Interpolation> MaybeAnimItem<'_, A, B> 
        where (A, B): InterpolateAssociation<Comp = A, Inter = B> {
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
