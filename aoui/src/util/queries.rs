use bevy::{ecs::{query::With, system::{Query, Res, SystemParam}}, math::Vec2, window::{PrimaryWindow, Window}};

use crate::AouiREM;


#[derive(SystemParam)]
pub struct ScalingFactor<'w, 's> {
    window: Query<'w, 's, &'static Window, With<PrimaryWindow>>,
}

impl ScalingFactor<'_, '_> {
    pub fn get(&self) -> f32 {
        self.window
            .get_single()
            .map(|x| x.scale_factor() as f32)
            .unwrap_or(2.0)
    }
}

#[derive(SystemParam)]
pub struct WindowSize<'w, 's> {
    window: Query<'w, 's, &'static Window, With<PrimaryWindow>>,
}

impl WindowSize<'_, '_> {
    pub fn get(&self) -> Vec2 {
        self.window
            .get_single()
            .map(|x| Vec2::new(x.width(), x.height()))
            .unwrap_or(Vec2::new(0.0, 0.0))
    }
}

#[derive(SystemParam)]
pub struct Rem<'w> {
    rem: Option<Res<'w, AouiREM>>,
}

impl Rem<'_> {
    pub fn get(&self) -> f32 {
        self.rem.as_ref()
            .map(|x| x.get())
            .unwrap_or(16.0)
    }
}