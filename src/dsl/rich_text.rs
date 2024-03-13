use bevy::{utils::HashMap, asset::Handle, text::Font};

use crate::frame_extension;

use super::HandleOrAsset;


frame_extension!(
    pub struct DslRichTextBuilder {
        /// The text string.
        pub text: String,
        /// Handle of the font asset.
        pub fonts: HashMap<String, Handle<Font>>,
        /// The base font of the text box,
        /// also determines the line gap.
        pub base_font: HandleOrAsset<Font>,
        /// Color of the text.
        pub base_color: Option<bevy::prelude::Color>,
    }
);
