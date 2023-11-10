use bevy_aoui::*;
use bevy::{prelude::*, window::PrimaryWindow, sprite::Anchor, ecs::system::SystemId};

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(AoUIPlugin)
        .add_plugins(Hover::run_at(Update))
        .add_plugins(Click::run_at(Update))
        .add_systems(Startup, init.pipe(init2))
        .add_systems(PreUpdate, mouse_events)
        .run();
}

pub fn init(world: &mut World) -> (SystemId, SystemId){
    let hover = world.register_system(on_hover);
    let click = world.register_system(on_click);
    (hover, click)
}

pub fn init2(input: In<(SystemId, SystemId)>, mut commands: Commands) {
    let (hover, click) = input.0;
    commands.spawn(Camera2dBundle::default());
    commands.spawn((AoUISpriteBundle {
            sprite: Sprite { 
                anchor: Anchor::Center,
                custom_size: Some(Vec2::new(200.0, 200.0)),
                color: Color::BLUE,
                ..Default::default()
            },
            ..Default::default()
        },
        Hitbox::FULL,
        Hover::handler(hover),
        Click::handler(click)
    ));
}


pub fn mouse_events(
    mut windows: Query<&mut Window, With<PrimaryWindow>>, 
    camera: Query<(&Camera, &GlobalTransform)>,
    input: Res<Input<MouseButton>>, 
    mut hover: EventWriter<Hover>, mut click: EventWriter<Click>) {
    
    let (camera, camera_transform) = camera.single();        
    let mouse_pos = match windows.single().cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate()){
        Some(p) => p,
        None => return,
    };    
    windows.single_mut().cursor.icon = CursorIcon::Arrow;
    if input.just_pressed(MouseButton::Left){
        click.send(Click(mouse_pos))
    } 
    hover.send(Hover(mouse_pos))
}

#[derive(Debug, Event)]
pub struct Hover(Vec2);

impl CursorEvent for Hover {
    type Points = [Vec2; 1];

    fn positions(&self) -> Self::Points {
        [self.0]
    }
}

#[derive(Debug, Event)]
pub struct Click(Vec2);

impl CursorEvent for Click {
    type Points = [Vec2; 1];

    fn positions(&self) -> Self::Points {
        [self.0]
    }
}


pub fn on_hover(mut windows: Query<&mut Window, With<PrimaryWindow>>) {
    let mut window = windows.single_mut();
    window.cursor.icon = CursorIcon::Hand;
}

pub fn on_click(mut sprites: Query<&mut Sprite>) {
    for mut sp in sprites.iter_mut() {
        sp.color = Color::RED
    }
}
