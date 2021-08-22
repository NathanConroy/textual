
pub struct Config {
    pub file_nm: String    
}

impl Config {
    pub fn new(args: Vec<String>) -> Result<Self, String> {
        if args.len() < 2 {
            return Err(String::from("Expected more arguments."));
        }
        Ok(Self {
            file_nm: String::from(&args[1]),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_init_config() {
        let pnm = String::from("textual");
        let fnm = String::from("file_nm");
        let result = Config::new(vec![pnm, fnm]);
        assert!(result.is_ok());
    }

    #[test]
    fn too_few_args() {
        let pnm = String::from("textual");
        let result = Config::new(vec![pnm]);
        assert!(result.is_err());
    }
}
