use parity_scale_codec::Compact;
use parity_scale_codec::Decode;
use scale_info::form::PortableForm;
use scale_info::interner::UntrackedSymbol;
use scale_info::prelude::any::TypeId;
use scale_info::PortableRegistry;
use scale_info::Type;
use scale_info::{TypeDef, TypeDefPrimitive};
pub trait VisitScale<'scale> {
    fn visit(&mut self, path: &Vec<&'scale str>, mut data: &'scale [u8], ty: &Type<PortableForm>) {
        println!("path {:?}, ty {:?}", path, ty);
        match ty.type_def() {
            TypeDef::Primitive(TypeDefPrimitive::Bool) => {
                let d = &mut data;
                let val = bool::decode(d).unwrap();
                println!("bool is {}", val);
            }
            TypeDef::Primitive(TypeDefPrimitive::Str) => {
                let val = std::str::from_utf8(data).unwrap();
                println!("str is {}", val);
            }
            TypeDef::Sequence(_) => {
                // u8 slice
                let val = data;
                println!("str is {:?}", val);
            }
            _ => {
                println!("ignoring a {:?}", ty.type_def());
            }
        }
    }
}

pub mod borrow_decode;

macro_rules! descale {
    (struct $n:ident <$scale:lifetime> {
        $(#[path($path:literal)] $fieldname:ident: $t:ty,)+
    }) => {
        #[derive(Default)]
        struct $n<$scale> {
            $(pub $fieldname: $t,)+
            _tag: std::marker::PhantomData<&$scale [u8]>
        }

        impl <$scale> $n<$scale> {
            fn parse(data: &'scale [u8], top_type: UntrackedSymbol<TypeId>, types: &scale_info::PortableRegistry) -> $n<$scale> {
                let mut slf = $n::<$scale>::default();
                crate::skeleton_decode(data, top_type, &mut slf, types);
                slf
            }
        }

        impl <'scale> VisitScale<'scale> for $n<$scale> {
            fn visit(&mut self, current_path: &Vec<&'scale str>, data: &'scale [u8], _ty: &scale_info::Type<scale_info::form::PortableForm>) {
                $(
                let p: Vec<_> = $path.split('.').collect();
                // println!("visited path {:?} == {:?}", path, p);
                if *current_path == p {
                    self.$fieldname = <$t as crate::borrow_decode::BorrowDecode>::borrow_decode(data);
                })+
            }
        }
    };
}

/// Walk the bytes with knowledge of the type and metadata and provide slices
/// to the visitor that it can optionally decode.
pub fn skeleton_decode<'scale>(
    data: &'scale [u8],
    ty_id: UntrackedSymbol<TypeId>,
    visitor: &mut impl VisitScale<'scale>,
    types: &PortableRegistry,
) {
    let ty = types.resolve(ty_id.id()).unwrap();
    let vec: Vec<&'scale str> = vec![];
    let cursor = &mut &*data;
    semi_decode_aux(vec, cursor, ty, visitor, types);
}

static NUMS: &[&str] = &["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
fn semi_decode_aux<'scale, V: VisitScale<'scale>>(
    mut stack: Vec<&'scale str>,
    data: &mut &'scale [u8],
    ty: &Type<PortableForm>,
    visitor: &mut V,
    types: &PortableRegistry,
) -> Vec<&'scale str> {
    // println!("decode {:?}", ty);
    match ty.type_def() {
        TypeDef::Composite(inner) => {
            for (i, field) in inner.fields().iter().enumerate() {
                let field_ty = types.resolve(field.ty().id()).unwrap();
                let s: &'scale str = NUMS[i];
                let fieldname: &'scale str = field.name().copied().unwrap_or(s);
                stack.push(fieldname);
                stack = semi_decode_aux(stack, data, field_ty, visitor, types);
                stack.pop();
            }
        }
        TypeDef::Primitive(TypeDefPrimitive::Str) => {
            let len: u32 = Compact::<u32>::decode(data).unwrap().into();
            let len = len as usize;
            visitor.visit(&stack, &data[..len], ty);
            *data = &data[len..];
        }
        TypeDef::Primitive(TypeDefPrimitive::Bool) => {
            // let size = ty..encoded_fixed_size().unwrap();
            visitor.visit(&stack, &data[..1], ty);
            *data = &data[1..];
        }
        TypeDef::Primitive(TypeDefPrimitive::U8) => {
            const LEN: usize = 1;
            visitor.visit(&stack, &data[..LEN], ty);
            *data = &data[LEN..];
        }
        TypeDef::Primitive(TypeDefPrimitive::U16) => {
            const LEN: usize = 2;
            visitor.visit(&stack, &data[..LEN], ty);
            *data = &data[LEN..];
        }
        TypeDef::Primitive(TypeDefPrimitive::U32) => {
            const LEN: usize = 4;
            visitor.visit(&stack, &data[..LEN], ty);
            *data = &data[LEN..];
        }
        TypeDef::Primitive(TypeDefPrimitive::U64) => {
            const LEN: usize = 8;
            visitor.visit(&stack, &data[..LEN], ty);
            *data = &data[LEN..];
        }
        TypeDef::Primitive(TypeDefPrimitive::U128) => {
            const LEN: usize = 16;
            visitor.visit(&stack, &data[..LEN], ty);
            *data = &data[LEN..];
        }
        TypeDef::Sequence(seq) => {
            let len: u64 = Compact::<u64>::decode(data).unwrap().into();
            let ty_id = seq.type_param();
            let ty = types.resolve(ty_id.id()).unwrap();
            if *ty.type_def() == TypeDef::Primitive(TypeDefPrimitive::U8) {
                visitor.visit(&stack, &data[..len as usize], ty);
            } else {
                println!("seq len = {}", len);
                for i in NUMS.iter().take(len as usize) {
                    // println!("i = {}", i);println!("bytes left to decode start: {:?}", &data);
                    stack.push(i);
                    stack = semi_decode_aux(stack, data, ty, visitor, types);
                    // println!("bytes left to decode end  : {:?}", &data);
                    stack.pop();
                }
            }
        }
        _ => {
            println!("don't understand a {:?}", ty.type_def());
        }
    }
    stack
}

#[cfg(test)]
mod tests {
    use crate::{skeleton_decode, VisitScale};
    use parity_scale_codec::*;
    use scale_info::interner::UntrackedSymbol;
    use scale_info::prelude::any::TypeId;
    use scale_info::PortableRegistry;
    struct S;
    impl<'scale> VisitScale<'scale> for S {}

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

    #[test]
    fn bool_test() {
        let val = false;
        let encoded = val.encode();

        let (id, types) = make_type::<bool>();

        skeleton_decode(&encoded[..], id, &mut S {}, &types)
    }

    #[test]
    fn string_test() {
        let val = "hello string";
        let encoded = val.encode();

        let (id, types) = make_type::<&str>();

        skeleton_decode(&encoded[..], id, &mut S {}, &types)
    }

    #[test]
    fn struct_test() {
        // Only try and decode the bool
        #[derive(Decode, Encode, scale_info::TypeInfo)]
        struct X {
            val: bool,
            name: String,
        }
        let val = X {
            val: false,
            name: "hi val".into(),
        };
        let encoded = val.encode();

        let (id, types) = make_type::<X>();

        skeleton_decode(&encoded[..], id, &mut S {}, &types);

        descale! {
            struct XParse<'scale> {
                #[path("val")]
                named_bool: bool,
                #[path("name")]
                named_bool2: &'scale str,
            }
        };
        let xx = XParse::parse(&encoded[..], id, &types);
        assert_eq!(xx.named_bool2, "hi val");
    }

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

        skeleton_decode(&encoded[..], id, &mut S {}, &types);

        descale! {
            struct XParse<'scale> {
                #[path("more_scale")]
                uncopied_bytes: &'scale [u8],
            }
        };
        let xx = XParse::parse(&encoded[..], id, &types);
        assert_eq!(xx.uncopied_bytes, vec![1, 2, 3, 4].as_slice());
    }

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

        skeleton_decode(&encoded[..], id, &mut S {}, &types);

        descale! {
            struct XParse<'scale> {
                #[path("a")]
                a: u8,
                #[path("b")]
                b: u16,
                #[path("c")]
                c: u32,
                #[path("d")]
                d: u64,
                #[path("e")]
                e: u128,
            }
        };
        let xx = XParse::parse(&encoded[..], id, &types);
        assert_eq!(xx.a, 1);
        assert_eq!(xx.b, 2);
        assert_eq!(xx.c, 3);
        assert_eq!(xx.d, 4);
        assert_eq!(xx.e, 5);
    }

    #[test]
    fn array_test() {
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
        println!("bytes {:?}", encoded);

        let (id, types) = make_type::<Y>();

        skeleton_decode(&encoded[..], id, &mut S {}, &types);

        descale! {
            struct XParse<'scale> {
                #[path("outer.0.val")]
                named_bool: bool,
                #[path("outer.1.name")]
                named_bool2: &'scale str,
            }
        };
        let xx = XParse::parse(&encoded[..], id, &types);
        assert_eq!(xx.named_bool, true);
        assert_eq!(xx.named_bool2, "skip meh");
    }
}
