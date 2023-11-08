use bevy::prelude::Update;
use events::{ MouseClick};


mod events;
mod input;


/// Core plugin for AoUI Rendering.
pub struct AoUIWidgetsPlugin;

impl bevy::prelude::Plugin for AoUIWidgetsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        
    }
}