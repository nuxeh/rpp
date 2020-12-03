use std::process::Command;

#[derive(Default)]
pub struct Rpp {
    time: bool,
    memory: bool,
}

impl Rpp {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn time(mut self, enabled: bool) -> Self {
        self.time = enabled;
        self
    }

    pub fn memory(mut self, enabled: bool) -> Self {
        self.memory = enabled;
        self
    }

    pub fn init(mut self) -> Self {
        self
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
