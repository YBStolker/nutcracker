#[derive(Debug)]
pub struct Player {
    name: String,
    stack: u32,
}

impl Player {
    pub fn new(name: String, stack: u32) -> Self {
        Self { name, stack }
    }
}
