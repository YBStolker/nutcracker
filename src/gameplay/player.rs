#[derive(Debug)]
pub struct Player {
    name: String,
    stack: u32,
}

impl Player {
    pub fn new(name: String, stack: u32) -> Self {
        Self { name, stack }
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn stack(&self) -> u32 {
        self.stack
    }
}
