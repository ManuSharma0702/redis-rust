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

fn read_line(line: &[u8]) -> Option<(&[u8], &[u8])>{
    let mut i = 0;
    while i < line.len(){
        if line[i] == b'\r' && line[i + 1] == b'\n' {
            return Some((&line[..i], &line[i..]));
        }
        i += 1;
    }
    None
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
        b'$' => bulk_string_parser(),
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
    let data = read_line(&input[1..]).expect("Missing CRRL");
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

fn bulk_array_parser(){
    unimplemented!();
}

fn bulk_string_parser(){
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
}












