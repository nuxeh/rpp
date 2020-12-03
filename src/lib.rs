use std::process::Command;
use std::time::{Duration, Instant};
use failure::{Error, bail};

#[derive(Default)]
pub struct Rpp {
    time: bool,
    memory: bool,
    command: Option<String>,
    results: Results,
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

    pub fn command(mut self, command: &str) -> Self {
        self.command = Some(command.to_owned());
        self
    }

    pub fn init(mut self) -> Self {
        self
    }

    pub fn run(&mut self) -> Result<(), Error> {
        if self.command.is_none() {
            bail!("Command hasn't been set");
        }

        let start = Instant::now();

        let nanos = start.elapsed().as_nanos();
        self.results.nanos = Some(nanos);
        Ok(())
    }
}

#[derive(Default)]
pub struct Results {
    memory: Option<usize>,
    nanos: Option<u128>,
}

impl Results {
    fn get_memory(&self) -> Option<usize> {
        self.memory
    }

    fn get_nanos(&self) -> Option<u128> {
        self.nanos
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
