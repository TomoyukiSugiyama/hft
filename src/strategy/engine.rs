pub struct Engine {
  name: String
}

impl Engine {
    pub fn new() -> Self{
        Self {
            name: "engine".to_string()
        }
    }

    pub fn name(&self) -> &str{
        &self.name
    }
}