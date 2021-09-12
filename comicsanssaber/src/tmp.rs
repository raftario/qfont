use fontunity::TMPFontAsset;
use quest_hook::libil2cpp::{unsafe_impl_reference_type, Il2CppObject};
use std::ops::{Deref, DerefMut};

pub struct TMPText {
    object: Il2CppObject,
}

impl TMPText {
    pub fn set_font(&mut self, font: &mut TMPFontAsset) {
        self.invoke_void("set_font", font).unwrap()
    }
}

unsafe_impl_reference_type!(in quest_hook::libil2cpp for TMPText => TMPro.TMP_Text);

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
