use libil2cpp::{unsafe_impl_reference_type, Argument, Il2CppArray, Il2CppObject, ObjectExt, Type};
use std::ops::{Deref, DerefMut};

#[repr(C)]
pub struct List<T: Type> {
    object: Il2CppObject,
    items: *mut Il2CppArray<T>,
    size: i32,
}

unsafe_impl_reference_type!(in libil2cpp for List<T> => System.Collections.Generic.List<T>);

impl<T: Type> List<T> {
    pub fn new(capacity: i32) -> &'static mut Self {
        <Self as ObjectExt>::new(capacity)
    }

    pub fn add<I>(&mut self, item: I)
    where
        I: Argument<Type = T>,
    {
        self.invoke("Add", item).unwrap()
    }
}

impl<T: Type> Deref for List<T> {
    type Target = Il2CppObject;

    fn deref(&self) -> &Self::Target {
        &self.object
    }
}

impl<T: Type> DerefMut for List<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.object
    }
}
