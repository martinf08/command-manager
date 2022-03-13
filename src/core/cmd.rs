use std::error::Error;
use std::process::Command;

#[derive(Debug)]
pub struct Cmd {
    cmd: String,
    args: Vec<String>,
}

impl Cmd {
    pub fn new(line: &str) -> Result<Self, Box<dyn Error>> {
        let (cmd, args) = Self::parse_line(line)?;

        Ok(Cmd { cmd, args })
    }

    fn parse_line(line: &str) -> Result<(String, Vec<String>), Box<dyn Error>> {
        let line = line.to_string();

        let mut split = line
            .split_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();

        match split.len() {
            x if x > 1 && split[0] != "sh" && split[1] != "-c" => {
                let cmd = "sh".to_string();
                let mut args = vec!["-c".to_string()];
                args.extend(vec![split.join(" ")]);

                return Ok((cmd, args));
            }

            x if x > 1 && split[0] == "sh" => {
                let cmd = split.remove(0);

                return Ok((cmd, split));
            }

            x if x == 1 && split[0] == "sh" => {
                let cmd = split.remove(0);

                return Ok((cmd, vec![]));
            }
            x if x == 1 && split[0] != "sh" => {
                let args = vec!["-c".to_string(), split.remove(0)]
                    .into_iter()
                    .map(|s| s.to_string())
                    .collect();

                return Ok(("sh".to_string(), args));
            }
            0 => panic!("Command line is empty"),
            _ => panic!("Command line is invalid"),
        }
    }

    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        let mut cmd = Command::new(self.cmd.clone());
        cmd.args(self.args.clone());
        cmd.spawn()?.wait()?;
        Ok(())
    }

    pub fn create_and_run(line: &str) -> Result<Self, Box<dyn Error>> {
        let cmd = Cmd::new(line)?;
        cmd.run()?;
        Ok(cmd)
    }
}

mod tests {
    use super::*;

    #[allow(unused)]
    fn exec_valid_cmd(line: &str) {
        let cmd = Cmd::new(line);

        assert!(cmd.is_ok());
        let cmd = cmd.unwrap();

        assert!(cmd.run().is_ok());
        cmd.run().unwrap();
    }

    #[test]
    fn test_parsing() {
        exec_valid_cmd("cd ~/");
    }

    #[test]
    fn test_initial_sh() {
        exec_valid_cmd("cd")
    }

    #[test]
    fn test_initial_sh_with_args() {
        exec_valid_cmd("sh -c cd")
    }
}
