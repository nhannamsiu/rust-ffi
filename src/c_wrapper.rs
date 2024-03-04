use crate::{
    expertise::Expertise,
    person::Person
};

use libc::{c_char};
use std::ffi::CStr;

pub struct c_expertise {
    pub(crate) expertise: Expertise
}

#[no_mangle]
pub unsafe extern "C" fn expertise_create(
    field: *const c_char,
    experience: usize,
) -> *mut c_expertise {

    let rust_field = CStr::from_ptr(field).to_str().expect("Bad encoding").to_owned();
    let expertise = Expertise{
        field: rust_field,
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
pub unsafe extern "C" fn expertise_update(
    wrapper: *mut c_expertise,
    field: *const c_char,
    experience: usize,
) {
    // raw pointer is unsafe, we always need to check if it valid before deref to avoid crash
    // not that you cannot move ownership back to Rust because it would cause unsafe behaviors on both C and Rust
    // luckly rust prevents that so this line won't work: let expertise = (*wrapper).expertise;
    if !wrapper.is_null() {
        let rust_field = CStr::from_ptr(field).to_str().expect("Bad encoding").to_owned();
        (*wrapper).expertise.field = rust_field;
        (*wrapper).expertise.experience = experience;
    }
}

#[no_mangle]
pub unsafe extern "C" fn expertise_destroy(wrapper: *mut c_expertise) {
    if wrapper.is_null() {
        return;
    }
    drop(Box::from_raw(wrapper))
}


pub struct c_person {
    pub(crate) person: Person
}

#[no_mangle]
pub unsafe extern "C" fn person_create(
    name: *const c_char,
    c_expertise: *mut c_expertise,
) -> *mut c_person {

    let rust_name = CStr::from_ptr(name).to_str().expect("Bad encoding").to_owned();
    let person = Person{
        name: rust_name,
        expertise: &mut (*c_expertise).expertise,
    };

    let wrapper = c_person{
        person
    };
    Box::into_raw(Box::new(wrapper))
}

#[no_mangle]
pub unsafe extern "C" fn person_print(wrapper: *mut c_person) {
    // raw pointer is unsafe, we always need to check if it valid before deref to avoid crash
    if !wrapper.is_null() {
        println!("{:?}", (*wrapper).person);
    }
    if !(*wrapper).person.expertise.is_null() {
        println!("{:?}", *(*wrapper).person.expertise);
    }
}

#[no_mangle]
pub unsafe extern "C" fn person_destroy(wrapper: *mut c_person) {
    if wrapper.is_null() {
        return;
    }
    drop(Box::from_raw(wrapper))
}