//! Args Parser Module

use std::env::Args;

pub struct Config {
    pub all_flag: bool,
    pub show_detail_flag: bool,
}

impl Config {
    pub fn new(mut args: Args) -> Result<Config, String> {
        args.next();

        let mut config = Config {
            all_flag: false,
            show_detail_flag: false,
        };

        for arg in args {
            if arg.starts_with("-") {
                for c in arg.chars().skip(1) {
                    match c {
                        'a' => config.all_flag = true,
                        'l' => config.show_detail_flag = true,
                        _ => return Err(format!("Invalid option: {}", c))
                    }
                }
            } else {
                return Err(format!("Invalid argument: {}", arg))
            }
        }

        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn config_new() {
    }
}
