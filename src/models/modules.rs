pub struct Modules {
    pub name: String,
    // sensors: Vec<Sensors>
}

impl Modules {
    pub fn new(name: String) -> Self {
        Modules { name: name }
    }
}
