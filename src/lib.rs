use std::process::exit;

#[derive(Debug)]
pub enum RespValue{
    SimpleString(Vec<u8>),
    Error(Vec<u8>),
    Integer(i64),
    BulkString(Option<Vec<u8>>),
    Arrays(Option<Vec<RespValue>>)
}

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

fn read_line(line: &[u8], skip_crlf_len: Option<usize>) -> Option<(&[u8], &[u8])>{
    let mut i = 0;
    let skip = skip_crlf_len.unwrap_or(0);
    while i < line.len(){
        if line[i] == b'\r' && line[i + 1] == b'\n' && i >= skip {
            return Some((&line[..i], &line[i..]));
        }
        i += 1;
    }
    None
}

fn read_integer(line: &[u8]) -> (i32, usize) {
    let mut r = 0;
    let mut n = 0;
    while line[r].is_ascii_digit(){
        let digit = (line[r] - b'0') as i32;
        n = n * 10 + digit;
        r += 1;
    }
    r += b"\r\n".len();
    (n, r)
}

///Identifies the data type from request, and calls the corresponding parser
///The input stream is in the form of bytes of vector
fn parse_dispatcher(input: &[u8]) -> &u8 {
    let data_type = input.first().unwrap_or_else(|| {
        eprint!("No input received");
        exit(1);
    });

    match data_type {
        b'+' | b'-' | b':' => {
           _ = simple_parser(input, data_type);
        },
        b'$' => {
            _ = bulk_string_parser(input);
        }
        b'*' => bulk_array_parser(),
        _ => eprint!("Unknown data type")
    }

    data_type
}

fn parse_int(input: &[u8]) -> i64{
    if input.is_empty() {
        eprintln!("Input is empty");
        exit(1);
    }
    let mut sign = 1;
    let mut i = 0;

    match input[0] {
        b'+' => i = 1,
        b'-' => {
            i = 1;
            sign = -1
        },
        _ => i = 1
    } 

    let mut n: i64 = 0;

    while i < input.len(){
        let d = input[i];
        if !d.is_ascii_digit(){
            eprintln!("Not valid digit");
            exit(1);
        } else {
            n = n.checked_mul(10)
                .and_then(|x| x.checked_add((d - b'0') as i64))
                .unwrap_or_else(|| {
                    eprint!("Not a valid integer");
                    exit(1);
                });
        }
        i += 1;
    }
    n * sign
}


fn simple_parser(input: &[u8], data_type: &u8) -> RespValue {
    let data = read_line(&input[1..], None).expect("Missing CFRL");
    match data_type {
        b'+' => {
            RespValue::SimpleString(data.0.to_vec())
        },
        b'-' => {
            RespValue::Error(data.0.to_vec())
        },
        b':' => {
            let val = parse_int(data.0);
            RespValue::Integer(val)
        }
        _ => {
            eprintln!("Unknown data type");
            exit(1);
        }
    }
}

fn bulk_string_parser(input: &[u8]) -> RespValue{
    let (size_of_string, length_of_string) = read_integer(&input[1..]);
    let data = read_line(&input[(length_of_string + 1)..], Some(size_of_string as usize)).expect("Missing CFRL");
    RespValue::BulkString(Some(data.0.to_vec()))
}

fn bulk_array_parser(){
    unimplemented!();
}


#[cfg(test)]
mod tests{
    use super::*;
    
    #[test]
    fn parse_dispatcher_test_string(){
        let input = b"+1\r\n";
        let res = parse_dispatcher(input);
        assert_eq!(*res, b'+');
    }
 
    #[test]
    fn parse_dispatcher_test_integer(){
        let input = b":+1\r\n";
        let res = parse_dispatcher(input);
        assert_eq!(*res, b':');
    }
 
    #[test]
    fn parse_dispatcher_test_error(){
        let input = b"-Error message\r\n";
        let res = parse_dispatcher(input);
        assert_eq!(*res, b'-');
    }

    #[test]
    fn simple_parser_integer_test(){
        let input = b":+1\r\n";
        let res = simple_parser(input, &b':');
        assert_eq!(res, RespValue::Integer(1));
    }

    #[test]
    fn simple_parser_string_test(){
        let input = b"+1\r\n";
        let res = simple_parser(input, &b'+');
        assert_eq!(res, RespValue::SimpleString(vec![b'1']))
    }

    #[test]
    fn simple_parser_error_test(){
        let input = b"-Error message\r\n";
        let res = simple_parser(input, &b'-');
        assert_eq!(res, RespValue::Error(b"Error message".to_vec()));
    }

    #[test]
    fn bulk_string_parse_test(){
        let input = b"$5\r\nHello\r\n";
        let res = bulk_string_parser(input);
        assert_eq!(res, RespValue::BulkString(Some(b"Hello".to_vec())));
    }

    #[test]
    fn bulk_string_parse_test2(){
        let input = b"$9\r\nHe\rl\nl\r\no\r\n";
        let res = bulk_string_parser(input);
        assert_eq!(res, RespValue::BulkString(Some(b"He\rl\nl\r\no".to_vec())));
    }
}












