use std::process::Command;
use std::time::{Duration, Instant};
use failure::{Error, bail};
use std::fs;

#[derive(Default)]
pub struct Rpp {
    time: bool,
    peak_vm: bool,
    command: Option<Command>,
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

    pub fn peak_vm(mut self, enabled: bool) -> Self {
        self.peak_vm = enabled;
        self
    }

    pub fn command(&mut self, command: &str) -> &mut Self {
        self.command = Some(Command::new(command));
        self
    }

    pub fn arg(&mut self, arg: &str) -> &mut Self {
        if let Some(ref mut c) = self.command {
            c.arg(arg);
        }
        self
    }

    pub fn run(&mut self) -> Result<(), Error> {
        if let Some(ref mut c) = self.command {
            let start = Instant::now();

            if let Ok(mut child) = c.spawn() {
                let pid = child.id();
                eprintln!("{}", pid);
                loop {
                    eprintln!("{:?}", get_peak_vm(pid));
                    match child.try_wait() {
                        Ok(Some(_)) => break,
                        Err(e) => bail!(e),
                        _ => (),
                    }
                }
            } else {
                bail!("error spawning child process");
            }

            self.results.duration = Some(start.elapsed());
        } else {
            bail!("command hasn't been initialised");
        }

        Ok(())
    }

    pub fn get_results(&self) -> &Results {
        &self.results
    }
}

#[derive(Default)]
pub struct Results {
    peak_vm: Option<usize>,
    duration: Option<Duration>,
}

impl Results {
    pub fn get_peak_vm(&self) -> Option<usize> {
        self.peak_vm
    }

    pub fn get_duration(&self) -> Option<Duration> {
        self.duration
    }

    pub fn get_nanos(&self) -> Option<u128> {
        self.duration.map(|d| d.as_nanos())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

fn get_peak_vm(pid: u32) -> Option<u32> {
    fs::read_to_string(format!("/proc/{}/status", pid))
        .ok()
        .map(|s| {
            s.lines()
                .filter(|l| l.contains("VmPeak"))
                .take(1)
                .collect::<String>()
                .rsplit(" ")
                .skip(1)
                .take(1)
                .collect::<String>()
                .parse()
                .ok()
        })
    .flatten()
}
