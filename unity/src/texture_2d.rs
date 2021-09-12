use libil2cpp::{unsafe_impl_reference_type, Il2CppArray, Il2CppObject, ObjectExt};
use std::ops::{Deref, DerefMut};

use crate::TextureFormat;

#[repr(C)]
pub struct Texture2D {
    object: Il2CppObject,
}

unsafe_impl_reference_type!(in libil2cpp for Texture2D => UnityEngine.Texture2D);

impl Texture2D {
    pub fn new(width: i32, height: i32, format: TextureFormat) -> &'static mut Self {
        <Self as ObjectExt>::new((width, height, format, false, false))
    }

    pub fn apply(&mut self) {
        self.invoke_void("Apply", ()).unwrap()
    }

    pub fn raw_texture_data(&mut self) -> &'static mut Il2CppArray<u8> {
        self.invoke("GetRawTextureData", ()).unwrap()
    }

    pub fn load_raw_texture_data(&mut self, data: &mut Il2CppArray<u8>) {
        self.invoke("LoadRawTextureData", data).unwrap()
    }
}

impl Deref for Texture2D {
    type Target = Il2CppObject;

    fn deref(&self) -> &Self::Target {
        &self.object
    }
}

impl DerefMut for Texture2D {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.object
    }
}
