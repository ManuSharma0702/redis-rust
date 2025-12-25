#[derive(Debug)]
pub enum Commands {
    PING,
    ECHO,
}

#[derive(Debug)]
pub enum CommandError{
    ParseFailed,
    InvalidRequest,
    UnknownCommand
}

