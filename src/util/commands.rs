use std::borrow::Borrow;
use std::marker::PhantomData;

use bevy::ecs::{entity::Entity, bundle::Bundle, component::Component};
use bevy::ecs::system::{Command, Commands, EntityCommands, Res, Resource, SystemParam};
use bevy::hierarchy::{Children, DespawnRecursive, BuildChildren, DespawnRecursiveExt};
use bevy::render::texture::{Image, BevyDefault};
use bevy::render::render_resource::{TextureDescriptor, Extent3d, TextureDimension, TextureUsages};
use bevy::asset::{AssetServer, Asset, Handle, AssetPath};
use bevy::utils::HashMap;
use bevy_defer::{AsObject, Object, signals::{SignalData, TypedSignal}};
use parking_lot::RwLock;
use crate::util::{CloneSplit, Widget};
use crate::widgets::button::RadioButton;

use super::WidgetBuilder;


/// Storage for named signals.
#[derive(Debug, Resource, Default)]
pub struct SignalPool(pub(crate) RwLock<HashMap<String, bevy_defer::Arc<SignalData<Object>>>>);

/// [`SystemParam`] combination of [`Commands`], [`AssetServer`] and [`SignalPool`].
#[derive(SystemParam)]
pub struct RCommands<'w, 's> {
    commands: Commands<'w, 's>,
    asset_server: Res<'w, AssetServer>,
    signals: Res<'w, SignalPool>,
}

/// Created a unnamed signal.
pub fn signal<T: AsObject, S: CloneSplit<TypedSignal<T>>>() -> S {
    CloneSplit::clone_split(TypedSignal::default())
}

impl<'w, 's> RCommands<'w, 's> {
    /// Obtain the underlying [`Commands`].
    pub fn commands(&mut self) -> &mut Commands<'w, 's> {
        &mut self.commands
    }

    /// Obtain an [`EntityCommands`].
    pub fn entity(&mut self, entity: Entity) -> EntityCommands {
        self.commands.entity(entity)
    }

    /// Obtain the underlying [`AssetServer`].
    pub fn assets(&self) -> &AssetServer {
        &self.asset_server
    }

    /// Add an [`Asset`].
    pub fn add_asset<T: Asset>(&self, item: T) -> Handle<T> {
        self.assets().add(item)
    }

    /// Add a [`Command`].
    pub fn add_command<T: Command>(&mut self, command: T) {
        self.commands().add(command)
    }

    /// Load an [`Asset`] from an asset path.
    pub fn load<'a, T: Asset>(&self, name: impl Into<AssetPath<'a>>) -> Handle<T> {
        self.assets().load(name)
    }

    /// Spawn a bundle.
    pub fn spawn_bundle(&mut self, bundle: impl Bundle) -> EntityCommands{
        self.commands.spawn(bundle)
    }

    /// Create a sprite as a render target.
    pub fn render_target<T: CloneSplit<Handle<Image>>>(&self, [width, height]: [u32; 2]) -> T{
        let handle = self.asset_server.add(Image {
            texture_descriptor: TextureDescriptor {
                label: None,
                size: Extent3d {
                    width,
                    height,
                    ..Default::default()
                },
                dimension: TextureDimension::D2,
                format: BevyDefault::bevy_default(),
                mip_level_count: 1,
                sample_count: 1,
                usage: TextureUsages::TEXTURE_BINDING
                    | TextureUsages::COPY_DST
                    | TextureUsages::RENDER_ATTACHMENT,
                view_formats: &[],
            },
            data: vec![0; width as usize * height as usize * 4],
            ..Default::default()
        });
        CloneSplit::clone_split(handle)
    }

    /// Spawn a `Widget` without passing in an `AssetServer`, this may panic.
    pub fn spawn_widget(&mut self, widget: impl Widget, extras: impl Bundle, children: impl AsRef<[Entity]>) -> Entity {
        let (id, container) = widget.spawn(self);
        self.entity(container).push_children(children.as_ref());
        self.entity(id)
            .insert(extras);
        id
    }

    pub fn spawn_dynamic(&mut self, widget: &WidgetBuilder<()>) -> Entity{
        widget.build(self, ())
    }

    pub fn spawn_fn<T>(&mut self, widget: &WidgetBuilder<T>, arg: T) -> Entity{
        widget.build(self, arg)
    }


    /// Created a tracked radio button group.
    pub fn radio_button_group<T: AsObject, S: CloneSplit<RadioButton>>(&self, default: T) -> S {
        CloneSplit::clone_split(RadioButton::new(default))
    }

    /// Created a named signal.
    pub fn signal<T: AsObject, S: CloneSplit<TypedSignal<T>>>(&self, name: impl Into<String> + Borrow<str>) -> S {
        let mut w = self.signals.0.write();
        if let Some(signal) = w.get(name.borrow()) {
            CloneSplit::clone_split(TypedSignal::from_inner(signal.clone()))
        } else {
            let signal = TypedSignal::default();
            w.insert(name.into(), signal.clone().into_inner());
            CloneSplit::clone_split(signal)
        }
    }

    /// Recursively despawn an entity, calls `despawn_recursive`.
    pub fn despawn(&mut self, entity: Entity) {
        self.commands.entity(entity).despawn_recursive()
    }

    /// Despawn descendants.
    pub fn despawn_descendants(&mut self, entity: Entity) {
        self.commands.entity(entity).despawn_descendants();
    }

    /// Despawn children with a specific component and their descendants.
    pub fn despawn_children_with<T: Component>(&mut self, entity: Entity) {
        pub struct DespawnDescendantsWith<T: Component>(Entity, PhantomData<T>);
        impl<T: Component> Command for DespawnDescendantsWith<T> {
            fn apply(self, world: &mut bevy::prelude::World) {
                let Some(children) = world.get::<Children>(self.0) else {return};
                let children = children.to_vec();
                for child in children {
                    if world.get::<T>(child).is_some() {
                        DespawnRecursive {entity: child}.apply(world);
                    }
                }
            }
        }

        self.commands.add(DespawnDescendantsWith::<T>(entity, PhantomData))
    }
}

impl AsRef<AssetServer> for RCommands<'_, '_> {
    fn as_ref(&self) -> &AssetServer {
        &self.asset_server
    }
}

impl<'w, 's> AsMut<Commands<'w, 's>> for RCommands<'w, 's> {
    fn as_mut(&mut self) -> &mut Commands<'w, 's> {
        &mut self.commands
    }
}
