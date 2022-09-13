#[derive(Debug)]
pub(crate) enum XError {
    EnumParse { _type: &'static str, value: String },
}

impl std::fmt::Display for XError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            XError::EnumParse { _type, value } => {
                write!(f, "Parse {_type} from {value} failed.")
            }
        }
    }
}

impl std::error::Error for XError {}
