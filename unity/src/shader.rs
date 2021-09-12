use libil2cpp::{unsafe_impl_reference_type, Il2CppObject, Il2CppString, Type};
use std::ops::{Deref, DerefMut};

#[repr(C)]
pub struct Shader {
    object: Il2CppObject,
}

unsafe_impl_reference_type!(in libil2cpp for Shader => UnityEngine.Shader);

impl Shader {
    pub fn find(name: &str) -> Option<&'static mut Self> {
        Self::class()
            .invoke("Find", Il2CppString::new(name))
            .unwrap()
    }
}

impl Deref for Shader {
    type Target = Il2CppObject;

    fn deref(&self) -> &Self::Target {
        &self.object
    }
}

impl DerefMut for Shader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.object
    }
}
