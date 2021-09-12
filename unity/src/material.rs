use libil2cpp::{unsafe_impl_reference_type, Il2CppObject, Il2CppString, ObjectExt};
use std::ops::{Deref, DerefMut};

use crate::{texture_2d::Texture2D, Shader};

#[repr(C)]
pub struct Material {
    object: Il2CppObject,
}

unsafe_impl_reference_type!(in libil2cpp for Material => UnityEngine.Material);

impl Material {
    pub fn new(shader: &mut Shader) -> &'static mut Self {
        <Self as ObjectExt>::new(shader)
    }

    pub fn set_main_texture(&mut self, texture: &mut Texture2D) {
        self.invoke("set_mainTexture", texture).unwrap()
    }

    pub fn set_float(&mut self, name: &str, value: f32) {
        self.invoke("SetFloat", (Il2CppString::new(name), value))
            .unwrap()
    }
}

impl Deref for Material {
    type Target = Il2CppObject;

    fn deref(&self) -> &Self::Target {
        &self.object
    }
}

impl DerefMut for Material {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.object
    }
}
