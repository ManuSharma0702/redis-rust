pub type ParseResult = Result<ParseValue, ParseError>;

#[derive(Debug)]
pub struct ParseValue{
    pub result: RespValue,
    pub bytes_read: usize
}

#[derive(Debug)]
pub enum ParseError {
    InvalidInput,
    InvalidLength,
    UnexpectedEof,
    MissingCRLF,
    InvalidRespValue
}

#[derive(Clone,Debug)]
pub enum RespValue{
    SimpleString(Vec<u8>),
    Error(Vec<u8>),
    Integer(i64),
    BulkString(Option<Vec<u8>>),
    Arrays(Option<Vec<RespValue>>)
}

