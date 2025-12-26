pub struct Stock{
    symbol: String,
    name: String,
}

impl Stock{
    pub fn new(symbol: String, name: String) -> Self{
        Self{ symbol, name }
    }

    pub fn symbol(&self) -> &str{
        &self.symbol
    }

    pub fn name(&self) -> &str{
        &self.name
    }
}