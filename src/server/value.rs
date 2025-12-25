use crate::{command::CommandError, resp::ParseError};

pub enum ServerError {
    Command(CommandError),
    Parse(ParseError),
}
