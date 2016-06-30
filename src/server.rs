use module::Module;

pub struct Server {
    pub modules: Vec<Module>,
}

impl Server {
    pub fn new() -> Self {
        Server {
            modules: vec![],
        }
    }
}

#[cfg(test)]
mod test {
    use server::{Server};
    use module::{Module};

    #[test]
    fn add_modules() {
        let mut server = Server::new();
        let module1 = Module::new();

        assert_eq!(server.modules.len(), 0);
        server.modules.push(module1);
        assert_eq!(server.modules.len(), 1);
    }
}
