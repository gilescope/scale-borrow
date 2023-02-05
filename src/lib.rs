// use parity_scale_codec::Compact;
// use parity_scale_codec::Decode;
// use scale_info::form::PortableForm;
// use scale_info::PortableRegistry;
// use scale_info::Type;
// use scale_info::{TypeDef, TypeDefPrimitive};
// pub trait VisitScale<'scale> {
//     // Visit value on current object
//     fn visit(
//         &mut self,
//         path: &[(&'scale str, u32)],
//         data: &'scale [u8],
//         ty: &'scale Type<PortableForm>,
//         types: &'scale PortableRegistry,
//     );
// }
// pub mod borrow_decode;
pub mod value;

use scale_decode::visitor::types::{Array, BitSequence, Composite, Sequence, Str, Tuple, Variant};
use scale_decode::visitor::TypeId;
pub use value::Value;
// use scale_decode::visitor::decode_with_visitor;

// #[macro_export]
// macro_rules! descale {
//     (struct $n:ident <$scale:lifetime> {
//         $(#[path($path:literal)] $fieldname:ident: $t:ty,)+
//     }) => {
//         #[derive(Default)]
//         struct $n<$scale> {
//             $(pub $fieldname: $t,)+
//             _tag: std::marker::PhantomData<&$scale [u8]>
//         }

//         impl <$scale> $n<$scale> {
//             fn parse(data: &'scale [u8], top_type: UntrackedSymbol<TypeId>, types: &'scale scale_info::PortableRegistry) -> $n<$scale> {
//                 let mut slf = $n::<$scale>::default();
//                 crate::skeleton_decode(data, top_type.id(), &mut slf, types);
//                 slf
//             }
//         }

//         impl <'scale> VisitScale<'scale> for $n<$scale> {
//             fn visit(&mut self, current_path: &[(&'scale str,u32)], data: &'scale [u8], _ty: &'scale scale_info::Type<scale_info::form::PortableForm>, _types: &'scale PortableRegistry) {
//                 $(
//                     let p: Vec<_> = $path.split('.').collect();//TODO: do earlier.
//                     // println!("visited path {:?} == {:?}", current_path, p);
//                     if current_path.len() == p.len() {
//                         let same = current_path.iter().zip(p).all(|((seg,_), p_seg)| *seg == p_seg);
//                         if same {
//                             // println!("visited path found");
//                             self.$fieldname = <$t as crate::borrow_decode::BorrowDecode>::borrow_decode(data);
//                         }
//                     }
//                 )+
//             }
//         }
//     };
// }

// /// Walk the bytes with knowledge of the type and metadata and provide slices
// /// to the visitor that it can optionally decode.
// pub fn skeleton_decode<'scale>(
//     data: &'scale [u8],
//     ty_id: u32,
//     visitor: &mut impl VisitScale<'scale>,
//     types: &'scale PortableRegistry,
// ) {
//     let id = ty_id;
//     let ty = types.resolve(id).unwrap();
//     let vec: Vec<(&'scale str, u32)> = vec![];
//     let cursor = &mut &*data;
//     semi_decode_aux(vec, cursor, ty, id, visitor, types);
// }

struct BorrowVisitor {}
impl scale_decode::visitor::Visitor for BorrowVisitor {
    type Value<'s, 'i> = crate::Value<'s, 'i>;
    type Error = scale_decode::visitor::DecodeError;

    fn visit_bool<'s, 'm>(
        self,
        value: bool,
        _type_id: TypeId,
    ) -> Result<Self::Value<'s, 'm>, Self::Error> {
        Ok(Value::Bool(value))
    }
    fn visit_char<'s, 'm>(
        self,
        value: char,
        _type_id: TypeId,
    ) -> Result<Self::Value<'s, 'm>, Self::Error> {
        Ok(Value::Char(value))
    }
    fn visit_u8<'s, 'm>(
        self,
        value: u8,
        _type_id: TypeId,
    ) -> Result<Self::Value<'s, 'm>, Self::Error> {
        Ok(Value::U8(value))
    }
    fn visit_u16<'s, 'm>(
        self,
        value: u16,
        _type_id: TypeId,
    ) -> Result<Self::Value<'s, 'm>, Self::Error> {
        Ok(Value::U16(value))
    }
    fn visit_u32<'s, 'm>(
        self,
        value: u32,
        _type_id: TypeId,
    ) -> Result<Self::Value<'s, 'm>, Self::Error> {
        Ok(Value::U32(value))
    }
    fn visit_u64<'s, 'm>(
        self,
        value: u64,
        _type_id: TypeId,
    ) -> Result<Self::Value<'s, 'm>, Self::Error> {
        Ok(Value::U64(value))
    }
    fn visit_u128<'s, 'm>(
        self,
        value: u128,
        _type_id: TypeId,
    ) -> Result<Self::Value<'s, 'm>, Self::Error> {
        Ok(Value::U128(Box::new(value)))
    }
    fn visit_u256<'info>(
        self,
        value: &'_ [u8; 32],
        _type_id: TypeId,
    ) -> Result<Self::Value<'_, 'info>, Self::Error> {
        Ok(Value::U256(value))
    }
    fn visit_i8<'s, 'm>(
        self,
        value: i8,
        _type_id: TypeId,
    ) -> Result<Self::Value<'s, 'm>, Self::Error> {
        Ok(Value::I8(value))
    }
    fn visit_i16<'s, 'm>(
        self,
        value: i16,
        _type_id: TypeId,
    ) -> Result<Self::Value<'s, 'm>, Self::Error> {
        Ok(Value::I16(value))
    }
    fn visit_i32<'s, 'm>(
        self,
        value: i32,
        _type_id: TypeId,
    ) -> Result<Self::Value<'s, 'm>, Self::Error> {
        Ok(Value::I32(value))
    }
    fn visit_i64<'s, 'm>(
        self,
        value: i64,
        _type_id: TypeId,
    ) -> Result<Self::Value<'s, 'm>, Self::Error> {
        Ok(Value::I64(value))
    }
    fn visit_i128<'s, 'm>(
        self,
        value: i128,
        _type_id: TypeId,
    ) -> Result<Self::Value<'s, 'm>, Self::Error> {
        Ok(Value::I128(Box::new(value)))
    }
    fn visit_i256<'info>(
        self,
        value: &[u8; 32],
        _type_id: TypeId,
    ) -> Result<Self::Value<'_, 'info>, Self::Error> {
        Ok(Value::I256(value))
    }

    /// Called when a sequence of values is seen in the input bytes.
    fn visit_sequence<'scale, 'info>(
        self,
        value: &mut Sequence<'scale, 'info>,
        _type_id: TypeId,
    ) -> Result<Self::Value<'scale, 'info>, Self::Error> {
        let mut res = vec![];

        //If it's a sequence of U8 then special case it:
        let first = value.decode_item(BorrowVisitor {});
        match first {
            Some(Ok(Value::U8(_))) => {
                return Ok(Value::Scale(&value.bytes()[1..]));
            }
            Some(Ok(not_u8)) => res.push(("0", not_u8)),
            _ => {}
        }

        let mut i = 1;
        while let Some(Ok(item)) = value.decode_item(BorrowVisitor {}) {
            res.push((NUMS[i], item));
            i += 1;
        }
        Ok(Value::Object(Box::new(res)))
    }

    /// Called when a composite value is seen in the input bytes.
    fn visit_composite<'scale, 'info>(
        self,
        value: &mut Composite<'scale, 'info>,
        _type_id: TypeId,
    ) -> Result<Self::Value<'scale, 'info>, Self::Error> {
        let mut i = 0;
        let mut res = vec![];
        let mut name = value.peek_name().unwrap_or("0");
        while let Some(Ok(item)) = value.decode_item(BorrowVisitor {}) {
            res.push((name, item));
            name = value.peek_name().unwrap_or(NUMS[i]);
            i += 1;
        }
        Ok(Value::Object(Box::new(res)))
    }

    /// Called when a tuple of values is seen in the input bytes.
    fn visit_tuple<'scale, 'info>(
        self,
        value: &mut Tuple<'scale, 'info>,
        _type_id: TypeId,
    ) -> Result<Self::Value<'scale, 'info>, Self::Error> {
        let mut i = 0;
        let mut res = vec![];
        while let Some(Ok(item)) = value.decode_item(BorrowVisitor {}) {
            res.push((NUMS[i], item));
            i += 1;
        }
        Ok(Value::Object(Box::new(res)))
    }

    fn visit_str<'s, 'm>(
        self,
        value: &mut Str<'s>,
        _type_id: TypeId,
    ) -> Result<Self::Value<'s, 'm>, Self::Error> {
        Ok(Value::Str(value.as_str().unwrap()))
    }

    /// Called when an array is seen in the input bytes.
    fn visit_array<'scale, 'info>(
        self,
        value: &mut Array<'scale, 'info>,
        _type_id: TypeId,
    ) -> Result<Self::Value<'scale, 'info>, Self::Error> {
        let mut i = 0;
        let mut res = vec![];
        while let Some(Ok(item)) = value.decode_item(BorrowVisitor {}) {
            res.push((NUMS[i], item));
            i += 1;
        }
        Ok(Value::Object(Box::new(res)))
    }

    /// Called when a variant is seen in the input bytes.
    fn visit_variant<'scale, 'info>(
        self,
        value: &mut Variant<'scale, 'info>,
        _type_id: TypeId,
    ) -> Result<Self::Value<'scale, 'info>, Self::Error> {
        let mut res = vec![];
        let inner_value = value.fields();
        let mut i = 0;
        let mut res_inner = vec![];
        let mut name = inner_value.peek_name().unwrap_or("0");
        while let Some(Ok(item)) = inner_value.decode_item(BorrowVisitor {}) {
            res_inner.push((name, item));
            i += 1;
            name = inner_value.peek_name().unwrap_or(NUMS[i]);
        }
        res.push((value.name(), Value::Object(Box::new(res_inner))));

        Ok(Value::Object(Box::new(res)))
    }

    /// Called when a bit sequence is seen in the input bytes.
    fn visit_bitsequence<'scale, 'm>(
        self,
        value: &mut BitSequence<'scale>,
        _type_id: TypeId,
    ) -> Result<Self::Value<'scale, 'm>, Self::Error> {
        Ok(Value::Bits(Box::new(value.decode().unwrap())))
    }
}

static NUMS: &[&str] = &[
    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14",
];

#[cfg(test)]
mod tests {
    use parity_scale_codec::*;
    use scale_decode::visitor::decode_with_visitor;
    use scale_info::interner::UntrackedSymbol;
    use scale_info::prelude::any::TypeId;
    use scale_info::PortableRegistry;
    use wasm_bindgen_test::*;

    use super::value::Value;
    use crate::BorrowVisitor;

    /// Given a type definition, return the PortableType and PortableRegistry
    /// that our decode functions expect.
    fn make_type<T: scale_info::TypeInfo + 'static>() -> (UntrackedSymbol<TypeId>, PortableRegistry)
    {
        let m = scale_info::MetaType::new::<T>();
        let mut types = scale_info::Registry::new();
        let id = types.register_type(&m);
        let portable_registry: PortableRegistry = types.into();

        (id, portable_registry)
    }

    #[wasm_bindgen_test]
    #[test]
    fn bool_test() {
        let val = false;
        let encoded = val.encode();

        let (id, types) = make_type::<bool>();

        let val = decode_with_visitor(&mut encoded.as_slice(), id.id(), &types, BorrowVisitor {})
            .unwrap();
        assert_eq!(val, Value::Bool(false));
    }

    // #[wasm_bindgen_test]
    // #[test]
    // #[cfg(feature = "bitvec")]
    // fn bitvec_test() {
    //     use bitvec::prelude::*;
    //     let val = bitvec![u8, Msb0;];
    //     let encoded = val.encode();

    //     let (id, types) = make_type::<BitVec<u8, bitvec::order::Lsb0>>();

    //     let val = ValueBuilder::parse(&encoded, id, &types);
    //     assert_eq!(val, Value::Bits(Box::new(bitvec![u8, Lsb0;])));
    // }

    // #[wasm_bindgen_test]
    // #[test]
    // #[cfg(not(feature = "bitvec"))]
    // fn bitvec_test2() {
    //     // use bitvec::prelude::*;
    //     let val = bitvec![u8, Msb0;];
    //     let encoded = val.encode();

    //     let (id, types) = make_type::<BitVec<u8, bitvec::order::Lsb0>>();

    //     let val = ValueBuilder::parse(&encoded, id.id(), &types);
    //     assert_eq!(val, Value::Scale(&[0]));
    // }

    #[wasm_bindgen_test]
    #[test]
    fn string_test() {
        let val = "hello string";
        let encoded = val.encode();

        let (id, types) = make_type::<&str>();
        let value = decode_with_visitor(&mut encoded.as_slice(), id.id(), &types, BorrowVisitor {})
            .unwrap();

        if let Value::Str(inner) = value {
            assert_eq!(val, inner);
        } else {
            panic!()
        }
    }

    #[wasm_bindgen_test]
    #[test]
    fn struct_test() {
        // Only try and decode the bool
        #[derive(Decode, Encode, scale_info::TypeInfo)]
        struct X {
            val: bool,
            name: String,
        }
        let val = X {
            val: true,
            name: "hi val".into(),
        };
        let encoded = val.encode();

        let (id, types) = make_type::<X>();
        let val = decode_with_visitor(&mut encoded.as_slice(), id.id(), &types, BorrowVisitor {})
            .unwrap();
        assert_eq!(
            val,
            Value::Object(Box::new(vec![
                ("val", Value::Bool(true)),
                ("name", Value::Str("hi val"))
            ]))
        );
    }

    #[wasm_bindgen_test]
    #[test]
    fn enum_test() {
        // Only try and decode the bool
        #[derive(Decode, Encode, scale_info::TypeInfo)]
        enum X {
            A,
            B(u32, u64),
            C { val: bool },
        }
        let val = X::C { val: true };
        let encoded = val.encode();

        let (id, types) = make_type::<X>();

        let val = decode_with_visitor(&mut encoded.as_slice(), id.id(), &types, BorrowVisitor {})
            .unwrap();

        assert_eq!(
            val,
            Value::Object(Box::new(vec![(
                "C",
                Value::Object(Box::new(vec![("val", Value::Bool(true))]),)
            )]))
        );
    }

    #[wasm_bindgen_test]
    #[test]
    fn tuple_test() {
        // Only try and decode the bool
        #[derive(Decode, Encode, scale_info::TypeInfo)]
        enum X {
            A,
            B(u32, (u64, u64)),
            C { val: bool },
        }
        let val = X::B(10, (20, 21));
        let encoded = val.encode();

        let (id, types) = make_type::<X>();

        let val = decode_with_visitor(&mut encoded.as_slice(), id.id(), &types, BorrowVisitor {})
            .unwrap();

        assert_eq!(
            val,
            Value::Object(Box::new(vec![(
                "B",
                Value::Object(Box::new(vec![
                    ("0", Value::U32(10)),
                    (
                        "1",
                        Value::Object(Box::new(vec![("0", Value::U64(20)), ("1", Value::U64(21))]),)
                    )
                ]),)
            )]))
        );
    }

    #[wasm_bindgen_test]
    #[test]
    fn slice_u8_test() {
        // Only try and decode the bool
        #[derive(Decode, Encode, scale_info::TypeInfo)]
        struct X {
            more_scale: Vec<u8>,
        }
        let val = X {
            more_scale: vec![1, 2, 3, 4],
        };
        let encoded = val.encode();

        let (id, types) = make_type::<X>();

        let val = decode_with_visitor(&mut encoded.as_slice(), id.id(), &types, BorrowVisitor {})
            .unwrap();

        assert_eq!(
            val,
            Value::Object(Box::new(vec![("more_scale", Value::Scale(&[1, 2, 3, 4])),]))
        );
    }

    #[wasm_bindgen_test]
    #[test]
    fn num_tests() {
        // Only try and decode the bool
        #[derive(Decode, Encode, scale_info::TypeInfo)]
        struct X {
            a: u8,
            b: u16,
            c: u32,
            d: u64,
            e: u128,
        }
        let val = X {
            a: 1,
            b: 2,
            c: 3,
            d: 4,
            e: 5,
        };
        let encoded = val.encode();

        let (id, types) = make_type::<X>();

        let val = decode_with_visitor(&mut encoded.as_slice(), id.id(), &types, BorrowVisitor {})
            .unwrap();

        assert_eq!(
            val,
            Value::Object(Box::new(vec![
                ("a", Value::U8(1)),
                ("b", Value::U16(2)),
                ("c", Value::U32(3)),
                ("d", Value::U64(4)),
                ("e", Value::U128(Box::new(5)))
            ]))
        );
    }

    #[wasm_bindgen_test]
    #[test]
    fn array_test() {
        #[derive(Decode, Encode, scale_info::TypeInfo)]
        struct Y {
            outer: [X; 2], // 8 len
        }

        // Only try and decode the bool
        #[derive(Decode, Encode, scale_info::TypeInfo)]
        struct X {
            val: bool,
            name: String,
        }
        let val = X {
            val: true,              // 1
            name: "skip me".into(), // 28 len then 115, 107, 105, 112, 32, 109, 101
        };
        let val2 = X {
            val: false,              // 0
            name: "skip meh".into(), // 28 len then 115, 107, 105, 112, 32, 109, 101, h
        };
        let y = Y { outer: [val, val2] };
        let encoded = y.encode();
        // println!("bytes {:?}", encoded);

        let (id, types) = make_type::<Y>();

        let val = decode_with_visitor(&mut encoded.as_slice(), id.id(), &types, BorrowVisitor {})
            .unwrap();

        assert_eq!(
            val,
            Value::Object(Box::new(vec![(
                "outer",
                Value::Object(Box::new(vec![
                    (
                        "0",
                        Value::Object(Box::new(vec![
                            ("val", Value::Bool(true)),
                            ("name", Value::Str("skip me"))
                        ]))
                    ),
                    (
                        "1",
                        Value::Object(Box::new(vec![
                            ("val", Value::Bool(false)),
                            ("name", Value::Str("skip meh"))
                        ]))
                    ),
                ]))
            )]))
        );
    }

    #[wasm_bindgen_test]
    #[test]
    fn vec_test() {
        #[derive(Decode, Encode, scale_info::TypeInfo)]
        struct Y {
            outer: Vec<X>, // 8 len
        }

        // Only try and decode the bool
        #[derive(Decode, Encode, scale_info::TypeInfo)]
        struct X {
            val: bool,
            name: String,
        }
        let val = X {
            val: true,              // 1
            name: "skip me".into(), // 28 len then 115, 107, 105, 112, 32, 109, 101
        };
        let val2 = X {
            val: false,              // 0
            name: "skip meh".into(), // 28 len then 115, 107, 105, 112, 32, 109, 101, h
        };
        let y = Y {
            outer: vec![val, val2],
        };
        let encoded = y.encode();

        let (id, types) = make_type::<Y>();

        let val = decode_with_visitor(&mut encoded.as_slice(), id.id(), &types, BorrowVisitor {})
            .unwrap();

        assert_eq!(
            val,
            Value::Object(Box::new(vec![(
                "outer",
                Value::Object(Box::new(vec![
                    (
                        "0",
                        Value::Object(Box::new(vec![
                            ("val", Value::Bool(true)),
                            ("name", Value::Str("skip me"))
                        ]))
                    ),
                    (
                        "1",
                        Value::Object(Box::new(vec![
                            ("val", Value::Bool(false)),
                            ("name", Value::Str("skip meh"))
                        ]))
                    ),
                ]))
            )]))
        );
    }

    #[test]
    fn test_value() {
        assert_eq!(std::mem::size_of::<super::value::Value>(), 24); // 16 in wasm32
        assert_eq!(std::mem::size_of::<u128>(), 16);
    }
}
