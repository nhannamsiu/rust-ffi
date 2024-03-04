use crate::c_wrapper::c_expertise;
use crate::expertise::Expertise;
#[derive(Debug)]
pub struct Person {
    pub name: String,
    pub expertise: *mut c_expertise,
}

impl Person {
    pub fn new(name: String, expertise: *mut c_expertise) -> Person {
        Person{
            name,
            expertise
        }
    }
}