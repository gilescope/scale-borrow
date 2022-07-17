use scale_info::TypeDef;
use scale_info::TypeDefPrimitive;

/// The underlying shape of a given value.
#[derive(Clone, Debug, PartialEq)]
pub enum Value<'scale> {
    /// A named or unnamed struct-like, array-like or tuple-like set of values.
    Object(Box<Vec<(&'scale str, Value<'scale>)>>), // Could this be an array rather than a vec?
    // // UnamedComposite(&'scale Vec<Value<T>>)
    // /// An enum variant.
    // Variant(&'scale (&'scale str, &'scale Value<'scale>)),
    // Truth
    Bool(bool),
    Char(char),
    Str(&'scale str),
    Scale(&'scale [u8]),
    // Array(Box<Vec<Value<'scale>>>),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(Box<u128>),
    I128(Box<i128>),
    // // / An unsigned 256 bit number (internally represented as a 32 byte array).
    U256(&'scale [u8; 32]),
    // // / A signed 256 bit number (internally represented as a 32 byte array).
    I256(&'scale [u8; 32]),

    #[cfg(feature = "bitvec")]
    Bits(Box<bitvec::prelude::BitVec<u8, bitvec::prelude::Lsb0>>),
}

#[derive(Default)]
pub struct ValueBuilder<'scale> {
    root: Option<Value<'scale>>,
}

impl<'scale> ValueBuilder<'scale> {
    pub fn parse(
        data: &'scale [u8],
        top_type_id: u32,
        types: &scale_info::PortableRegistry,
    ) -> Value<'scale> {
        let mut slf = ValueBuilder::<'scale>::default();
        crate::skeleton_decode(data, top_type_id, &mut slf, types);
        slf.root.take().unwrap()
    }

    fn append(
        path: &[(&'scale str, u32)],
        current: &mut Value<'scale>,
        new_field: &'scale str,
        new_val: Value<'scale>,
    ) {
        if let Value::<'scale>::Object(fields) = current {
            if path.is_empty() {
                // println!("appending path {:?} fin {:?}  / {:?} to {:?}",path, new_field, new_val, &fields);
                fields.push((new_field, new_val));
                return;
            }

            let ((head, head_ty), tail) = path.split_first().unwrap();
            for (field, child) in fields.iter_mut() {
                if field == head {
                    // println!("appending deeper new path {:?} | {:?}  / {:?} ", &tail, new_field, new_val);
                    ValueBuilder::append(tail, child, new_field, new_val);
                    return;
                }
            }
            // println!("appending path {:?} notfound {:?} adding {:?} | {:?}  / {:?} ", &tail, head, fields, new_field, new_val);

            fields.push((
                head,
                Value::Object(Box::new(vec![("_ty", Value::U32(*head_ty))])),
            ));
            let (_, new_current) = fields.last_mut().unwrap();
            ValueBuilder::append(tail, new_current, new_field, new_val);
        } else {
            panic!()
        }
    }

    #[cfg(not(feature = "bitvec"))]
    #[inline]
    fn parse_bitvec(data: &'scale [u8]) -> Option<Value> {
        Some(Value::Scale(data))
    }

    #[cfg(feature = "bitvec")]
    #[inline]
    fn parse_bitvec(mut data: &'scale [u8]) -> Option<Value> {
        assert_eq!(data.len(), 1, "bitvec size not suppored - please send pr.");
        Some(
             Value::Bits(Box::new(
                <bitvec::prelude::BitVec<u8, bitvec::prelude::Lsb0>
                as
                parity_scale_codec::Decode>::decode(&mut data).unwrap())))
    }
}

impl<'scale> super::VisitScale<'scale> for ValueBuilder<'scale> {
    fn visit(
        &mut self,
        current_path: &[(&'scale str, u32)],
        data: &'scale [u8],
        ty: &scale_info::Type<scale_info::form::PortableForm>,
    ) {
        let new_val = match ty.type_def() {
            scale_info::TypeDef::Primitive(TypeDefPrimitive::Str) => Some(Value::Str(
                <&'scale str as crate::borrow_decode::BorrowDecode>::borrow_decode(data),
            )),
            scale_info::TypeDef::Primitive(TypeDefPrimitive::Bool) => Some(Value::Bool(
                <bool as crate::borrow_decode::BorrowDecode>::borrow_decode(data),
            )),
            scale_info::TypeDef::Primitive(TypeDefPrimitive::U8) => Some(Value::U8(
                <u8 as crate::borrow_decode::BorrowDecode>::borrow_decode(data),
            )),
            scale_info::TypeDef::Primitive(TypeDefPrimitive::U16) => Some(Value::U16(
                <u16 as crate::borrow_decode::BorrowDecode>::borrow_decode(data),
            )),
            scale_info::TypeDef::Primitive(TypeDefPrimitive::U32) => Some(Value::U32(
                <u32 as crate::borrow_decode::BorrowDecode>::borrow_decode(data),
            )),
            scale_info::TypeDef::Primitive(TypeDefPrimitive::U64) => Some(Value::U64(
                <u64 as crate::borrow_decode::BorrowDecode>::borrow_decode(data),
            )),
            scale_info::TypeDef::Primitive(TypeDefPrimitive::U128) => Some(Value::U128(Box::new(
                <u128 as crate::borrow_decode::BorrowDecode>::borrow_decode(data),
            ))),

            TypeDef::Sequence(_seq) => {
                //TODO: assumed u8
                Some(Value::Scale(data))
            }
            TypeDef::BitSequence(_seq) => ValueBuilder::parse_bitvec(data),
            TypeDef::Compact(inner) => {
                let inner = types.resolve(inner.type_param().id()).unwrap();
                match inner.type_def() {
                    TypeDef::Primitive(TypeDefPrimitive::U32) => {
                        Some(Value::U32(<Compact<u32> as crate::borrow_decode::BorrowDecode>::borrow_decode(data).into()))
                    },
                    _ => panic!("unsupported {:?}", inner)
                }
            }
            _ => {
                panic!("skipping {:?}", ty);
            }
        };

        // place val in right location.
        let last = if self.root.is_none() {
            if current_path.is_empty() {
                self.root = new_val;
                return;
            }
            let (last, last_ty) = current_path.last().unwrap();
            self.root = Some(Value::Object(Box::new(vec![("_ty", Value::U32(*last_ty))])));
            last
        } else {
            let (last, _) = current_path.last().unwrap();
            last
        };

        // println!("appending {:?}  / {:?}", current_path, new_val);

        ValueBuilder::append(
            &current_path[..current_path.len() - 1],
            self.root.as_mut().unwrap(),
            last,
            new_val.unwrap(),
        );
    }
}
