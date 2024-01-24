#[derive(Debug, Clone)]
pub struct Player {
    name: String,
    stack: u32,
}

impl Player {
    pub fn new(name: impl Into<String>, stack: u32) -> Self {
        let name = name.into();
        Self { name, stack }
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn stack(&self) -> u32 {
        self.stack
    }
}
