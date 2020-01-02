#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum Command {
    Set { key: String, value: String },
    SetNX { key: String, value: String },
    Get { key: String },
    Del { keys: Vec<String> },
    Multi,
    Exec,
    Abort,
    Shutdown,
}

impl ToString for Command {
    fn to_string(&self) -> String {
        use self::Command::*;

        match self {
            Set { key, value } => format!("set {} {}", key, value),
            SetNX { key, value } => format!("setnx {} {}", key, value),
            Get { key } => format!("get {}", key),
            Del { .. } => todo!(),
            _ => todo!(),
        }
    }
}
