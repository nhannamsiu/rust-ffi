use crate::expertise::Expertise;

pub struct Person {
    pub name: String,
    pub expertise: Expertise,
}

impl Person {
    pub fn new(name: String, expertise: Expertise) -> Person {
        Person{
            name,
            expertise
        }
    }
}