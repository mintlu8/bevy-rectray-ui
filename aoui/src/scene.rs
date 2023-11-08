use bevy::{prelude::{Component, Vec2, Vec3}, math::Affine2};


/// Treat `offset` as a position in a coordinate system.
#[derive(Component)]
pub struct SceneLayout(Box<dyn Fn(Vec2)-> Vec3 + Send + Sync>);

impl Default for SceneLayout {
    fn default() -> Self {
        Self(Box::new(|x| x.extend(0.0)))
    }
}

trait Placement: Clone + Send + Sync {
    fn call(&self, position: Vec2) -> Vec3;
}

impl<T> Placement for T where T: Fn(Vec2) -> Vec3 + Clone + Send + Sync {
    fn call(&self, position: Vec2) -> Vec3 {
        self(position)
    }
}

impl SceneLayout {

    pub fn transform(&self, v: Vec2) -> Vec3 {
        (self.0)(v)
    }

    pub fn squares(size: f32) -> Self{
        Self(Box::new(move |x| (x * size).extend(0.0)))
    }

    pub fn rectangles(x: f32, y: f32) -> Self {
        Self(Box::new(move |a| (a * Vec2::new(x, y)).extend(0.0)))
    }

    // Simple isometric layout
    pub fn isometric(x: f32, y: f32, z: f32) -> Self {
        let x = x / 2.0;
        let y = y / 2.0;
        Self(Box::new(move |a| Vec3::new(
            a.x * (-x + y),
            a.y * (x + y), 
            z * (a.x + a.y))
        ))
    }

    pub fn with_origin(self, origin: Vec2) -> Self {
        Self(Box::new(move |x| self.0(x - origin)))
    }

    pub fn with_transform(self, transform: Affine2) -> Self {
        Self(Box::new(move |x| self.0(transform.transform_vector2(x))))
    }
}