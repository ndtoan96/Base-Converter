//! Main module of the program.

use std::io::{stdin, stdout, Write};
use std::fmt;
use std::error::Error;

pub const START_CMD: &str = ":";
const HELP_MSG: &str = "
    -- Base Converter -- <Author: Nguyen Duc Toan>
Usage:
    :from <base> to <base>      change input base and output base
    :from <base>                change input base
    :to <base>                  change output base
<base> can be \"hex\", \"dec\", \"bin\"
    :h or :help                 print help message
    :q or :quit                 stop program

";

/// Enum for base types.
#[derive(PartialEq)]
enum Base {
    Bin,
    Dec,
    Hex,
}

impl fmt::Display for Base {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Base::Bin => write!(f, "bin"),
            Base::Hex => write!(f, "hex"),
            Base::Dec => write!(f, "dec"),
        }
    }
}

impl Base {
    /// Read a string and convert it to u64 based on base type.
    /// # Example:
    /// ```
    /// assert_eq!(Base::Bin.to_num("0b10").ok(), Some(3));
    /// assert_eq!(Base::Bin.to_num("0001_0000").ok(), Some(16));
    /// assert_eq!(Base::Hex.to_num("0xff").ok(), Some(255));
    /// ```
    pub fn to_num(&self, input: &str) -> Result<u64, Box<dyn Error>> {
    	let input = input.strip_suffix('u').unwrap_or(input);
        match self {
            Base::Bin => {
                let input = input.trim().to_lowercase().replace("_", "");
                if let Some(input) = input.strip_prefix("0b") {
                    let ret = u64::from_str_radix(input, 2)?;
                    return Ok(ret);
                } else {
                    let ret = u64::from_str_radix(&input, 2)?;
                    return Ok(ret);
                }
            },
            Base::Dec => {
                let ret = u64::from_str_radix(input, 10)?;
                return Ok(ret);
            }
            Base::Hex => {
                let input = input.trim().to_lowercase();
                if let Some(input) = input.strip_prefix("0x") {
                    let ret = u64::from_str_radix(input, 16)?;
                    return Ok(ret);
                } else {
                    let ret = u64::from_str_radix(&input, 16)?;
                    return Ok(ret);
                }
            }
        }
    }

    /// Format an u64 number based on base type. Return the formated `String`.
    /// # Example:
    /// ```
    /// assert_eq!(Base::Bin.from(4), "100");
    /// assert_eq!(Base::Bin.from(16), "0001_0000");
    /// assert_eq!(Base::Hex.from(255), "0xff");
    /// ```
    pub fn from(&self, mut num: u64) -> String {
        match self {
            Base::Hex => format!("0x{:x}", num),
            Base::Dec => format!("{}", num),
            Base::Bin => {
                if num < 16 {
                    format!("{:b}", num)
                } else {
                    let mut ret = Vec::new();
                    while num > 0 {
                        let four_bits = num & 0b1111;
                        num = num >> 4;
                        ret.push(format!("{:04b}", four_bits));
                    }
                    ret.into_iter().rev().collect::<Vec<String>>().join("_")
                }
            }
        }
    }
}

/// Main struct that manage the workflow of the aplication.
pub struct App {
    in_base: Base,
    out_base: Base,
}

impl App {
    /// Creat a new instance of App.
    pub fn new() -> Self {
        Self {
            in_base: Base::Hex,
            out_base: Base::Bin,
        }
    }

    /// Read stdin for user input.
    pub fn get_input(&self) -> String {
        print!("<{}>$ ", self.in_base);
        stdout().flush().expect("Fail flushing stdout");
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Fail reading input");
        return input.trim().to_string()
    }

    /// Print output to stdout.
    pub fn print(&self, out: &str) {
        println!("<{}> {}", self.out_base, out);
    }

    /// Convert an input from input base to output base. Default input base is hex
    /// and output base is bin. These bases can be changed with command `:from <base>`
    /// and `:to <base>`
    pub fn convert(&self, input: &str) -> Result<String, Box<dyn Error>> {
        let num = self.in_base.to_num(input)?;
        return Ok(self.out_base.from(num));
    }

    /// Check if user input is a command.
    pub fn is_command(&self, cmd: &str) -> bool {
        return cmd.starts_with(START_CMD);
    }

    /// Execute most of commands, except `:q` or `:quit`, these commands are
    /// executed from `main` function because it relates to the flow of the
    /// program, not the configuration.
    pub fn execute(&mut self, cmd: &str) -> Result<(), String> {
        if let Some(mut cmd) = cmd.strip_prefix(START_CMD) {
            cmd = cmd.trim();
            if cmd == "h" || cmd == "help" {
                self.help();
                return Ok(());
            }
            let words: Vec<&str> = cmd.split_ascii_whitespace().collect();
            if !(words.len() == 2 || words.len() == 4) {
                return Err(format!("Error: wrong command format"));
            } else {
                self.change_base(words[0], words[1])?;
                if words.len() == 4 {
                    self.change_base(words[2], words[3])?;
                }
            }
        } else {
            return Err(format!("Error: wrong command format"));
        }
        Ok(())
    }

    /// Change input and output base.
    fn change_base(&mut self, cmd: &str, arg: &str) -> Result<(), String> {
        match cmd {
            "from" => {
                self.in_base = match arg {
                    "hex" => Base::Hex,
                    "dec" => Base::Dec,
                    "bin" => Base::Bin,
                    _ => {
                        return Err(format!("No type {}", arg));
                    }
                }
            },
            "to" => {
                self.out_base = match arg {
                    "hex" => Base::Hex,
                    "dec" => Base::Dec,
                    "bin" => Base::Bin,
                    _ => {
                        return Err(format!("No type {}", arg));
                    }
                }
            }
            _ => {
                return Err(format!("Error: wrong command format"));
            }
        }

        Ok(())
    }

    /// Print help message.
    fn help(&self) {
        print!("{}", HELP_MSG);
    }
}

#[cfg(test)]
mod test_app {
    use super::*;
    #[test]
    #[ignore = "manual"]
    fn test_print() {
        let app = App::new();
        app.print("10");
    }

    #[test]
    #[ignore = "manual"]
    fn test_get_input() {
        let app = App::new();
        println!("{:?}", app.get_input());
    }

    #[test]
    fn test_is_command() {
        let app = App::new();

        // true cases
        assert!(app.is_command(&format!("{}from hex to dec", START_CMD)));

        // false cases
        assert!(!app.is_command("from hex to dec"));
        assert!(!app.is_command("0x42"));
        assert!(!app.is_command(""));
        assert!(!app.is_command("72"));
    }

    #[test]
    fn test_change_base() {
        let mut app = App::new();

        // Ok cases
        assert!(app.execute(&format!("{}from hex to dec", START_CMD)).is_ok() && app.in_base == Base::Hex && app.out_base == Base::Dec);
        assert!(app.execute(&format!("{}from hex to hex", START_CMD)).is_ok() && app.in_base == Base::Hex && app.out_base == Base::Hex);
        assert!(app.execute(&format!("{}from hex to bin", START_CMD)).is_ok() && app.in_base == Base::Hex && app.out_base == Base::Bin);
        assert!(app.execute(&format!("{}from bin to bin", START_CMD)).is_ok() && app.in_base == Base::Bin && app.out_base == Base::Bin);
        assert!(app.execute(&format!("{}from bin to dec", START_CMD)).is_ok() && app.in_base == Base::Bin && app.out_base == Base::Dec);
        assert!(app.execute(&format!("{}from bin to hex", START_CMD)).is_ok() && app.in_base == Base::Bin && app.out_base == Base::Hex);
        assert!(app.execute(&format!("{}  from bin   to     hex  ", START_CMD)).is_ok());
        assert!(app.execute(&format!("{}from hex", START_CMD)).is_ok() && app.in_base == Base::Hex);
        assert!(app.execute(&format!("{}from dec", START_CMD)).is_ok() && app.in_base == Base::Dec);
        assert!(app.execute(&format!("{}from bin", START_CMD)).is_ok() && app.in_base == Base::Bin);
        assert!(app.execute(&format!("{}to hex", START_CMD)).is_ok() && app.out_base == Base::Hex);
        assert!(app.execute(&format!("{}to dec", START_CMD)).is_ok() && app.out_base == Base::Dec);
        assert!(app.execute(&format!("{}to bin", START_CMD)).is_ok() && app.out_base == Base::Bin);

        // Err cases
        assert!(app.execute("from hex to dec").is_err());
        assert!(app.execute(&format!("{}:from hex to dec", START_CMD)).is_err());
        assert!(app.execute(&format!("{}from hex to dex", START_CMD)).is_err());
        assert!(app.execute(&format!("{}from hex to dec.", START_CMD)).is_err());
        assert!(app.execute(&format!("{}from hec", START_CMD)).is_err());
        assert!(app.execute(&format!("{}fro hex", START_CMD)).is_err());
        assert!(app.execute(&format!("{}t hex", START_CMD)).is_err());
        assert!(app.execute(&format!("{}to hx", START_CMD)).is_err());
    }
}

#[cfg(test)]
mod test_base {
    use super::*;
    #[test]
    fn test_to_num() {
        // Ok cases
        assert_eq!(Base::Hex.to_num("0xff").ok(), Some(255));
        assert_eq!(Base::Hex.to_num("ff").ok(), Some(255));
        assert_eq!(Base::Hex.to_num("0XFF").ok(), Some(255));
        assert_eq!(Base::Hex.to_num("0").ok(), Some(0));
        assert_eq!(Base::Hex.to_num("0x00").ok(), Some(0));
        assert_eq!(Base::Hex.to_num("0xffffffffffffffff").ok(), Some(std::u64::MAX));
        assert_eq!(Base::Bin.to_num("0b101010001101").ok(), Some(2701));
        assert_eq!(Base::Bin.to_num("0B101010001101").ok(), Some(2701));
        assert_eq!(Base::Bin.to_num("0b1010_1000_1101").ok(), Some(2701));
        assert_eq!(Base::Bin.to_num("1010_1000_1101").ok(), Some(2701));
        assert_eq!(Base::Dec.to_num("101").ok(), Some(101));
        
        // Error cases
        assert!(Base::Hex.to_num("0xgk").is_err());
        assert!(Base::Hex.to_num("-0xgk").is_err());
        assert!(Base::Bin.to_num("0b12").is_err());
        assert!(Base::Bin.to_num("012").is_err());
        assert!(Base::Dec.to_num("-012").is_err());
        assert!(Base::Dec.to_num("0d012").is_err());
    }

    #[test]
    fn test_bin_format() {
        assert_eq!(Base::Bin.from(15), String::from("1111"));
        assert_eq!(Base::Bin.from(16), String::from("0001_0000"));
        assert_eq!(Base::Bin.from(0), String::from("0"));
        assert_eq!(Base::Bin.from(1), String::from("1"));
        assert_eq!(Base::Bin.from(2), String::from("10"));
    }
}