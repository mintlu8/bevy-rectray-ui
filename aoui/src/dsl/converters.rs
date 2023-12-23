use std::borrow::Cow;
use bevy::asset::{Asset, Handle, AssetServer};
use crate::{signals::DataTransfer, widgets::button::Payload};
use super::{DslFrom, DslInto};

/// Extended `Option` for the DSL.
/// 
/// Since we basically cannot extend `Option<T>`'s features
/// due to a blanket impl, this provides specific implementation
/// for a nullable struct.
#[derive(Debug, Default)]
pub enum OptionX<T> {
    Some(T),
    #[default]
    None,
}

impl<T> OptionX<T> {
    pub fn expect(self, s: &str) -> T {
        match self {
            OptionX::Some(v) => v,
            OptionX::None => panic!("{}", s),
        }
    }

    pub fn unwrap_or(self, or: T) -> T {
        match self {
            OptionX::Some(v) => v,
            OptionX::None => or,
        }
    }

    pub fn unwrap_or_else(self, or: impl FnOnce() -> T) -> T {
        match self {
            OptionX::Some(v) => v,
            OptionX::None => or(),
        }
    }
}

impl<T: DataTransfer + Clone> DslFrom<T> for OptionX<Payload> {
    fn dfrom(value: T) -> Self {
        OptionX::Some(Payload::new(value))
    }
}

/// Handle, string or None,
/// `get` returns the default asset on `None`, try_get returns `None`.
#[derive(Debug, Default, PartialEq)]
pub enum HandleOrString<T: Asset>{
    #[default]
    None,
    Handle(Handle<T>),
    String(String),
}

#[doc(hidden)]
pub trait IntoAssets{
    fn get(&self) -> &AssetServer;
}

impl IntoAssets for &AssetServer {
    fn get(&self) -> &AssetServer {
        self
    }
}

impl IntoAssets for Option<&AssetServer> {
    fn get(&self) -> &AssetServer {
        self.expect("Please pass in the AssetServer.")
    }
}

impl<T: Asset> HandleOrString<T> {
    /// This uses the default behavior of treating unspecified as the default asset.
    pub fn get(self, assets: impl IntoAssets) -> Handle<T>{
        match self {
            HandleOrString::None => Default::default(),
            HandleOrString::Handle(handle) => handle,
            HandleOrString::String(string) => {
                assets.get().load(string)
            },
        }
    }

    pub fn expect(self, assets: impl IntoAssets, err: &str) -> Handle<T>{
        match self {
            HandleOrString::None => panic!("{}", err),
            HandleOrString::Handle(handle) => handle,
            HandleOrString::String(string) => {
                assets.get().load(string)
            },
        }
    }

    pub fn try_get(self, assets: impl IntoAssets) -> Option<Handle<T>>{
        match self {
            HandleOrString::None => None,
            HandleOrString::Handle(handle) => Some(handle),
            HandleOrString::String(string) => {
                Some(assets.get().load(string))
            },
        }
    }

    pub fn is_some(&self) -> bool{
        match self {
            HandleOrString::None => false,
            _ => true,
        }
    }

    pub fn is_none(&self) -> bool{
        !self.is_some()
    }
}

impl<T: Asset> DslInto<HandleOrString<T>> for Handle<T> {
    fn dinto(self) -> HandleOrString<T> {
        HandleOrString::Handle(self)
    }
}

impl<T: Asset> DslInto<HandleOrString<T>> for &Handle<T> {
    fn dinto(self) -> HandleOrString<T> {
        HandleOrString::Handle(self.clone())
    }
}

impl<T: Asset> DslInto<HandleOrString<T>> for &str {
    fn dinto(self) -> HandleOrString<T> {
        HandleOrString::String(self.to_owned())
    }
}

impl<T: Asset> DslInto<HandleOrString<T>> for String {
    fn dinto(self) -> HandleOrString<T> {
        HandleOrString::String(self)
    }
}

impl<T: Asset> DslInto<HandleOrString<T>> for &&str {
    fn dinto(self) -> HandleOrString<T> {
        HandleOrString::String((*self).to_owned())
    }
}

impl<T: Asset> DslInto<HandleOrString<T>> for &String {
    fn dinto(self) -> HandleOrString<T> {
        HandleOrString::String(self.clone())
    }
}

impl<'t, T: Asset> DslInto<HandleOrString<T>> for Cow<'t, str> {
    fn dinto(self) -> HandleOrString<T> {
        HandleOrString::String(self.into_owned())
    }
}


/// Handle, Asset or None,
/// `get` returns the default asset on `None`, try_get returns `None`.
#[derive(Debug, Default)]
pub enum HandleOrAsset<T: Asset>{
    #[default]
    None,
    Handle(Handle<T>),
    Asset(T),
}

impl<T: Asset> HandleOrAsset<T> {
    pub fn get(self, assets: impl IntoAssets) -> Handle<T>{
        match self {
            HandleOrAsset::None => Default::default(),
            HandleOrAsset::Handle(handle) => handle,
            HandleOrAsset::Asset(asset) => {
                assets.get().add(asset)
            },
        }
    }

    pub fn expect(self, assets: impl IntoAssets, msg: &str) -> Handle<T>{
        match self {
            HandleOrAsset::None => panic!("{}", msg),
            HandleOrAsset::Handle(handle) => handle,
            HandleOrAsset::Asset(asset) => {
                assets.get().add(asset)
            },
        }
    }

    pub fn try_get(self, assets: Option<&AssetServer>) -> Option<Handle<T>>{
        match self {
            HandleOrAsset::None => None,
            HandleOrAsset::Handle(handle) => Some(handle),
            HandleOrAsset::Asset(asset) => {
                Some(assets.get().add(asset))
            },
        }
    }

}

impl<T: Asset> DslFrom<Handle<T>> for HandleOrAsset<T>{
    fn dfrom(value: Handle<T>) -> Self {
        HandleOrAsset::Handle(value)
    }
}

impl<T: Asset> DslFrom<&Handle<T>> for HandleOrAsset<T>{
    fn dfrom(value: &Handle<T>) -> Self {
        HandleOrAsset::Handle(value.clone())
    }
}


impl<T: Asset> DslFrom<T> for HandleOrAsset<T>{
    fn dfrom(value: T) -> Self {
        HandleOrAsset::Asset(value)
    }
}

