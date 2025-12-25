use super::value::*;

impl PartialEq for RespValue{
    fn eq(&self, other: &Self) -> bool {
        use RespValue::*;

        match (self, other) {
            (SimpleString(a), SimpleString(b)) => a == b,
            (Error(a), Error(b)) => a == b,
            (Integer(a), Integer(b)) => a == b,
            (BulkString(Some(a)), BulkString(Some(b))) => a == b,
            (BulkString(None), BulkString(None)) => true,
            (Arrays(Some(a)), Arrays(Some(b))) => a == b,
            (Arrays(None), Arrays(None)) => true,
            _ => false
        }

    }
}

fn read_line(line: &[u8], min_len_before_crlf: usize) -> Result<(&[u8], &[u8]), ParseError> {
    let mut i = 0;
    
    if line.len() < min_len_before_crlf {
        return Err(ParseError::MissingCRLF);
    }

    while i + 1< line.len(){
        if line[i] == b'\r' && line[i + 1] == b'\n' && i >= min_len_before_crlf {
            //skip the CLRF token in the remaining returned
            return Ok((&line[..i], &line[i+2..]));
        }
        i += 1;
    }
    Err(ParseError::MissingCRLF)
}


fn read_integer(line: &[u8]) -> Result<(i64, usize), ParseError> {
    if line.is_empty() {
        return Err(ParseError::UnexpectedEof);
    }
    let mut idx = 0;
    let mut sign = 1;

    if line[idx] == b'-' {
        sign = -1;
        idx += 1;
    } else if line[idx] == b'+' {
        idx += 1;
    }

    let mut value: i64 = 0;

    //No digit present
    if idx >= line.len(){
        return Err(ParseError::UnexpectedEof);
    }

    while idx < line.len() {
        if !line[idx].is_ascii_digit() {
            return Err(ParseError::InvalidLength);
        }
        value = value * 10 + (line[idx] - b'0') as i64;
        idx += 1;
    }

    Ok((sign * value, idx))
}


///Identifies the data type from request, and calls the corresponding parser
///The input stream is in the form of bytes of vector
///Need this to return RespValue as well as the bytes consumed
pub fn parse_dispatcher(input: &[u8]) -> ParseResult {
    let data_type = match input.first() {
        Some(n) => n,
        None => return Err(ParseError::InvalidInput)
    };

    match data_type {
        b'+' | b'-' | b':' => {
            simple_parser(input, data_type)
        },
        b'$' => {
            bulk_string_parser(input)
        }
        b'*' => {
            bulk_array_parser(input)
        },
        _ => {
            eprintln!("Simple Parser: Unknown data type");
            Err(ParseError::InvalidInput)
        }
    }
}

fn simple_parser(input: &[u8], data_type: &u8) -> ParseResult {
    let data = read_line(&input[1..], 0)?;
    match data_type {
        b'+' => {
            Ok(ParseValue{
                result: RespValue::SimpleString(data.0.to_vec()),
                bytes_read: input.len() - data.1.len()
            })
        },
        b'-' => {
            Ok(ParseValue{
                result: RespValue::Error(data.0.to_vec()),
                bytes_read: input.len() - data.1.len()
            })
        },
        b':' => {
            let (val, _) = read_integer(data.0)?;
            dbg!(val);
            Ok(ParseValue{
                result: RespValue::Integer(val),
                bytes_read: input.len() - data.1.len()
            })
        }
        _ => {
            eprintln!("Unknown data type");
            Err(ParseError::InvalidInput)
        }
    }
}

fn bulk_string_parser(input: &[u8]) -> ParseResult {
    let size_input = read_line(&input[1..], 0)?;
    let (size_of_string, mut length_of_string) = read_integer(size_input.0)?;       
    length_of_string += 2;
    if size_of_string < 0{
        return Ok(ParseValue{
            result: RespValue::BulkString(None),
            bytes_read: length_of_string + 1
        });
    }

    let data = read_line(&input[(length_of_string + 1)..], size_of_string as usize)?;

    if data.0.len() != size_of_string as usize {
        return Err(ParseError::InvalidLength);
    }
  
    Ok(ParseValue{
        result: RespValue::BulkString(Some(data.0.to_vec())),
        bytes_read: input.len() - data.1.len()
    })
}

fn bulk_array_parser(input: &[u8]) -> ParseResult { 
    let size_input = read_line(&input[1..], 0)?;
    let (size_of_array, mut length_of_size) = read_integer(size_input.0)?;
    length_of_size += 2;
    if size_of_array < 0{
        return Ok(ParseValue{
            result: RespValue::Arrays(None),
            bytes_read: length_of_size + 1
        });
    }

    let mut curr_input = &input[(length_of_size + 1)..];

    let mut bulk_array = Vec::new();

    let mut total_bytes_read = length_of_size + 1;

    for _ in 0..size_of_array {

        if curr_input.is_empty() {
            return Err(ParseError::UnexpectedEof);
        }
        
        let curr_element = parse_dispatcher(curr_input)?;

        bulk_array.push(curr_element.result);
        total_bytes_read += curr_element.bytes_read;
        curr_input = input
            .get(total_bytes_read..)
            .ok_or(ParseError::UnexpectedEof)?;
    }

    Ok(ParseValue{
        result: RespValue::Arrays(Some(bulk_array)),
        bytes_read: total_bytes_read
    })
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn simple_parser_integer_test(){
        let input = b":+2\r\n";
        let res = simple_parser(input, &b':');
        assert_eq!(res.unwrap().result, RespValue::Integer(2));
    }

    #[test]
    fn simple_parser_integer_test1(){
        let input = b":1\r\n";
        let res = simple_parser(input, &b':');
        assert_eq!(res.unwrap().result, RespValue::Integer(1));
    }

    #[test]
    fn simple_parser_string_test(){
        let input = b"+1\r\n";
        let res = simple_parser(input, &b'+');
        assert_eq!(res.unwrap().result, RespValue::SimpleString(vec![b'1']))
    }

    #[test]
    fn simple_parser_error_test(){
        let input = b"-Error message\r\n";
        let res = simple_parser(input, &b'-');
        assert_eq!(res.unwrap().result, RespValue::Error(b"Error message".to_vec()));
    }

    #[test]
    fn bulk_string_parse_test(){
        let input = b"$5\r\nHello\r\n";
        let res = bulk_string_parser(input);
        assert_eq!(res.unwrap().result, RespValue::BulkString(Some(b"Hello".to_vec())));
    }

    #[test]
    fn bulk_string_parse_test2(){
        let input = b"$9\r\nHe\rl\nl\r\no\r\n";
        let res = bulk_string_parser(input);
        assert_eq!(res.unwrap().result, RespValue::BulkString(Some(b"He\rl\nl\r\no".to_vec())));
    }

    #[test]
    fn bulk_array_parse_test(){
        let input = b"*2\r\n$5\r\nhello\r\n$5\r\nworld\r\n";
        let res = bulk_array_parser(input);
        let result = RespValue::Arrays(Some(vec![
            RespValue::BulkString(Some(b"hello".to_vec())),
            RespValue::BulkString(Some(b"world".to_vec())),
        ]));
        assert_eq!(res.unwrap().result, result);
    }

    #[test]
    fn bulk_array_empty() {
        let input = b"*0\r\n";
        let res = bulk_array_parser(input);
        let result = RespValue::Arrays(Some(vec![]));
        assert_eq!(res.unwrap().result, result);
    }

    #[test]
    fn bulk_array_single_element() {
        let input = b"*1\r\n$3\r\nfoo\r\n";
        let res = bulk_array_parser(input);
        let result = RespValue::Arrays(Some(vec![
            RespValue::BulkString(Some(b"foo".to_vec())),
        ]));
        assert_eq!(res.unwrap().result, result);
    }


    #[test]
    fn bulk_array_null() {
        let input = b"*-1\r\n";
        let res = bulk_array_parser(input);
        let result = RespValue::Arrays(None);
        assert_eq!(res.unwrap().result, result);
    }
    #[test]
    fn bulk_array_with_null_bulk_string() {
        let input = b"*2\r\n$3\r\nfoo\r\n$-1\r\n";
        let res = bulk_array_parser(input);
        let result = RespValue::Arrays(Some(vec![
            RespValue::BulkString(Some(b"foo".to_vec())),
            RespValue::BulkString(None),
        ]));
        assert_eq!(res.unwrap().result, result);
    }
    #[test]
    fn bulk_array_empty_bulk_string() {
        let input = b"*1\r\n$0\r\n\r\n";
        let res = bulk_array_parser(input);
        let result = RespValue::Arrays(Some(vec![
            RespValue::BulkString(Some(Vec::new())),
        ]));
        assert_eq!(res.unwrap().result, result);
    }
    #[test]
    fn bulk_array_many_elements() {
        let input = b"*3\r\n$1\r\na\r\n$1\r\nb\r\n$1\r\nc\r\n";
        let res = bulk_array_parser(input);
        let result = RespValue::Arrays(Some(vec![
            RespValue::BulkString(Some(b"a".to_vec())),
            RespValue::BulkString(Some(b"b".to_vec())),
            RespValue::BulkString(Some(b"c".to_vec())),
        ]));
        assert_eq!(res.unwrap().result, result);
    }
    #[test]
    fn bulk_array_length_mismatch() {
        let input = b"*2\r\n$3\r\nfoo\r\n";
        let res = bulk_array_parser(input);
        assert!(res.is_err());
    }

    #[test]
    fn read_integer_negative() {
        let (v, n) = read_integer(b"-1").unwrap();
        assert_eq!(v, -1);
        assert_eq!(n, 2);
    }

    #[test]
    fn read_integer_positive_with_plus() {
        let (v, n) = read_integer(b"+42").unwrap();
        assert_eq!(v, 42);
        assert_eq!(n, 3);
    }

    #[test]
    fn read_integer_zero() {
        let (v, n) = read_integer(b"0").unwrap();
        assert_eq!(v, 0);
        assert_eq!(n, 1);
    }

    #[test]
    fn parse_dispatcher_simple_string() {
        let input = b"+OK\r\n";
        let res = parse_dispatcher(input);
        assert_eq!(res.unwrap().result, RespValue::SimpleString(b"OK".to_vec()));
    }

    #[test]
    fn parse_dispatcher_integer() {
        let input = b":-10\r\n";
        let res = parse_dispatcher(input);
        assert_eq!(res.unwrap().result, RespValue::Integer(-10));
    }

    #[test]
    fn parse_dispatcher_bulk_string() {
        let input = b"$4\r\ntest\r\n";
        let res = parse_dispatcher(input);
        assert_eq!(
            res.unwrap().result,
            RespValue::BulkString(Some(b"test".to_vec()))
        );
    }

    #[test]
    fn parse_dispatcher_array() {
        let input = b"*1\r\n$3\r\nfoo\r\n";
        let res = parse_dispatcher(input);
        assert_eq!(
            res.unwrap().result,
            RespValue::Arrays(Some(vec![
                RespValue::BulkString(Some(b"foo".to_vec()))
            ]))
        );
    }
    #[test]
    fn bulk_array_nested_mixed() {
        let input = b"*3\r\n$3\r\nfoo\r\n*2\r\n:1\r\n:2\r\n$3\r\nbar\r\n";
        let res = bulk_array_parser(input);

        let expected = RespValue::Arrays(Some(vec![
            RespValue::BulkString(Some(b"foo".to_vec())),
            RespValue::Arrays(Some(vec![
                RespValue::Integer(1),
                RespValue::Integer(2),
            ])),
            RespValue::BulkString(Some(b"bar".to_vec())),
        ]));

        assert_eq!(res.unwrap().result, expected);
    }

    #[test]
    fn bulk_array_nested_empty() {
        let input = b"*1\r\n*0\r\n";
        let res = bulk_array_parser(input);

        let expected = RespValue::Arrays(Some(vec![
            RespValue::Arrays(Some(vec![])),
        ]));

        assert_eq!(res.unwrap().result, expected);
    }

    #[test]
    fn bulk_array_with_null_array() {
        let input = b"*2\r\n*-1\r\n$3\r\nfoo\r\n";
        let res = bulk_array_parser(input);

        let expected = RespValue::Arrays(Some(vec![
            RespValue::Arrays(None),
            RespValue::BulkString(Some(b"foo".to_vec())),
        ]));

        assert_eq!(res.unwrap().result, expected);
    }

    #[test]
    fn bulk_array_null_nested() {
        let input = b"*1\r\n*1\r\n*-1\r\n";
        let res = bulk_array_parser(input);

        let expected = RespValue::Arrays(Some(vec![
            RespValue::Arrays(Some(vec![
                RespValue::Arrays(None),
            ])),
        ]));

        assert_eq!(res.unwrap().result, expected);
    }

    #[test]
    fn integer_plus_zero() {
        let input = b":+0\r\n";
        let res = simple_parser(input, &b':');
        assert_eq!(res.unwrap().result, RespValue::Integer(0));
    }

    #[test]
    fn integer_large_value() {
        let input = b":2147483647\r\n";
        let res = simple_parser(input, &b':');
        assert_eq!(res.unwrap().result, RespValue::Integer(2147483647));
    }

    #[test]
    fn bulk_string_truncated() {
        let input = b"$5\r\nHell";
        let res = bulk_string_parser(input);
        assert!(res.is_err());
    }
    #[test]
    fn bulk_string_invalid_length() {
        let input = b"$x\r\nabc\r\n";
        let res = bulk_string_parser(input);
        assert!(res.is_err());
    }
    #[test]
    fn bulk_string_parser_invalid(){
        let input = b"$3\r\nabcd\r\n";
        let res = bulk_string_parser(input);
        assert!(res.is_err());
    }
    #[test]
    fn bulk_array_invalid_length() {
        let input = b"*x\r\n";
        let res = bulk_array_parser(input);
        assert!(res.is_err());
    }
    #[test]
    fn bulk_array_nested_eof() {
        let input = b"*2\r\n$3\r\nfoo\r\n*2\r\n:1\r\n";
        let res = bulk_array_parser(input);
        assert!(res.is_err());
    }
    #[test]
    fn integer_missing_crlf() {
        let input = b":123\n";
        let res = simple_parser(input, &b':');
        assert!(res.is_err());
    }
    #[test]
    fn bulk_string_missing_crlf_after_length() {
        let input = b"$3\nabc\r\n";
        let res = bulk_string_parser(input);
        assert!(res.is_err());
    }
    #[test]
    fn bulk_array_bytes_read_exact() {
        let input = b"*1\r\n$3\r\nfoo\r\n";
        let res = bulk_array_parser(input).unwrap();
        assert_eq!(res.bytes_read, input.len());
    }

    #[test]
    fn bulk_array_numbers() {
        let input = b"*2\r\n:1\r\n:2\r\n";
        let res = bulk_array_parser(input).unwrap();

        let expected = RespValue::Arrays(Some(vec![
            RespValue::Integer(1),
            RespValue::Integer(2),
        ]));

        assert_eq!(res.result, expected);
    }

    
    #[test]
    fn read_integer_invalid_input(){
        let input = b"+";
        let res = read_integer(input);
        assert!(res.is_err());
    }

    #[test]
    fn bulk_array_parser_ping_input(){
        let input = b"*1\r\n$4\r\nPING\r\n";
        let res = bulk_array_parser(input).unwrap();
        let expected = RespValue::Arrays(Some(vec![
                RespValue::BulkString(Some(b"PING".to_vec()))
        ]));
        assert_eq!(res.result, expected);
    }
}













