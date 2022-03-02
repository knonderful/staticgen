use super::tuple_util::TupleUtil;
use super::Error;
use super::Serializer;
use crate::model::FieldType;
use crate::model::Structured;
use serde::Serialize;
use std::borrow::Cow;
use std::io::Write;

pub struct TupleStructSerializer<'a, W> {
    serializer: &'a mut Serializer<W>,
    util: TupleUtil,
    name: Cow<'static, str>,
}

impl<'a, W> TupleStructSerializer<'a, W>
where
    W: Write,
{
    pub fn begin(
        serializer: &'a mut Serializer<W>,
        name: Cow<'static, str>,
        len: usize,
    ) -> Result<Self, Error> {
        serializer.writer.write(&name)?;
        let util = TupleUtil::begin(len, &mut serializer.writer)?;

        Ok(Self {
            serializer,
            util,
            name,
        })
    }
}

impl<'a, W> serde::ser::SerializeTupleStruct for TupleStructSerializer<'a, W>
where
    W: Write,
{
    type Ok = FieldType;
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        self.util.element_begin(&mut self.serializer.writer)?;
        let field_type = value.serialize(&mut *self.serializer)?;
        self.util
            .element_end(&mut self.serializer.writer, field_type)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        let built = self.util.end(&mut self.serializer.writer).map_err(|msg| {
            msg.message_prepend(format!("Could not build struct '{}'", self.name))
        })?;

        let structure = Structured::Tuple(built);
        self.serializer.structs_mut().merge(&self.name, structure)?;
        Ok(FieldType::Struct(self.name.into()))
    }
}
