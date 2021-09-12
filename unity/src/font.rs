use libil2cpp::{unsafe_impl_reference_type, Il2CppArray, Il2CppObject, ObjectExt};
use std::{
    lazy::SyncOnceCell,
    mem::transmute,
    ops::{Deref, DerefMut},
};

use crate::{CharacterInfo, Material};

#[repr(C)]
pub struct Font {
    object: Il2CppObject,
}

unsafe_impl_reference_type!(in libil2cpp for Font => UnityEngine.Font);

impl Font {
    pub fn new() -> &'static mut Self {
        <Self as ObjectExt>::new(())
    }

    pub fn set_character_info(&mut self, info: &mut Il2CppArray<CharacterInfo>) {
        static METHOD: SyncOnceCell<extern "C" fn(*mut Font, *mut Il2CppArray<CharacterInfo>)> =
            SyncOnceCell::new();
        METHOD.get_or_init(|| unsafe {
            let ptr = libil2cpp::raw::resolve_icall(
                b"UnityEngine.Font::set_characterInfo\0".as_ptr().cast(),
            );
            transmute(ptr.unwrap())
        })(self, info)
    }

    pub fn set_material(&mut self, material: &mut Material) {
        static METHOD: SyncOnceCell<extern "C" fn(*mut Font, *mut Material)> = SyncOnceCell::new();
        METHOD.get_or_init(|| unsafe {
            let ptr =
                libil2cpp::raw::resolve_icall(b"UnityEngine.Font::set_material\0".as_ptr().cast());
            transmute(ptr.unwrap())
        })(self, material)
    }
}

impl Deref for Font {
    type Target = Il2CppObject;

    fn deref(&self) -> &Self::Target {
        &self.object
    }
}

impl DerefMut for Font {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.object
    }
}
