#[derive(Debug)]
pub struct Expertise {
    pub field: String,
    pub experience: usize,
}

impl Expertise {
    pub fn new(field: String, experience: usize) -> Expertise {
        Expertise{
            field,
            experience
        }
    }
}