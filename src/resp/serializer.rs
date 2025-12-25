use crate::resp::ParseError;
use super::value::RespValue;

pub fn serializer(value: &RespValue) -> Result<Vec<u8>, ParseError> {

    match value {
        RespValue::SimpleString(v) => {
            let mut out = vec![b'+'];
            out.extend(v);
            out.extend(b"\r\n");
            Ok(out)
        },
        RespValue::Error(v) => {
            let mut out = vec![b'-'];
            out.extend(v);
            out.extend(b"\r\n");
            Ok(out)
        }
        RespValue::Integer(v) => {
            Ok(format!(":{}\r\n", v).into_bytes())
        },
        RespValue::BulkString(Some(v)) => {
            let mut out = format!("${}\r\n", v.len()).into_bytes();
            out.extend(v);
            out.extend(b"\r\n");
            Ok(out)
        },
        RespValue::BulkString(None) => {
            Ok(b"$-1\r\n".to_vec())
        },
        RespValue::Arrays(Some(arr)) => {
            let mut out = format!("*{}\r\n", arr.len()).into_bytes();
            for elem in arr{
                out.extend(serializer(elem)?);
            }
            Ok(out)
        },
        RespValue::Arrays(None) => {
            Ok(b"*-1\r\n".to_vec())
        }
        
    }

}


#[cfg(test)]
mod tests{
    use super::*;
    use crate::resp::value::RespValue;

    #[test]
    fn serialize_simple_string() {
        let value = RespValue::SimpleString(b"OK".to_vec());
        let res = serializer(&value).unwrap();
        assert_eq!(res, b"+OK\r\n".to_vec());
    }

    #[test]
    fn serialize_error() {
        let value = RespValue::Error(b"ERR".to_vec());
        let res = serializer(&value).unwrap();
        assert_eq!(res, b"-ERR\r\n".to_vec());
    }

    #[test]
    fn serialize_integer() {
        let value = RespValue::Integer(42);
        let res = serializer(&value).unwrap();
        assert_eq!(res, b":42\r\n".to_vec());
    }

    #[test]
    fn serialize_integer_negative() {
        let value = RespValue::Integer(-123);
        let res = serializer(&value).unwrap();
        assert_eq!(res, b":-123\r\n".to_vec());
    }

    #[test]
    fn serialize_bulk_string_some() {
        let value = RespValue::BulkString(Some(b"hello".to_vec()));
        let res = serializer(&value).unwrap();
        assert_eq!(res, b"$5\r\nhello\r\n".to_vec());
    }

    #[test]
    fn serialize_bulk_string_none() {
        let value = RespValue::BulkString(None);
        let res = serializer(&value).unwrap();
        assert_eq!(res, b"$-1\r\n".to_vec());
    }

    #[test]
    fn serialize_empty_bulk_string() {
        let value = RespValue::BulkString(Some(vec![]));
        let res = serializer(&value).unwrap();
        assert_eq!(res, b"$0\r\n\r\n".to_vec());
    }

    #[test]
    fn serialize_array_simple() {
        let value = RespValue::Arrays(Some(vec![
            RespValue::Integer(1),
            RespValue::Integer(2),
        ]));
        let res = serializer(&value).unwrap();
        assert_eq!(res, b"*2\r\n:1\r\n:2\r\n".to_vec());
    }

    #[test]
    fn serialize_array_with_bulk_strings() {
        let value = RespValue::Arrays(Some(vec![
            RespValue::BulkString(Some(b"foo".to_vec())),
            RespValue::BulkString(Some(b"bar".to_vec())),
        ]));
        let res = serializer(&value).unwrap();
        assert_eq!(res, b"*2\r\n$3\r\nfoo\r\n$3\r\nbar\r\n".to_vec());
    }

    #[test]
    fn serialize_array_nested() {
        let value = RespValue::Arrays(Some(vec![
            RespValue::BulkString(Some(b"foo".to_vec())),
            RespValue::Arrays(Some(vec![
                RespValue::Integer(1),
                RespValue::Integer(2),
            ])),
            RespValue::BulkString(Some(b"bar".to_vec())),
        ]));
        let res = serializer(&value).unwrap();
        assert_eq!(
            res,
            b"*3\r\n$3\r\nfoo\r\n*2\r\n:1\r\n:2\r\n$3\r\nbar\r\n".to_vec()
        );
    }

    #[test]
    fn serialize_array_with_nulls() {
        let value = RespValue::Arrays(Some(vec![
            RespValue::BulkString(None),
            RespValue::Arrays(None),
        ]));
        let res = serializer(&value).unwrap();
        assert_eq!(res, b"*2\r\n$-1\r\n*-1\r\n".to_vec());
    }

    #[test]
    fn serialize_array_none() {
        let value = RespValue::Arrays(None);
        let res = serializer(&value).unwrap();
        assert_eq!(res, b"*-1\r\n".to_vec());
    }

    #[test]
    fn serialize_complex_nested() {
        let value = RespValue::Arrays(Some(vec![
            RespValue::Integer(1),
            RespValue::Arrays(Some(vec![
                RespValue::BulkString(Some(b"x".to_vec())),
                RespValue::BulkString(None),
            ])),
            RespValue::SimpleString(b"OK".to_vec()),
        ]));
        let res = serializer(&value).unwrap();
        assert_eq!(
            res,
            b"*3\r\n:1\r\n*2\r\n$1\r\nx\r\n$-1\r\n+OK\r\n".to_vec()
        );
    }

}
