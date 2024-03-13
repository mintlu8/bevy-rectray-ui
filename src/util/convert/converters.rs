use bevy::{asset::{Asset, Handle}, ecs::entity::Entity};
use bevy_defer::AsObject;
use crate::{widgets::button::Payload, util::WidgetBuilder, util::RCommands};

use super::{DslConvert, DslFrom, SealToken};


/// Extended `Option` for the DSL.
///
/// Since dependents of this crate cannot implement `DslFrom` on `Option<T>` with foreign types,
/// [`DslFromOptionEx`](super::DslFromOptionEx) can be used to make conversion to OptionX.
///
/// Using a crate local option type also works here.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum OptionEx<T> {
    Some(T),
    #[default]
    None,
}

impl<T> OptionEx<T> {
    pub fn expect(self, s: &str) -> T {
        match self {
            OptionEx::Some(v) => v,
            OptionEx::None => panic!("{}", s),
        }
    }

    pub fn unwrap_or(self, or: T) -> T {
        match self {
            OptionEx::Some(v) => v,
            OptionEx::None => or,
        }
    }

    pub fn unwrap_or_else(self, or: impl FnOnce() -> T) -> T {
        match self {
            OptionEx::Some(v) => v,
            OptionEx::None => or(),
        }
    }

    pub fn into_option(self) -> Option<T> {
        match self {
            OptionEx::Some(x) => Some(x),
            OptionEx::None => None,
        }
    }
}

/// For downstream crates,
/// implement this for specialized `Option` conversion with [`OptionEx`].
///
/// Enables conversion from `T` to [`OptionEx<Self>`].
pub trait DslFromOptionEx<T> {
    fn dfrom_option(value: T) -> Self;
}

impl<T, U> DslConvert<OptionEx<U>, 'O'> for T where U: DslFromOptionEx<T> {
    fn parse(self) -> OptionEx<U> {
        OptionEx::Some(U::dfrom_option(self))
    }
    fn sealed(_: SealToken) {}
}

impl<T> DslConvert<Option<Payload>, 'P'> for T where T: AsObject{
    fn parse(self) -> Option<Payload> {
        Some(Payload::new(self))
    }
    fn sealed(_: SealToken) {}
}

/// An [`Asset`], [`Handle<Asset>`], string path of an asset or none/default.
#[derive(Debug, Clone, Default, PartialEq)]
pub enum IntoAsset<T: Asset>{
    #[default]
    None,
    Raw(T),
    Handle(Handle<T>),
    String(String),
}

impl<T> DslConvert<IntoAsset<T>, 'A'> for T where T: Asset {
    fn parse(self) -> IntoAsset<T> {
        IntoAsset::Raw(self)
    }
    fn sealed(_: SealToken) {}
}

impl<T> DslConvert<IntoAsset<T>, 'A'> for Handle<T> where T: Asset {
    fn parse(self) -> IntoAsset<T> {
        IntoAsset::Handle(self)
    }
    fn sealed(_: SealToken) {}
}

impl<T> DslConvert<IntoAsset<T>, 'A'> for &Handle<T> where T: Asset {
    fn parse(self) -> IntoAsset<T> {
        IntoAsset::Handle(self.clone())
    }
    fn sealed(_: SealToken) {}
}

impl<T> DslConvert<IntoAsset<T>, 'a'> for String where T: Asset {
    fn parse(self) -> IntoAsset<T> {
        IntoAsset::String(self)
    }
    fn sealed(_: SealToken) {}
}

impl<T> DslConvert<IntoAsset<T>, 'a'> for &str where T: Asset {
    fn parse(self) -> IntoAsset<T> {
        IntoAsset::String(self.to_owned())
    }
    fn sealed(_: SealToken) {}
}


/// An [`Entity`] or a [`WidgetBuilder`].
#[derive(Debug, Clone, Default)]
pub enum IntoEntity<T>{
    #[default]
    None,
    Entity(Entity),
    Builder(WidgetBuilder<T>),
}

impl IntoEntity<()> {
    pub fn build_expect(&self, commands: &mut RCommands, panic: &str) -> Entity {
        match self {
            IntoEntity::None => panic!("{}", panic),
            IntoEntity::Entity(e) => *e,
            IntoEntity::Builder(b) => commands.spawn_dynamic(b),
        }
    }

    pub fn build_or(&self, commands: &mut RCommands, or: impl FnOnce() -> Entity) -> Entity {
        match self {
            IntoEntity::None => or(),
            IntoEntity::Entity(e) => *e,
            IntoEntity::Builder(b) => commands.spawn_dynamic(b),
        }
    }
}

impl<T> IntoEntity<T> {
    pub fn build_with_expect(&self, commands: &mut RCommands, arg: T, panic: &str) -> Entity {
        match self {
            IntoEntity::None => panic!("{}", panic),
            IntoEntity::Entity(e) => *e,
            IntoEntity::Builder(b) => commands.spawn_fn(b, arg),
        }
    }

    pub fn build_with_or(&self, commands: &mut RCommands, arg: T, or: impl FnOnce(T) -> Entity) -> Entity {
        match self {
            IntoEntity::None => or(arg),
            IntoEntity::Entity(e) => *e,
            IntoEntity::Builder(b) => commands.spawn_fn(b, arg),
        }
    }
}

impl<T> DslFrom<Entity> for IntoEntity<T> {
    fn dfrom(value: Entity) -> Self {
        IntoEntity::Entity(value)
    }
}

impl<T> DslFrom<WidgetBuilder<T>> for IntoEntity<T> {
    fn dfrom(value: WidgetBuilder<T>) -> Self {
        IntoEntity::Builder(value)
    }
}

impl<F> DslConvert<IntoEntity<()>, 'ë'> for F where F: Fn(&mut RCommands) -> Entity + Send + Sync + 'static {
    fn parse(self) -> IntoEntity<()> {
        IntoEntity::Builder(WidgetBuilder::new(self))
    }
    fn sealed(_: SealToken) {}
}

impl<F, T> DslConvert<IntoEntity<T>, 'ê'> for F where F: Fn(&mut RCommands, T) -> Entity + Send + Sync + 'static {
    fn parse(self) -> IntoEntity<T> {
        IntoEntity::Builder(WidgetBuilder::new(self))
    }
    fn sealed(_: SealToken) {}
}


impl RCommands<'_, '_>{
    /// Load a dsl `IntoAsset`, if `None`, returns the default value.
    pub fn load_or_default<T: Asset>(&self, asset: IntoAsset<T>) -> Handle<T> {
        match asset {
            IntoAsset::None => Default::default(),
            IntoAsset::Raw(val) => self.add_asset(val),
            IntoAsset::Handle(handle) => handle,
            IntoAsset::String(string) => self.load(string),
        }
    }

    /// Load a dsl `IntoAsset`, if `None`, panic.
    pub fn load_or_panic<T: Asset>(&self, asset: IntoAsset<T>, err_msg: &str) -> Handle<T> {
        match asset {
            IntoAsset::None => panic!("{}", err_msg),
            IntoAsset::Raw(val) => self.add_asset(val),
            IntoAsset::Handle(handle) => handle,
            IntoAsset::String(string) => self.load(string),
        }
    }

    /// Load a dsl `IntoAsset`, returns an `Option`.
    pub fn try_load<T: Asset>(&self, asset: IntoAsset<T>) -> Option<Handle<T>> {
        match asset {
            IntoAsset::None => None,
            IntoAsset::Raw(val) => Some(self.add_asset(val)),
            IntoAsset::Handle(handle) => Some(handle),
            IntoAsset::String(string) => Some(self.load(string)),
        }
    }
}


impl<T: Asset> IntoAsset<T> {
    pub fn is_some(&self) -> bool{
        !matches!(self, Self::None)
    }

    pub fn is_none(&self) -> bool{
        matches!(self, Self::None)
    }
}
