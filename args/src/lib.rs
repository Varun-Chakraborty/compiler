pub struct Args {
    pub input_filename: Option<String>,
    pub debug: bool,
    pub pretty: bool,
    pub log_to: Option<String>,
    pub path: Option<String>,
    pub filename: Option<String>,
}

#[derive(Debug, thiserror::Error)]
pub enum ArgsError {
    #[error("Invalid flag: {0}")]
    InvalidFlag(String),
}

impl Args {
    pub fn parse() -> Result<Self, ArgsError> {
        let args: Vec<String> = std::env::args().collect();
        if args.len() < 2 {
            return Ok(Self {
                input_filename: None,
                debug: false,
                pretty: false,
                log_to: None,
                path: None,
                filename: None,
            });
        }
        let debug = args.contains(&"--debug".to_string());
        let pretty = args.contains(&"--pretty".to_string());
        let log_to = args.iter().fold(None, |acc, x| {
            if x.contains("--log=") {
                Some(x[6..].to_string())
            } else {
                acc
            }
        });
        let path = args.iter().fold(None, |acc, x| {
            if x.contains("--path=") {
                Some(x[7..].to_string())
            } else {
                acc
            }
        });
        let filename = args.iter().fold(None, |acc, x| {
            if x.contains("--filename=") {
                Some(x[11..].to_string())
            } else {
                acc
            }
        });
        Ok(Self {
            input_filename: Some(args[1].clone()),
            debug,
            pretty,
            log_to,
            path,
            filename,
        })
    }
}
