#[cfg(feature = "display")]
use core::fmt::{Display, Formatter};

/// The underlying shape of a given value.
///
/// 'scale is the lifetime of the binary blob.
/// 'info is the lifetime of the metadata.
#[derive(Clone, Debug)]
pub enum Value<'scale, 'info> {
    /// A named or unnamed struct-like, array-like or tuple-like set of values.
    Object(Box<Vec<(&'info str, Value<'scale, 'info>)>>),

    Bool(bool),
    Char(char),
    Str(&'scale str),
    Scale(&'scale [u8]),
    // Escape hatch for when you can't borrow.
    ScaleOwned(Box<Vec<u8>>),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(Box<u128>),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(Box<i128>),
    /// An unsigned 256 bit number (internally represented as a 32 byte array).
    U256(&'scale [u8; 32]),
    /// A signed 256 bit number (internally represented as a 32 byte array).
    I256(&'scale [u8; 32]),
    Bits(Box<scale_bits::Bits>),
}

impl<'scale, 'info> PartialEq for Value<'scale, 'info> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Object(val1), Value::Object(val2)) => val1 == val2,
            (Value::Bool(val1), Value::Bool(val2)) => val1 == val2,
            (Value::Char(val1), Value::Char(val2)) => val1 == val2,
            (Value::Str(val1), Value::Str(val2)) => val1 == val2,
            (Value::Scale(val1), Value::Scale(val2)) => val1 == val2,
            (Value::ScaleOwned(val1), Value::ScaleOwned(val2)) => val1 == val2,
            (Value::U8(val1), Value::U8(val2)) => val1 == val2,
            (Value::U16(val1), Value::U16(val2)) => val1 == val2,
            (Value::U32(val1), Value::U32(val2)) => val1 == val2,
            (Value::U64(val1), Value::U64(val2)) => val1 == val2,
            (Value::U128(val1), Value::U128(val2)) => val1 == val2,
            (Value::U256(val1), Value::U256(val2)) => val1 == val2,
            (Value::I8(val1), Value::I8(val2)) => val1 == val2,
            (Value::I16(val1), Value::I16(val2)) => val1 == val2,
            (Value::I32(val1), Value::I32(val2)) => val1 == val2,
            (Value::I64(val1), Value::I64(val2)) => val1 == val2,
            (Value::I128(val1), Value::I128(val2)) => val1 == val2,
            (Value::I256(val1), Value::I256(val2)) => val1 == val2,
            (Value::Bits(_val1), Value::Bits(_val2)) => true, //TODO: can we do better than this?
            _ => false,
        }
    }
}

#[cfg(feature = "display")]
impl<'scale, 'info> Display for Value<'scale, 'info> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        const TRUNC_LEN: usize = 100;
        match self {
            Self::Object(contents) => {
                write!(f, "{{").unwrap();
                let mut first = true;
                for (k, v) in contents.iter() {
                    if !first {
                        write!(f, ", ").unwrap();
                    }
                    k.fmt(f).unwrap();
                    write!(f, ": ").unwrap();
                    v.fmt(f).unwrap();
                    first = false;
                }
                write!(f, "}}").unwrap();
            }
            Self::Scale(slice) => {
                if slice.len() <= TRUNC_LEN {
                    write!(f, "Scale(0x{})", hex::encode(slice)).unwrap();
                } else {
                    write!(f, "Scale(0x{}...)", hex::encode(&slice[..TRUNC_LEN - 3])).unwrap();
                }
            }
            Self::ScaleOwned(v) => {
                if v.len() <= TRUNC_LEN {
                    write!(f, "ScaleOwned(0x{})", hex::encode(v.as_slice())).unwrap();
                } else {
                    write!(
                        f,
                        "ScaleOwned(0x{}...)",
                        hex::encode(&v.as_slice()[..TRUNC_LEN - 3])
                    )
                    .unwrap();
                }
            }
            _ => <Self as core::fmt::Debug>::fmt(self, f).unwrap(),
        };
        Ok(())
    }
}

impl<'a, 'scale, 'info> IntoIterator for &'a Value<'scale, 'info>
where
    'info: 'scale,
{
    type Item = &'a (&'scale str, Value<'scale, 'info>);
    type IntoIter = core::slice::Iter<'a, (&'scale str, Value<'scale, 'info>)>;

    fn into_iter(self) -> Self::IntoIter {
        if let Value::Object(ref vals) = *self {
            vals.iter()
        } else {
            debug_assert!(false); // This is not a good sign.
            todo!();
            // vec![].iter()
        }
    }
}

impl<'scale, 'info> Value<'scale, 'info> {
    pub fn get(&self, path: &str) -> Option<&Value> {
        let p: Vec<_> = path.split('.').collect();
        let mut cur = self;

        for pa in p {
            if let Value::Object(fields) = cur {
                if let Some((_, sub_val)) = fields.iter().find(|(name, _)| *name == pa) {
                    cur = sub_val;
                } else {
                    return None;
                }
            }
        }

        Some(cur)
    }

    // Assume that this is an object with just one field. TODO! rename only()
    pub fn only(&'scale self) -> Option<(&'scale str, &'scale Self)> {
        if let Self::Object(fields) = self {
            if fields.len() == 1 {
                let (name, val) = &fields[0];
                Some((name, val))
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn only2(&'scale self) -> Option<(&'scale str, &'scale str, &'scale Self)> {
        self.only()
            .and_then(|(head, tail)| tail.only().map(|(second, tail)| (head, second, tail)))
    }

    pub fn only3(&'scale self) -> Option<(&'scale str, &'scale str, &'scale str, &'scale Self)> {
        self.only2().and_then(|(first, second, tail)| {
            tail.only()
                .map(|(third, tail)| (first, second, third, tail))
        })
    }

    pub fn expect(&'scale self, expect1: &str) -> Option<&'scale Self> {
        self.only().and_then(|(head, tail)| {
            if head != expect1 {
                return None;
            }
            Some(tail)
        })
    }

    pub fn expect2(&'scale self, expect1: &str, expect2: &str) -> Option<&'scale Self> {
        self.expect(expect1).and_then(|tail| tail.expect(expect2))
    }

    pub fn expect3(
        &'scale self,
        expect1: &str,
        expect2: &str,
        expect3: &str,
    ) -> Option<&'scale Self> {
        self.expect2(expect1, expect2)
            .and_then(|tail| tail.expect(expect3))
    }

    pub fn expect4(
        &'scale self,
        expect1: &str,
        expect2: &str,
        expect3: &str,
        expect4: &str,
    ) -> Option<&'scale Self> {
        self.expect3(expect1, expect2, expect3)
            .and_then(|tail| tail.expect(expect4))
    }

    pub fn find(&'scale self, find1: &str) -> Option<&'scale Self> {
        if let Self::Object(fields) = self {
            for (field, val) in fields.iter() {
                if *field == find1 {
                    return Some(val);
                }
            }
        }
        None
    }

    pub fn find2(&'scale self, find1: &str, find2: &str) -> Option<&'scale Self> {
        self.find(find1).and_then(|val| {
            if let Self::Object(fields) = val {
                for (field, val) in fields.iter() {
                    if *field == find2 {
                        return Some(val);
                    }
                }
            }
            None
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::Value;

    #[test]
    fn test_iter() {
        let val = Value::Object(Box::new(vec![("0", Value::U32(0)), ("1", Value::U32(1))]));

        let it = val.into_iter();
        for i in it {
            println!("{:?}", i);
        }
    }

    #[test]
    #[cfg(feature = "display")]
    fn test_display() {
        let data = &[1, 2, 3, 4, 17, 18, 19, 20];
        let val = Value::Object(Box::new(vec![
            ("0", Value::U32(0)),
            ("1", Value::Scale(data)),
        ]));

        assert_eq!(
            r#"{0: U32(0), 1: Scale(0x0102030411121314)}"#,
            val.to_string()
        );

        let data = &[7; 200];
        let val = Value::Object(Box::new(vec![
            ("0", Value::U32(0)),
            ("1", Value::Scale(data)),
        ]));

        assert_eq!(
            r#"{0: U32(0), 1: Scale(0x07070707070707070707070707070707070707070707070707070707070707070707070707070707070707070707070707070707070707070707070707070707070707070707070707070707070707070707070707070707070707070707070707...)}"#,
            val.to_string()
        );
    }
}
