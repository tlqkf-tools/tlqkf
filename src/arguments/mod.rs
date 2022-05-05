#[derive(Clone, Debug)]
pub struct Arguments(pub Vec<String>);

impl Arguments {
    pub fn contains(&self, value: &str) -> bool {
        return (*self.0).into_iter().find(|&arg| arg == value).is_some();
    }
}

mod mod_test;
