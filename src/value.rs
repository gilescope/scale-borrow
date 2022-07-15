use std::default;

use scale_info::interner::UntrackedSymbol;
use scale_info::prelude::any::TypeId;
use scale_info::TypeDef;
use scale_info::TypeDefPrimitive;

/// The underlying shape of a given value.
#[derive(Clone, Debug, PartialEq)]
pub enum Value<'scale> {
    /// A named or unnamed struct-like, array-like or tuple-like set of values.
    Object(Box<Vec<(&'scale str, Value<'scale>)>>),
    // // UnamedComposite(&'scale Vec<Value<T>>)
    // /// An enum variant.
    // Variant(&'scale (&'scale str, &'scale Value<'scale>)),
    // Truth
    Bool(bool),
    Char(char),
    Str(&'scale str),
    Scale(&'scale [u8]),
    U32(u32),
    U64(u64),
    U128(&'scale u128),
    I128(&'scale i128),
    // // / An unsigned 256 bit number (internally represented as a 32 byte array).
    U256(&'scale [u8; 32]),
    // // / A signed 256 bit number (internally represented as a 32 byte array).
    I256(&'scale [u8; 32]),
}

#[derive(Default)]
pub struct ValueBuilder<'scale> {
    root: Option<Value<'scale>>,
}

impl<'scale> ValueBuilder<'scale> {
    pub fn parse(
        data: &'scale [u8],
        top_type: UntrackedSymbol<TypeId>,
        types: &scale_info::PortableRegistry,
    ) -> Value<'scale> {
        let mut slf = ValueBuilder::<'scale>::default();
        crate::skeleton_decode(data, top_type, &mut slf, types);
        slf.root.take().unwrap()
    }

    fn append(
        path: &[&'scale str],
        current: &mut Value<'scale>,
        new_field: &'scale str,
        new_val: Value<'scale>,
    ) {
        if let Value::<'scale>::Object(fields) = current {
            if path.is_empty() {
                fields.push((new_field, new_val));
                return;
            }

            for (field, child) in fields.iter_mut() {
                if *field == path[0] {
                    ValueBuilder::append(&path[..path.len() - 1], child, new_field, new_val);
                    return;
                }
            }

            fields.push((path[0], Value::Object(Box::new(Vec::<_>::new()))));

            ValueBuilder::append(&path[..path.len() - 1], current, new_field, new_val);
        } else {
            panic!()
        }
    }
}

impl<'scale> super::VisitScale<'scale> for ValueBuilder<'scale> {
    fn visit(
        &mut self,
        current_path: &Vec<&'scale str>,
        data: &'scale [u8],
        ty: &scale_info::Type<scale_info::form::PortableForm>,
    ) {
        // $(
        // let p: Vec<_> = $path.split('.').collect();
        // // println!("visited path {:?} == {:?}", path, p);
        // if *current_path == p {
        let new_val = match ty.type_def() {
            scale_info::TypeDef::Primitive(TypeDefPrimitive::Str) => Some(Value::Str(
                &<&'scale str as crate::borrow_decode::BorrowDecode>::borrow_decode(data),
            )),
            scale_info::TypeDef::Primitive(TypeDefPrimitive::Bool) => Some(Value::Bool(
                <bool as crate::borrow_decode::BorrowDecode>::borrow_decode(data),
            )),
            _ => {
                panic!("skipping {:?}", ty);
            }
        };

        // place val in right location.
        if self.root.is_none() {
            if current_path.is_empty() {
                self.root = new_val;
                return;
            }
            self.root = Some(Value::Object(Box::new(Vec::<_>::new())));
        }
        // let mut val: &mut Value = self.root.as_mut().unwrap();
        // // Calculate insertion point
        // for p in current_path.iter().take(current_path.len() - 1) {
        //     if let Value::Object(fields) = &mut val {
        //         let mut found = false;
        //         for (field, child) in fields.iter_mut() {
        //             if field == p {
        //                 // already exists.
        //                 // val = child;
        //                 found = true;
        //                 break;
        //             }
        //         }
        //         // if !found {
        //         //     fields.push((p, Value::Object(Box::new(Vec::<_>::new()))));
        //         //     val = &mut fields.last().unwrap().1;
        //         //     // we can continue creating at this point...
        //         // }
        //     } else {
        //         panic!();
        //     }
        // }

        ValueBuilder::append(
            &current_path[..current_path.len() - 1],
            &mut self.root.as_mut().unwrap(),
            &current_path.last().unwrap(),
            new_val.unwrap(),
        );

        // if let Value::Object(fields) = val {
        //     fields.push((current_path.last().unwrap(), new_val.unwrap()));
        // } else { panic!() }

        // self.root = Some(val);

        // })+
    }
}
