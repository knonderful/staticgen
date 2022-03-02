mod key_serializer;

use super::Error;
use super::Serializer;
use crate::model::{
    FieldType, Structured, StructuredStruct, StructuredStructBuilder, StructuredTuple,
    StructuredTupleBuilder,
};
use crate::ser::map_serializer::key_serializer::MapKeySerializer;
use crate::ser::struct_util::StructUtil;
use crate::ser::tuple_util::TupleUtil;
use crate::ser::Structs;
use serde::Serialize;
use std::borrow::Cow;
use std::fmt::Debug;
use std::io::Write;

pub struct MapSerializer<'a, W> {
    serializer: &'a mut Serializer<W>,
    util: StructUtil,
    name: Cow<'static, str>,
    expected_len: Option<usize>,
    last_key: Option<String>,
}

impl<'a, W> MapSerializer<'a, W>
where
    W: Write,
{
    pub fn begin(
        serializer: &'a mut Serializer<W>,
        name: Cow<'static, str>,
        len: Option<usize>,
    ) -> Result<Self, Error> {
        let writer = &mut serializer.writer;
        writer.write(&name)?;

        // TODO: Remove this unwrap and make the length optional down to the builder...
        let util = StructUtil::begin(len.unwrap(), &mut serializer.writer)?;

        Ok(Self {
            serializer,
            util,
            name,
            expected_len: len,
            last_key: None,
        })
    }
}

impl<'a, W> serde::ser::SerializeMap for MapSerializer<'a, W>
where
    W: Write,
{
    type Ok = FieldType;
    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        let key = key.serialize(MapKeySerializer)?;
        if let Some(last_key) = &self.last_key {
            Err(Error::message(format!(
                "Got key \"{key}\" without receiving the value of the previous key (\"{last_key}\")."
            )))
        } else {
            self.last_key = Some(key);
            Ok(())
        }
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        if let Some(key) = self.last_key.take() {
            self.util.element_begin(&key, &mut self.serializer.writer)?;
            let field_type = value.serialize(&mut *self.serializer)?;
            self.util
                .element_end(key.into(), &mut self.serializer.writer, field_type)
        } else {
            Err(Error::message(
                "Got value without previously receiving a key for it.",
            ))
        }
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        let built = self.util.end(&mut self.serializer.writer).map_err(|msg| {
            msg.message_prepend(format!("Could not build struct '{}'", self.name))
        })?;

        let structure = Structured::Struct(built);
        self.serializer.structs_mut().merge(&self.name, structure)?;
        Ok(FieldType::Struct(self.name.into()))
    }
}
