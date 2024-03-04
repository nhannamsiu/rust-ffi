use crate::expertise::Expertise;

#[derive(Debug)]
pub struct Person {
    pub name: String,
    pub expertise: *mut Expertise,
}

impl Person {
    pub fn new(name: String, expertise: *mut Expertise) -> Person {
        Person{
            name,
            expertise
        }
    }
}