use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct MyeetErr {
    message: Option<String>
}

impl MyeetErr {
    pub fn with_text(text: &str) -> MyeetErr {
        MyeetErr {
            message: Some(String::from(text))
        }
    }
}

impl Error for MyeetErr {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None    
    }
}

impl fmt::Display for MyeetErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(s) = &self.message {
            write!(f, "{}", s)
        }
        else {
            write!(f, "no error info")
        }
    }
}
