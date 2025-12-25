#[derive(Debug)]
pub enum Commands {
    PING,
}

#[derive(Debug)]
pub enum CommandError{
    ParseFailed,
    InvalidRequest,
    UnknownCommand
}

