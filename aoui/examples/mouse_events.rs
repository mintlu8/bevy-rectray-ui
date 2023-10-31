use bevy_aoui::*;
use bevy::{prelude::*, window::PrimaryWindow, sprite::Anchor};

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: bevy::window::PresentMode::AutoNoVsync,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins(AoUIPlugin)
        .add_event::<OnMouseHover>()
        .add_event::<OnMouseHoverWithEntity>()
        .add_event::<OnMouseClick>()
        .add_event::<OnMouseClickWithEntity>()
        .add_systems(Startup, init)
        .add_systems(Update, mouse_events)
        .add_systems(Update, EventPipe::<OnMouseHover>::system_ok_only.after(mouse_events)
            .before(on_hover))
        .add_systems(Update, EventPipe::<OnMouseClick>::system_ok_only.after(mouse_events)
            .before(on_click))
        .add_systems(Update, on_click)
        .add_systems(Update, on_hover)
        .run();
}

macro_rules! add {
    ($commands: expr, $assets: expr, $anchor: ident) => {
        {
            $commands.spawn((AoUISpriteBundle {
                sprite: Sprite { 
                    anchor: Anchor::$anchor,
                    custom_size: Some(Vec2::new(200.0, 200.0)),
                    color: Color::BLUE,
                    ..Default::default()
                },
                texture: $assets.load("square.png"),
                ..Default::default()
            },
            ))
        }
    };
}

pub fn init(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    add!(commands, assets, BottomLeft);
    add!(commands, assets, CenterLeft);
    add!(commands, assets, TopLeft);
    add!(commands, assets, TopCenter);
    add!(commands, assets, BottomCenter);
    add!(commands, assets, TopRight);
    add!(commands, assets, CenterRight);
    add!(commands, assets, BottomRight);
    add!(commands, assets, Center);
}

#[derive(Debug, Event, Clone)]
pub struct OnMouseHover(Vec2);

#[derive(Debug, Event, Clone)]
pub struct OnMouseHoverWithEntity(Vec2, Entity);


#[derive(Debug, Event, Clone)]
pub struct OnMouseClick(Vec2);

#[derive(Debug, Event, Clone)]
pub struct OnMouseClickWithEntity(Vec2, Entity);

impl CursorEvent for OnMouseHover {
    type FlagTy = u32;
    const FLAG: Self::FlagTy = 0;
    type WithEntity = OnMouseHoverWithEntity;
    type WithoutEntity = ();

    fn position(&self) -> Vec2 {
        self.0
    }

    fn with_entity(&self, entity: Entity) -> Self::WithEntity {
        OnMouseHoverWithEntity(self.0, entity)
    }

    fn without_entity(&self) -> Self::WithoutEntity {
        ()
    }
}

impl CursorEvent for OnMouseClick {
    type FlagTy = u32;
    const FLAG: Self::FlagTy = 0;
    type WithEntity = OnMouseClickWithEntity;
    type WithoutEntity = ();

    fn position(&self) -> Vec2 {
        self.0
    }

    fn with_entity(&self, entity: Entity) -> Self::WithEntity {
        OnMouseClickWithEntity(self.0, entity)
    }

    fn without_entity(&self) -> Self::WithoutEntity {
        ()
    }
}

pub fn mouse_events(
    windows: Query<&Window, With<PrimaryWindow>>,     
    camera: Query<(&Camera, &GlobalTransform)>,

    buttons: Res<Input<MouseButton>>,
    mut write: EventWriter<OnMouseHover>,
    mut write2: EventWriter<OnMouseClick>
) {
    let (camera, camera_transform) = camera.single();
    let mouse_pos = match windows.single().cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate()){
        Some(p) => p,
        None => return,
    };

    if !buttons.pressed(MouseButton::Left) {
        write.send(OnMouseHover(mouse_pos))
    }
    if buttons.just_pressed(MouseButton::Left) {
        write2.send(OnMouseClick(mouse_pos))
    }

}

pub fn on_hover(
        mut windows: Query<&mut Window, With<PrimaryWindow>>, 
        mut write: EventReader<OnMouseHoverWithEntity>
) {
    let mut window = windows.single_mut();
    for OnMouseHoverWithEntity(..) in write.into_iter() {
        window.cursor.icon = CursorIcon::Hand;
        return;
    }
    window.cursor.icon = CursorIcon::Arrow;
    
}

pub fn on_click(
    mut read: EventReader<OnMouseClickWithEntity>,
    mut sprites: Query<&mut Sprite>
) {
    for OnMouseClickWithEntity(_, entity) in read.into_iter() {
        match sprites.get_mut(*entity){
            Ok(mut sp) => sp.color = Color::RED,
            _ => (),
        }
    }
}
