use bevy::prelude::*;
use bevy_aoui::{AoUIPlugin, Transform2D};
use bevy_aoui_widgets::{AoUIExtensionsPlugin, events::CursorState};
use bevy_prototype_lyon::prelude::*;

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: bevy::window::PresentMode::AutoNoVsync,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_systems(Startup, init)
        .add_plugins(AoUIPlugin)
        .add_plugins(AoUIExtensionsPlugin)
        .add_plugins(ShapePlugin)
        .run();
}


pub fn init(mut commands: Commands, assets: Res<AssetServer>) {
    use bevy_aoui_widgets::dsl::prelude::*;
    commands.spawn(Camera2dBundle::default());
    marker!(Dragged);
    rectangle! ((commands, assets) {
        dimension: [100, 100],
        hitbox: Rect(1),
        fill: color!(lavender),
        extra: EventFlags::Hover|EventFlags::Drag,
        extra: Dragged,
        extra: handler!{LeftDrag => 
            fn handle_drag(mut query: Query<&mut Transform2D, With<Dragged>>, res: Res<CursorState>) {
                query.single_mut().offset.edit_raw(|x| *x = res.cursor_position())
            }
        },
        extra: SetCursor { 
            flags: EventFlags::Hover|EventFlags::Drag, 
            icon: CursorIcon::Hand,
        }
    });
}
