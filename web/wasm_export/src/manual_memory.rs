pub fn create_pointer<T>(thing: T) -> u64 {
    Box::into_raw(Box::new(thing)) as u64
}

pub fn drop_pointer<T>(pointer: u64) {
    let pointer = pointer as *mut T;
    let _fit = unsafe { Box::from_raw(pointer) };
}
pub fn use_pointer_mut<T>(pointer: u64) -> &'static mut T {
    let pointer = pointer as *mut T;
    unsafe { pointer.as_mut() }.unwrap()
}
pub fn use_pointer<T>(pointer: u64) -> &'static T {
    let pointer = pointer as *mut T;
    unsafe { pointer.as_ref() }.unwrap()
}
