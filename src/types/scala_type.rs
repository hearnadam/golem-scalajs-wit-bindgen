use std::fmt::Display;

use super::{Primitive, TypeMap, TypeName};

use wit_parser::Type as WitType;

pub enum Type {
    Primitive(Primitive),
    Custom(TypeName),
}

impl Type {
    pub fn from_wit(ty: WitType, type_map: &TypeMap) -> Self {
        match ty {
            WitType::Bool => Type::Primitive(Primitive::Boolean),
            WitType::U8 | WitType::S8 => Type::Primitive(Primitive::Byte),
            WitType::U16 | WitType::S16 => Type::Primitive(Primitive::Short),
            WitType::U32 | WitType::S32 => Type::Primitive(Primitive::Integer),
            WitType::U64 | WitType::S64 => Type::Primitive(Primitive::Long),
            WitType::Float32 => Type::Primitive(Primitive::Float),
            WitType::Float64 => Type::Primitive(Primitive::Double),
            WitType::Char => Type::Primitive(Primitive::Char),
            WitType::String => Type::Primitive(Primitive::String),
            WitType::Id(id) => Type::Custom(type_map.get(&id).unwrap().clone()),
        }
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Primitive(primitive) => write!(f, "{primitive}"),
            Type::Custom(type_name) => write!(f, "{type_name}"),
        }
    }
}