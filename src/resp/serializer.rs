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
