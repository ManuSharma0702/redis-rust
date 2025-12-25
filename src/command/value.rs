#[derive(Debug)]
pub enum Commands {
    PING,
    ECHO,
    SET,
    GET
}

#[derive(Debug)]
pub enum CommandError{
    ParseFailed,
    InvalidRequest,
    UnknownCommand
}

