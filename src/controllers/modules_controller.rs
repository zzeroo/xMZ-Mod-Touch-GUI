use super::super::models::modules::Modules;

pub struct ModulesController;

impl ModulesController {
    pub fn get_modules() -> Vec<Modules> {
        (1..101).map(|name| Modules::new(format!("Module {}", name))).collect()
    }
}
