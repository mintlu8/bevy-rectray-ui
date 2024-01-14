use bevy::asset::AssetLoader;
use bevy::{prelude::*, diagnostic::FrameTimeDiagnosticsPlugin};
use bevy_aoui::util::WorldExtension;
use bevy_aoui::AouiPlugin;
use bevy_aoui::util::AouiCommands;

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: bevy::window::PresentMode::AutoNoVsync,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .init_asset_loader::<AtlasImporter>()
        .add_systems(Startup, init)
        .add_plugins(AouiPlugin)
        .register_cursor_default(CursorIcon::Arrow)
        .run();
}

#[derive(serde::Deserialize)]
pub struct Atlas {
    pub file: String,
    pub size: [f32; 2],
    pub atlas: Vec<[f32; 4]>,
}

#[derive(Debug, Default)]
pub struct AtlasImporter;

impl AssetLoader for AtlasImporter {
    type Asset = TextureAtlas;

    type Settings = ();

    type Error = std::convert::Infallible;

    fn load<'a>(
        &'a self,
        reader: &'a mut bevy::asset::io::Reader,
        _: &'a Self::Settings,
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        use bevy::asset::AsyncReadExt;
        Box::pin(async {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await.unwrap();
            let atlas: Atlas = serde_json::from_slice(&bytes).unwrap();
            let mut result = TextureAtlas::new_empty(load_context.load(atlas.file), Vec2::new(atlas.size[0], atlas.size[1]));
            for rect in atlas.atlas {
                let [x, y, w, h] = rect;
                result.add_texture(Rect { min: Vec2 { x, y }, max: Vec2 { x: x + w, y: y + h } });
            }
            Ok(result)
        })
    }

    fn extensions(&self) -> &[&str] {
        &["json"]
    }
}


pub fn init(mut commands: AouiCommands) {
    use bevy_aoui::dsl::prelude::*;
    commands.spawn_bundle(Camera2dBundle::default());

    text!(commands {
        anchor: TopRight,
        text: "FPS: 0.00",
        color: color!(gold),
        extra: fps_signal(|fps: f32, text: &mut Text| {
            format_widget!(text, "FPS: {:.2}", fps);
        })
    });
    
    vstack!(commands  {
        child: atlas! {
            dimension: [128, 128],
            atlas: "ducky.json",
            padding: [1, 1],
            extra: transition!(
                Index 0.2 Linear repeat (2, 7)
            )
        },
        // different orientation from the corresponding bevy fn
        // maybe fix later.
        child: atlas! {
            dimension: [128, 128],
            atlas: [6, 4],
            sprites: "ducky.png",
            padding: [1, 1],
            extra: transition!(
                Index 0.2 Linear repeat (6, 11)
            )
        },
        child: atlas! {
            dimension: [128, 128],
            sprites: [
                "ducky/1.png",
                "ducky/2.png",
                "ducky/3.png",
                "ducky/4.png",
                "ducky/5.png",
                "ducky/6.png",
            ],
            padding: [1, 1],
            extra: transition!(
                Index 0.2 Linear repeat (0, 5)
            )
        },

        child: atlas! {
            dimension: [128, 128],
            atlas: [
                [  0, 33, 32, 31],
                [ 32, 33, 32, 31],
                [ 64, 33, 32, 31],
                [ 96, 33, 32, 31],
                [128, 33, 32, 31],
                [160, 33, 32, 31],
            ],
            sprites: commands.load("ducky.png"),
            padding: [1, 1],
            extra: transition!(
                Index 0.2 Linear repeat (0, 5)
            )
        },
    });
}
