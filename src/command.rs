#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum Command {
    Set { key: String, value: String },
    SetNX { key: String, value: String },
    Get { key: String },
    Del { keys: Vec<String> },
    Multi,
    Exec,
    Abort,
}
