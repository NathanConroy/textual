

pub struct Config {
    pub file_nm: String    
}

impl Config {
    pub fn new(args: Vec<String>) -> Self {
        Self {
            file_nm: String::from(&args[1]),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_args() {
        let pnm = String::from("textual");
        let fnm = String::from("file_nm");
        let config = Config::new(vec![pnm, fnm]);
    }
}
