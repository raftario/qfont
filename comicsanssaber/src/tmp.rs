use std::ops::{Deref, DerefMut};

use fontunity::Font;
use quest_hook::libil2cpp::{unsafe_impl_reference_type, Il2CppObject, Type};

pub struct TMPText {
    object: Il2CppObject,
}

pub struct TMPFontAsset {
    object: Il2CppObject,
}

impl TMPText {
    pub fn set_font(&mut self, font: &mut TMPFontAsset) {
        self.invoke_void("set_font", font).unwrap()
    }
}

impl TMPFontAsset {
    pub fn new(font: &mut Font) -> &'static mut Self {
        Self::class().invoke("CreateFontAsset", font).unwrap()
    }
}

unsafe_impl_reference_type!(in quest_hook::libil2cpp for TMPText => TMPro.TMP_Text);

unsafe_impl_reference_type!(in quest_hook::libil2cpp for TMPFontAsset => TMPro.TMP_FontAsset);

impl Deref for TMPText {
    type Target = Il2CppObject;

    fn deref(&self) -> &Self::Target {
        &self.object
    }
}

impl DerefMut for TMPText {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.object
    }
}

impl Deref for TMPFontAsset {
    type Target = Il2CppObject;

    fn deref(&self) -> &Self::Target {
        &self.object
    }
}

impl DerefMut for TMPFontAsset {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.object
    }
}
