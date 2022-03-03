#![allow(unused)]

use crate::ser::Error;
use std::borrow::{Borrow, Cow};
use std::cell::BorrowMutError;
use linked_hash_map::LinkedHashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct StructuredStructBuilder {
    fields: LinkedHashMap<Cow<'static, str>, FieldType>,
    target_len: usize,
}

impl StructuredStructBuilder {
    fn new(len: usize) -> Self {
        Self {
            fields: LinkedHashMap::with_capacity(len),
            target_len: len,
        }
    }

    pub fn field(&mut self, name: Cow<'static, str>, field_type: FieldType) -> Result<(), Error> {
        if self.fields.contains_key(&name) {
            Err(Error::Bug(format!(
                "Attempt at adding the same field twice: '{}'.",
                &name
            )))
        } else {
            self.fields.insert(name, field_type);
            Ok(())
        }
    }

    pub fn build(self) -> Result<StructuredStruct, Error> {
        let actual_len = self.fields.len();
        let expected_len = self.target_len;
        if actual_len != expected_len {
            Err(Error::message(format!(
                "Expected {} fields, but found {}.",
                expected_len, actual_len
            )))
        } else {
            Ok(StructuredStruct::new(self.fields))
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructuredStruct {
    fields: LinkedHashMap<Cow<'static, str>, FieldType>,
}

impl StructuredStruct {
    pub fn new(fields: LinkedHashMap<Cow<'static, str>, FieldType>) -> Self {
        Self { fields }
    }

    pub fn fields(&self) -> &LinkedHashMap<Cow<'static, str>, FieldType> {
        &self.fields
    }

    fn keys_to_string<'a>(keys: impl Iterator<Item = &'a Cow<'static, str>>) -> String {
        let mut out = String::from("[");
        let mut keys: Vec<&Cow<'static, str>> = keys.collect();
        keys.sort();

        for (i, key) in keys.iter().enumerate() {
            if i > 0 {
                out.push_str(", ");
            }
            out.push('"');
            out.push_str(&key);
            out.push('"');
        }
        out.push(']');
        out
    }

    pub fn merge(&mut self, other: &StructuredStruct) -> Result<(), Error> {
        if self.fields.keys().count() != other.fields.keys().count() {
            return Err(Error::message(format!(
                "Conflicting fields: {} vs {}.",
                Self::keys_to_string(self.fields.keys()),
                Self::keys_to_string(other.fields.keys()),
            )));
        }

        for (field, field_type) in &mut self.fields {
            if let Some(other_field_type) = other.fields.get(field) {
                field_type.merge(other_field_type).map_err(|msg| {
                    msg.message_prepend(format!("Could not merge field \"{}\".", field))
                })?;
            } else {
                return Err(Error::message(format!(
                    "Conflicting fields: {} vs {}.",
                    Self::keys_to_string(self.fields.keys()),
                    Self::keys_to_string(other.fields.keys()),
                )));
            }
        }

        Ok(())
    }

    pub fn builder(len: usize) -> StructuredStructBuilder {
        StructuredStructBuilder::new(len)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructuredTupleBuilder {
    elements: Vec<FieldType>,
    target_len: usize,
}

impl StructuredTupleBuilder {
    fn new(len: usize) -> Self {
        Self {
            elements: Vec::with_capacity(len),
            target_len: len,
        }
    }

    pub fn element(&mut self, field_type: FieldType) -> Result<(), Error> {
        self.elements.push(field_type);
        Ok(())
    }

    pub fn build(self) -> Result<StructuredTuple, Error> {
        let actual_len = self.elements.len();
        let expected_len = self.target_len;
        if actual_len != expected_len {
            Err(Error::message(format!(
                "Expected {} elements, but found {}.",
                expected_len, actual_len
            )))
        } else {
            Ok(StructuredTuple::new(self.elements))
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructuredTuple {
    elements: Vec<FieldType>,
}

impl StructuredTuple {
    pub fn new(elements: Vec<FieldType>) -> Self {
        Self { elements }
    }

    pub fn elements(&self) -> &[FieldType] {
        &self.elements
    }

    pub fn merge(&mut self, other: &StructuredTuple) -> Result<(), Error> {
        if self.elements.len() != other.elements.len() {
            return Err(Error::message(format!(
                "Conflicting tuple lengths: {} vs {}.",
                self.elements.len(),
                other.elements.len(),
            )));
        }

        for (index, (field_type, other_field_type)) in
            &mut self.elements.iter_mut().zip(&other.elements).enumerate()
        {
            field_type.merge(other_field_type).map_err(|msg| {
                msg.message_prepend(format!("Could not merge element with index {index}."))
            })?;
        }

        Ok(())
    }

    pub fn builder(len: usize) -> StructuredTupleBuilder {
        StructuredTupleBuilder::new(len)
    }

    pub fn take(self) -> Vec<FieldType> {
        self.elements
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Structured {
    Struct(StructuredStruct),
    Tuple(StructuredTuple),
    Unit,
}

impl Structured {
    fn conflicting_types(a: &str, b: &str) -> Result<(), Error> {
        Err(Error::message(format!(
            "Conflicting types: {} vs {}.",
            a, b
        )))
    }

    pub fn merge(&mut self, other: &Structured) -> Result<(), Error> {
        match self {
            Structured::Struct(inner) => match other {
                Structured::Struct(other_inner) => inner.merge(other_inner),
                Structured::Tuple(_) => {
                    Self::conflicting_types("Structured::Struct", "Structured::Tuple")
                }
                Structured::Unit => {
                    Self::conflicting_types("Structured::Struct", "Structured::Unit")
                }
            },
            Structured::Tuple(inner) => match other {
                Structured::Struct(_) => {
                    Self::conflicting_types("Structured::Tuple", "Structured::Struct")
                }
                Structured::Tuple(other_inner) => inner.merge(other_inner),
                Structured::Unit => {
                    Self::conflicting_types("Structured::Tuple", "Structured::Unit")
                }
            },
            Structured::Unit => match other {
                Structured::Struct(_) => {
                    Self::conflicting_types("Structured::Unit", "Structured::Struct")
                }
                Structured::Tuple(_) => {
                    Self::conflicting_types("Structured::Unit", "Structured::Tuple")
                }
                Structured::Unit => Ok(()),
            },
        }
    }
}

macro_rules! field_type_arg {
    ($name:ident ( $type:ty ), $ref_type:ty) => {
        #[derive(Clone, Debug, PartialEq)]
        pub struct $name($type);

        impl $name {
            fn new(value: $type) -> Self {
                Self(value)
            }

            pub fn value(&self) -> &$ref_type {
                &self.0
            }
        }

        impl From<$type> for $name {
            fn from(value: $type) -> Self {
                Self::new(value)
            }
        }
    };
}

field_type_arg!(StructArg(Cow<'static, str>), str);
field_type_arg!(EnumArg(Cow<'static, str>), str);
field_type_arg!(TupleArg(Vec<FieldType>), [FieldType]);

#[derive(Clone, Debug, PartialEq)]
pub struct OptionalTypeArg(Option<Box<FieldType>>);
impl OptionalTypeArg {
    fn new(value: Option<Box<FieldType>>) -> Self {
        Self(value)
    }

    pub fn value(&self) -> Option<&FieldType> {
        self.0.as_ref().map(Borrow::borrow)
    }

    fn merge(&mut self, other: &OptionalTypeArg) -> Result<(), MergeError> {
        if let Some(field_type) = self.0.as_mut().map(|v| v.as_mut()) {
            if let Some(other_field_type) = other.value() {
                return field_type.merge_internal(other_field_type);
            }
        } else {
            if let Some(other_field_type) = other.value() {
                self.0 = Some(Box::new(other_field_type.clone()))
            }
        }
        Ok(())
    }
}

impl From<Option<Box<FieldType>>> for OptionalTypeArg {
    fn from(value: Option<Box<FieldType>>) -> Self {
        Self::new(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum FieldType {
    Bool,
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
    F32,
    F64,
    Char,
    Str,
    Unit,
    Struct(StructArg),
    Enum(EnumArg),
    Tuple(TupleArg),
    Sequence(OptionalTypeArg),
    Option(OptionalTypeArg),
}

enum MergeError {
    ConflictingFieldTypes,
    ConflictingArguments,
}

impl From<MergeError> for Result<(), MergeError> {
    fn from(err: MergeError) -> Self {
        Err(err)
    }
}

impl FieldType {
    fn merge_internal(&mut self, other: &FieldType) -> Result<(), MergeError> {
        macro_rules! handle_primitive {
            ($variant:expr) => {{
                if other != &$variant {
                    return MergeError::ConflictingFieldTypes.into();
                }
            }};
        };

        match self {
            FieldType::Bool => handle_primitive!(FieldType::Bool),
            FieldType::U8 => handle_primitive!(FieldType::U8),
            FieldType::U16 => handle_primitive!(FieldType::U16),
            FieldType::U32 => handle_primitive!(FieldType::U32),
            FieldType::U64 => handle_primitive!(FieldType::U64),
            FieldType::I8 => handle_primitive!(FieldType::I8),
            FieldType::I16 => handle_primitive!(FieldType::I16),
            FieldType::I32 => handle_primitive!(FieldType::I32),
            FieldType::I64 => handle_primitive!(FieldType::I64),
            FieldType::F32 => handle_primitive!(FieldType::F32),
            FieldType::F64 => handle_primitive!(FieldType::F64),
            FieldType::Char => handle_primitive!(FieldType::Char),
            FieldType::Str => handle_primitive!(FieldType::Str),
            FieldType::Unit => handle_primitive!(FieldType::Unit),
            FieldType::Struct(arg) => {
                if let FieldType::Struct(other_arg) = other {
                    if other_arg != arg {
                        return MergeError::ConflictingArguments.into();
                    }
                } else {
                    return MergeError::ConflictingFieldTypes.into();
                }
            }
            FieldType::Enum(arg) => {
                if let FieldType::Enum(other_arg) = other {
                    if other_arg != arg {
                        return MergeError::ConflictingArguments.into();
                    }
                } else {
                    return MergeError::ConflictingFieldTypes.into();
                }
            }
            FieldType::Tuple(arg) => {
                if let FieldType::Tuple(other_arg) = other {
                    let entries = &mut arg.0;
                    let other_entries = &other_arg.0;
                    if entries.len() != other_entries.len() {
                        return MergeError::ConflictingArguments.into();
                    }
                    for (entry, other_entry) in entries.iter_mut().zip(other_entries.iter()) {
                        entry.merge_internal(other_entry)?;
                    }
                } else {
                    return MergeError::ConflictingFieldTypes.into();
                }
            }
            FieldType::Sequence(arg) => {
                return if let FieldType::Sequence(other_arg) = other {
                    arg.merge(other_arg)
                } else {
                    MergeError::ConflictingFieldTypes.into()
                }
            }
            FieldType::Option(arg) => {
                return if let FieldType::Option(other_arg) = other {
                    arg.merge(other_arg)
                } else {
                    MergeError::ConflictingFieldTypes.into()
                }
            }
        }

        Ok(())
    }

    pub fn merge(&mut self, other: &FieldType) -> Result<(), Error> {
        use MergeError::*;
        self.merge_internal(other).map_err(|err| match err {
            ConflictingFieldTypes => Error::message(format!(
                "Found conflicting field types: {:?} vs {:?}.",
                self, other
            )),
            ConflictingArguments => Error::message(format!(
                "Found conflicting arguments: {:?} vs {:?}.",
                self, other
            )),
        })
    }
}
