#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

#[cfg(not(feature = "std"))]
use alloc::string::ToString;

use crate::traits::Serialize;

#[cfg_attr(feature = "std", derive(Debug))]
#[derive(Clone, PartialEq, Copy)]
pub struct NumberValue {
    pub integer: u128,
    pub fraction: u128,
    pub fraction_length: u32,
    pub exponent: i32,
    pub negative: bool,
}

impl NumberValue {
    /// Losslessly convert the inner value to `f64`.
    #[cfg(any(feature = "std", feature = "float"))]
    pub fn to_f64(self) -> f64 {
        self.into()
    }
}

#[cfg(any(feature = "std", feature = "float"))]
impl Into<f64> for NumberValue {
    fn into(self) -> f64 {
        #[cfg(not(feature = "std"))]
        use num_traits::float::FloatCore as _;

        let sign = if self.negative { -1.0 } else { 1.0 };
        (self.integer as f64 + self.fraction as f64 / 10f64.powi(self.fraction_length as i32))
            * 10f64.powi(self.exponent)
            * sign
    }
}

pub type JsonObject = Vec<(Vec<char>, JsonValue)>;

#[cfg_attr(feature = "std", derive(Debug))]
#[derive(Clone, PartialEq)]
pub enum JsonValue {
    Object(JsonObject),
    Array(Vec<JsonValue>),
    String(Vec<char>),
    Number(NumberValue),
    Boolean(bool),
    Null,
}

impl JsonValue {
    /// Returns a boolean indicating whether this value is an object or not.
    pub fn is_object(&self) -> bool {
        match self {
            JsonValue::Object(_) => true,
            _ => false,
        }
    }

    /// Returns a reference to the key-value vec if this value is an object, otherwise returns None.
    pub fn as_object(&self) -> Option<&[(Vec<char>, JsonValue)]> {
        match self {
            JsonValue::Object(obj) => Some(obj),
            _ => None,
        }
    }

    /// Returns the wrapped object if the value is an object, otherwise returns None.
    pub fn to_object(self) -> Option<JsonObject> {
        match self {
            JsonValue::Object(obj) => Some(obj),
            _ => None,
        }
    }

    /// Returns a boolean indicating whether this value is an array or not.
    pub fn is_array(&self) -> bool {
        match self {
            JsonValue::Array(_) => true,
            _ => false,
        }
    }

    /// Returns a reference to the wrapped array if this value is an array, otherwise returns None.
    pub fn as_array(&self) -> Option<&[JsonValue]> {
        match self {
            JsonValue::Array(arr) => Some(arr),
            _ => None,
        }
    }

    /// Returns the wrapped vector if the value is an array, otherwise returns None.
    pub fn to_array(self) -> Option<Vec<JsonValue>> {
        match self {
            JsonValue::Array(arr) => Some(arr),
            _ => None,
        }
    }

    /// Returns a boolean indicating whether this value is a string or not.
    pub fn is_string(&self) -> bool {
        match self {
            JsonValue::String(_) => true,
            _ => false,
        }
    }

    /// Returns a char slice if this value is a string, otherwise returns None.
    pub fn as_string(&self) -> Option<&[char]> {
        match self {
            JsonValue::String(s) => Some(s),
            _ => None,
        }
    }

    /// Returns the wrapped vector if the value is a string, otherwise returns None.
    pub fn to_string(self) -> Option<Vec<char>> {
        match self {
            JsonValue::String(s) => Some(s),
            _ => None,
        }
    }

    /// Returns a boolean indicating whether this value is a number or not.
    pub fn is_number(&self) -> bool {
        match self {
            JsonValue::Number(_) => true,
            _ => false,
        }
    }

    /// Returns a reference to wrapped `NumberValue` if this value is a number, otherwise returns None.
    pub fn as_number(&self) -> Option<&NumberValue> {
        match self {
            JsonValue::Number(n) => Some(n),
            _ => None,
        }
    }

    /// Returns the wrapped NumberValue if the value is a number, otherwise returns None.
    pub fn to_number(self) -> Option<NumberValue> {
        match self {
            JsonValue::Number(n) => Some(n),
            _ => None,
        }
    }

    /// Returns a boolean indicating whether this value is a boolean or not.
    pub fn is_bool(&self) -> bool {
        match self {
            JsonValue::Boolean(_) => true,
            _ => false,
        }
    }

    /// Returns a reference to the wrapped boolean if this value is a boolean, otherwise returns None.
    pub fn as_bool(&self) -> Option<&bool> {
        match self {
            JsonValue::Boolean(b) => Some(b),
            _ => None,
        }
    }

    /// Returns the wrapped boolean if the value is a boolean, otherwise returns None.
    pub fn to_bool(self) -> Option<bool> {
        match self {
            JsonValue::Boolean(b) => Some(b),
            _ => None,
        }
    }

    /// Returns a boolean indicating whether this value is null or not.
    pub fn is_null(&self) -> bool {
        match self {
            JsonValue::Null => true,
            _ => false,
        }
    }
}

impl Serialize for NumberValue {
    fn serialize_to(&self, buffer: &mut Vec<u8>, _indent: u32, _level: u32) {
        if self.negative {
            buffer.push(b'-');
        }
        buffer.extend_from_slice(self.integer.to_string().as_bytes());

        if self.fraction > 0 {
            buffer.push(b'.');

            let fraction_nums = self.fraction.to_string();
            let fraction_length = self.fraction_length as usize;
            for _ in 0..fraction_length - fraction_nums.len() {
                buffer.push(b'0');
            }
            buffer.extend_from_slice(fraction_nums.as_bytes())
        }
        if self.exponent != 0 {
            buffer.push(b'e');
            if self.exponent < 0 {
                buffer.push(b'-');
            }
            buffer.extend_from_slice(self.exponent.abs().to_string().as_bytes());
        }
    }
}

fn push_string(buffer: &mut Vec<u8>, chars: &Vec<char>) {
    buffer.push('"' as u8);
    for ch in chars {
        match ch {
            '\x08' => buffer.extend_from_slice(br#"\b"#),
            '\x0c' => buffer.extend_from_slice(br#"\f"#),
            '\n' => buffer.extend_from_slice(br#"\n"#),
            '\r' => buffer.extend_from_slice(br#"\r"#),
            '\t' => buffer.extend_from_slice(br#"\t"#),
            '\"' => buffer.extend_from_slice(br#"\""#),
            '\\' => buffer.extend_from_slice(br#"\\"#),
            _ => match ch.len_utf8() {
                1 => {
                    let mut buff = [0u8; 1];
                    ch.encode_utf8(&mut buff);
                    buffer.push(buff[0]);
                }
                2 => {
                    let mut buff = [0u8; 2];
                    ch.encode_utf8(&mut buff);
                    buffer.extend_from_slice(&buff);
                }
                3 => {
                    let mut buff = [0u8; 3];
                    ch.encode_utf8(&mut buff);
                    buffer.extend_from_slice(&buff);
                }
                4 => {
                    let mut buff = [0u8; 4];
                    ch.encode_utf8(&mut buff);
                    buffer.extend_from_slice(&buff);
                }
                _ => panic!("Invalid UTF8 character"),
            },
        }
    }
    buffer.push('"' as u8);
}

fn push_new_line_indent(buffer: &mut Vec<u8>, indent: u32, level: u32) {
    if indent > 0 {
        buffer.push('\n' as u8);
    }
    let count = (indent * level) as usize;
    buffer.reserve(count);
    for _ in 0..count {
        buffer.push(' ' as u8);
    }
}

impl Serialize for JsonValue {
    fn serialize_to(&self, buffer: &mut Vec<u8>, indent: u32, level: u32) {
        match self {
            JsonValue::Object(obj) => {
                buffer.push('{' as u8);
                if obj.len() > 0 {
                    push_new_line_indent(buffer, indent, level + 1);
                    push_string(buffer, &obj[0].0);
                    buffer.push(':' as u8);
                    if indent > 0 {
                        buffer.push(' ' as u8);
                    }
                    obj[0].1.serialize_to(buffer, indent, level + 1);
                    for (key, val) in obj.iter().skip(1) {
                        buffer.push(',' as u8);
                        push_new_line_indent(buffer, indent, level + 1);
                        push_string(buffer, key);
                        buffer.push(':' as u8);
                        if indent > 0 {
                            buffer.push(' ' as u8);
                        }
                        val.serialize_to(buffer, indent, level + 1);
                    }
                    push_new_line_indent(buffer, indent, level);
                    buffer.push('}' as u8);
                } else {
                    buffer.push('}' as u8);
                }
            }
            JsonValue::Array(arr) => {
                buffer.push('[' as u8);
                if arr.len() > 0 {
                    push_new_line_indent(buffer, indent, level + 1);
                    arr[0].serialize_to(buffer, indent, level + 1);
                    for val in arr.iter().skip(1) {
                        buffer.push(',' as u8);
                        push_new_line_indent(buffer, indent, level + 1);
                        val.serialize_to(buffer, indent, level);
                    }
                    push_new_line_indent(buffer, indent, level);
                    buffer.push(']' as u8);
                } else {
                    buffer.push(']' as u8);
                }
            }
            JsonValue::String(str) => push_string(buffer, str),
            JsonValue::Number(num) => num.serialize_to(buffer, indent, level),
            JsonValue::Boolean(true) => buffer.extend_from_slice(b"true"),
            JsonValue::Boolean(false) => buffer.extend_from_slice(b"false"),
            JsonValue::Null => buffer.extend_from_slice(b"null"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn json_value_convenience_methods() {
        let obj = JsonValue::Object(vec![(vec![], JsonValue::Null)]);
        assert!(obj.is_object());
        assert_eq!(obj.as_object(), Some(&[(vec![], JsonValue::Null)][..]));
        assert_eq!(obj.as_array(), None);
        assert_eq!(obj.as_bool(), None);
        assert_eq!(obj.as_number(), None);
        assert_eq!(obj.as_string(), None);
        assert_eq!(
            obj.clone().to_object(),
            Some(vec![(vec![], JsonValue::Null)]),
        );
        assert_eq!(obj.clone().to_array(), None);
        assert_eq!(obj.clone().to_bool(), None);
        assert_eq!(obj.clone().to_number(), None);
        assert_eq!(obj.clone().to_string(), None);

        let arr = JsonValue::Array(vec![JsonValue::Null]);
        assert!(arr.is_array());
        assert_eq!(arr.as_array(), Some(&[JsonValue::Null][..]));
        assert_eq!(arr.as_object(), None);
        assert_eq!(arr.as_bool(), None);
        assert_eq!(arr.as_number(), None);
        assert_eq!(arr.as_string(), None);
        assert_eq!(arr.clone().to_array(), Some(vec![JsonValue::Null]));
        assert_eq!(arr.clone().to_object(), None);
        assert_eq!(arr.clone().to_bool(), None);
        assert_eq!(arr.clone().to_number(), None);
        assert_eq!(arr.clone().to_string(), None);

        let s = JsonValue::String(vec!['a']);
        assert!(s.is_string());
        assert_eq!(s.as_string(), Some(&['a'][..]));
        assert_eq!(s.as_object(), None);
        assert_eq!(s.as_bool(), None);
        assert_eq!(s.as_number(), None);
        assert_eq!(s.as_array(), None);
        assert_eq!(s.clone().to_string(), Some(vec!['a']));
        assert_eq!(s.clone().to_object(), None);
        assert_eq!(s.clone().to_bool(), None);
        assert_eq!(s.clone().to_number(), None);
        assert_eq!(s.clone().to_array(), None);

        let n = JsonValue::Number(NumberValue {
            integer: 0,
            fraction: 0,
            fraction_length: 0,
            exponent: 0,
            negative: false,
        });
        assert!(n.is_number());
        assert_eq!(
            n.as_number(),
            Some(&NumberValue {
                integer: 0,
                fraction: 0,
                fraction_length: 0,
                exponent: 0,
                negative: false,
            }),
        );
        assert_eq!(n.as_object(), None);
        assert_eq!(n.as_bool(), None);
        assert_eq!(n.as_string(), None);
        assert_eq!(n.as_array(), None);
        assert_eq!(
            n.clone().to_number(),
            Some(NumberValue {
                integer: 0,
                fraction: 0,
                fraction_length: 0,
                exponent: 0,
                negative: false,
            }),
        );
        assert_eq!(n.clone().to_object(), None);
        assert_eq!(n.clone().to_bool(), None);
        assert_eq!(n.clone().to_string(), None);
        assert_eq!(n.clone().to_array(), None);

        let b = JsonValue::Boolean(false);
        assert!(b.is_bool());
        assert_eq!(b.as_bool(), Some(&false));
        assert_eq!(b.as_object(), None);
        assert_eq!(b.as_number(), None);
        assert_eq!(b.as_string(), None);
        assert_eq!(b.as_array(), None);
        assert_eq!(b.clone().to_bool(), Some(false));
        assert_eq!(b.clone().to_object(), None);
        assert_eq!(b.clone().to_number(), None);
        assert_eq!(b.clone().to_string(), None);
        assert_eq!(b.clone().to_array(), None);

        let null = JsonValue::Null;
        assert!(null.is_null());
        assert_eq!(null.as_array(), None);
        assert_eq!(null.as_bool(), None);
        assert_eq!(null.as_number(), None);
        assert_eq!(null.as_object(), None);
        assert_eq!(null.as_string(), None);
        assert_eq!(null.clone().to_array(), None);
        assert_eq!(null.clone().to_bool(), None);
        assert_eq!(null.clone().to_number(), None);
        assert_eq!(null.clone().to_object(), None);
        assert_eq!(null.clone().to_string(), None);
    }

    #[test]
    fn serialize_number_value() {
        let val = NumberValue {
            integer: 1234,
            fraction: 0,
            fraction_length: 0,
            exponent: 0,
            negative: false,
        };
        assert_eq!(val.serialize(), b"1234");

        let val = NumberValue {
            integer: 1234,
            fraction: 0,
            fraction_length: 0,
            exponent: 0,
            negative: true,
        };
        assert_eq!(val.serialize(), b"-1234");

        let val = NumberValue {
            integer: 1234,
            fraction: 5678,
            fraction_length: 4,
            exponent: 0,
            negative: true,
        };
        assert_eq!(val.serialize(), b"-1234.5678");

        let val = NumberValue {
            integer: 1234,
            fraction: 1,
            fraction_length: 3,
            exponent: 0,
            negative: false,
        };
        assert_eq!(val.serialize(), b"1234.001");

        let val = NumberValue {
            integer: 1234,
            fraction: 0,
            fraction_length: 0,
            exponent: 3,
            negative: false,
        };
        assert_eq!(val.serialize(), b"1234e3");

        let val = NumberValue {
            integer: 1234,
            fraction: 0,
            fraction_length: 0,
            exponent: -5,
            negative: false,
        };
        assert_eq!(val.serialize(), b"1234e-5");

        let val = NumberValue {
            integer: 1234,
            fraction: 56,
            fraction_length: 4,
            exponent: -5,
            negative: false,
        };
        assert_eq!(val.serialize(), b"1234.0056e-5");

        let val = NumberValue {
            integer: 1234,
            fraction: 5,
            fraction_length: 2,
            exponent: 5,
            negative: true,
        };
        assert_eq!(val.serialize(), b"-1234.05e5");
    }

    #[test]
    fn serialize_works() {
        let obj = JsonValue::Object(vec![(
            "test\"123".chars().into_iter().collect(),
            JsonValue::Null,
        )]);
        assert_eq!(
            std::str::from_utf8(&obj.format(4)[..]).unwrap(),
            r#"{
    "test\"123": null
}"#
        );

        let obj = JsonValue::Object(vec![
            (
                vec!['t', 'e', 's', 't'],
                JsonValue::Number(NumberValue {
                    integer: 123,
                    fraction: 4,
                    fraction_length: 2,
                    exponent: 0,
                    negative: false,
                }),
            ),
            (
                vec!['t', 'e', 's', 't', '2'],
                JsonValue::Array(vec![
                    JsonValue::Number(NumberValue {
                        integer: 1,
                        fraction: 0,
                        fraction_length: 0,
                        exponent: -4,
                        negative: false,
                    }),
                    JsonValue::Number(NumberValue {
                        integer: 2,
                        fraction: 41,
                        fraction_length: 3,
                        exponent: 2,
                        negative: false,
                    }),
                    JsonValue::Boolean(true),
                    JsonValue::Boolean(false),
                    JsonValue::Null,
                    JsonValue::String(vec!['\"', '1', 'n', '\"']),
                    JsonValue::Object(vec![]),
                    JsonValue::Array(vec![]),
                ]),
            ),
        ]);

        assert_eq!(
            std::str::from_utf8(&obj.format(4)[..]).unwrap(),
            r#"{
    "test": 123.04,
    "test2": [
        1e-4,
        2.041e2,
        true,
        false,
        null,
        "\"1n\"",
        {},
        []
    ]
}"#
        );

        assert_eq!(
            std::str::from_utf8(&obj.serialize()[..]).unwrap(),
            r#"{"test":123.04,"test2":[1e-4,2.041e2,true,false,null,"\"1n\"",{},[]]}"#
        );
    }

    #[test]
    fn to_f64_works() {
        use assert_float_eq::*;

        assert_f64_near!(
            NumberValue {
                integer: 1,
                fraction: 5,
                fraction_length: 1,
                exponent: 0,
                negative: true,
            }
            .to_f64(),
            -1.5
        );

        assert_f64_near!(
            NumberValue {
                integer: 0,
                fraction: 5,
                fraction_length: 1,
                exponent: 0,
                negative: true,
            }
            .to_f64(),
            -0.5
        );

        assert_f64_near!(
            NumberValue {
                integer: 0,
                fraction: 5,
                fraction_length: 1,
                exponent: 0,
                negative: false,
            }
            .to_f64(),
            0.5
        );

        assert_f64_near!(
            NumberValue {
                integer: 1,
                fraction: 15,
                fraction_length: 3,
                exponent: 1,
                negative: false,
            }
            .to_f64(),
            10.15
        );

        assert_f64_near!(
            NumberValue {
                integer: 1,
                fraction: 15,
                fraction_length: 3,
                exponent: -1,
                negative: true,
            }
            .to_f64(),
            -0.1015
        );
    }
}
