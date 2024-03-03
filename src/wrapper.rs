use crate::expertise::Expertise;

use libc::{c_char, c_void};
use std::ffi::CStr;

pub struct c_expertise {
    pub(crate) expertise: Expertise
}

#[no_mangle]
pub unsafe extern "C" fn expertise_create(
    field: *const c_char,
    experience: usize,
) -> *mut c_expertise {

    let rust_field = CStr::from_ptr(field);
    let rust_str = rust_field.to_str().expect("Bad encoding");
    let owned = rust_str.to_owned();

    let expertise = Expertise{
        field: owned,
        experience
    };
   let wrapper = c_expertise{
        expertise
    };
    Box::into_raw(Box::new(wrapper))
}

#[no_mangle]
pub unsafe extern "C" fn expertise_print(wrapper: *mut c_expertise) {
    println!("{:?}", (*wrapper).expertise)
}

#[no_mangle]
pub unsafe extern "C" fn expertise_destroy(expertise: *mut Expertise) {
    if expertise.is_null() {
        return;
    }
    drop(Box::from_raw(expertise))
}