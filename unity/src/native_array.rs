use libil2cpp::unsafe_impl_value_type;
use std::slice;

#[repr(C)]
pub struct NativeArray<T> {
    buffer: *mut T,
    length: i32,
    allocator: i32,
}

unsafe_impl_value_type!(in libil2cpp for NativeArray<T> => Unity.Collections.NativeArray<T>);

impl<T> NativeArray<T> {
    /// # Safety
    /// The memory pointed to by the array must still be valid
    pub unsafe fn as_slice(&self) -> &[T] {
        slice::from_raw_parts(self.buffer, self.length as usize)
    }

    /// # Safety
    /// The memory pointed to by the array must still be valid
    pub unsafe fn as_mut_slice(&mut self) -> &mut [T] {
        slice::from_raw_parts_mut(self.buffer, self.length as usize)
    }
}
