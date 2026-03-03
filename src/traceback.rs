#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Traceback {
   pub (crate) text: String,
}

impl From<String> for Traceback {
    fn from(text: String) -> Self {
        Self { text }
    }
}

