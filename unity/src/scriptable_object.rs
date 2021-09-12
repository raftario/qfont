use libil2cpp::{
    raw, unsafe_impl_reference_type, Il2CppObject, Il2CppReflectionType, Type, WrapRaw,
};
use std::{
    mem::transmute,
    ops::{Deref, DerefMut},
};

#[repr(C)]
pub struct ScriptableObject {
    object: Il2CppObject,
}

unsafe_impl_reference_type!(in libil2cpp for ScriptableObject => UnityEngine.ScriptableObject);

impl ScriptableObject {
    #[rustfmt::skip]
    pub fn create_instance<T>() -> &'static mut T
    where
        for<'a> T: Type<Held<'a> = Option<&'a mut T>>,
    {
        let ty = T::class().ty();
        let ty = unsafe { Il2CppReflectionType::wrap_mut(raw::type_get_object(ty.raw())) };
        let object: &mut Self = Self::class().invoke("CreateInstance", ty).unwrap();
        unsafe { transmute(object) }
    }
}

impl Deref for ScriptableObject {
    type Target = Il2CppObject;

    fn deref(&self) -> &Self::Target {
        &self.object
    }
}

impl DerefMut for ScriptableObject {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.object
    }
}
