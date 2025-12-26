#[derive(Debug, PartialEq)]
pub enum Commands {
    PING,
    ECHO,
    SET,
    GET
}

#[derive(Debug, PartialEq)]
pub enum CommandError{
    ParseFailed,
    InvalidRequest,
    UnknownCommand
}

