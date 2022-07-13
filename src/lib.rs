use parity_scale_codec::Compact;
use parity_scale_codec::Decode;
use scale_info::form::PortableForm;
use scale_info::interner::UntrackedSymbol;
use scale_info::prelude::any::TypeId;
use scale_info::PortableRegistry;
use scale_info::Type;
use scale_info::{TypeDef, TypeDefPrimitive};
pub trait VisitScale<'scale> {
    fn visit(&self, path: &Vec<&'scale str>, mut data: &'scale [u8], ty: &Type<PortableForm>) {
        println!("path {:?}, ty {:?}", path, ty);
        match ty.type_def() {
            TypeDef::Primitive(TypeDefPrimitive::Bool) => {
                let d = &mut data;
                let val = bool::decode(d).unwrap();
                println!("bool is {}", val);
            }
            TypeDef::Primitive(TypeDefPrimitive::Str) => {
                let val = std::str::from_utf8(&data).unwrap();
                println!("str is {}", val);
            }
            _ => {
                println!("ignoring a {:?}", ty.type_def());
            }
        }
    }
}

pub fn skeleton_decode<'scale>(
    //stack: Vec<&'scale str>,
    data: &'scale [u8],
    ty_id: UntrackedSymbol<TypeId>,
    visitor: &impl VisitScale<'scale>,
    types: &PortableRegistry,
) {
    let ty = types.resolve(ty_id.id()).unwrap();
    let vec: Vec<&'scale str> = vec![];
    let cursor = &mut &*data;
    semi_decode_aux(vec, cursor, ty, visitor, types);
}

static NUMS: &[&'static str] = &["0", "1", "2", "3"];
fn semi_decode_aux<'scale, V: VisitScale<'scale>>(
    mut stack: Vec<&'scale str>,
    data: &mut &'scale [u8],
    ty: &Type<PortableForm>,
    visitor: &V,
    types: &PortableRegistry,
) -> Vec<&'scale str> {
    // println!("decode {:?}", ty);
    match ty.type_def() {
        TypeDef::Composite(inner) => {
            for (i, field) in inner.fields().iter().enumerate() {
                let field_ty = types.resolve(field.ty().id()).unwrap();
                let s: &'scale str = NUMS[i];
                let fieldname: &'scale str = &*field.name().map(|f| *f).unwrap_or(s);
                stack.push(fieldname);
                // let condition = visitor.visit(&stack);

                // if condition {
                stack = semi_decode_aux(stack, data, field_ty, visitor, types);
                // } else {
                //     //TODO: skip
                //     stack = semi_decode_aux(stack, data, field_ty, visitor, types);
                // }
                stack.pop();
            }
        }
        TypeDef::Primitive(TypeDefPrimitive::Str) => {
            let len: u32 = Compact::<u32>::decode(data).unwrap().into();
            let len = len as usize;
            // let mut called = false;
            // visitor.visit_str(|| {
            //     called = true;
            //     std::str::from_utf8(&data[..len]).unwrap()
            // });
            visitor.visit(&stack, &data[..len], ty);
            *data = &data[len..];
        }
        TypeDef::Primitive(TypeDefPrimitive::Bool) => {
            // Always decode
            println!("bytes to decode: {:?}", &data);
            // let b = bool::decode(data).unwrap();

            visitor.visit(&stack, data, ty);
            *data = &data[1..];
        }
        TypeDef::Sequence(seq) => {
            let len: u64 = Compact::<u64>::decode(data).unwrap().into();
            let ty_id = seq.type_param();
            let ty = types.resolve(ty_id.id()).unwrap();
            println!("seq len = {}", len);
            for i in 0..len as usize {
                // println!("i = {}", i);println!("bytes left to decode start: {:?}", &data);
                stack.push(NUMS[i]);
                stack = semi_decode_aux(stack, data, ty, visitor, types);
                // println!("bytes left to decode end  : {:?}", &data);
                stack.pop();
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

        skeleton_decode(&encoded[..], id, &S {}, &types)
    }

    #[test]
    fn string_test() {
        let val = "hello string";
        let encoded = val.encode();

        let (id, types) = make_type::<&str>();

        skeleton_decode(&encoded[..], id, &S {}, &types)
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
            name: "skip me".into(),
        };
        let encoded = val.encode();

        let (id, types) = make_type::<X>();

        skeleton_decode(&encoded[..], id, &S {}, &types)
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
            val: false,             // 0
            name: "skip me".into(), // 28 len then 115, 107, 105, 112, 32, 109, 101
        };
        let y = Y {
            outer: vec![val, val2],
        };
        let encoded = y.encode();
        println!("bytes {:?}", encoded);

        let (id, types) = make_type::<Y>();

        skeleton_decode(&encoded[..], id, &S {}, &types)
    }
}
