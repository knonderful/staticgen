use super::Error;
use super::Serializer;
use crate::model::{FieldType, StructuredTuple, StructuredTupleBuilder};
use crate::ser::tuple_util::TupleUtil;
use serde::Serialize;
use std::borrow::Cow;
use std::io::Write;

pub struct TupleSerializer<'a, W> {
    serializer: &'a mut Serializer<W>,
    util: TupleUtil,
}

impl<'a, W> TupleSerializer<'a, W>
where
    W: Write,
{
    pub fn begin(serializer: &'a mut Serializer<W>, len: usize) -> Result<Self, Error> {
        let util = TupleUtil::begin(len, &mut serializer.writer)?;

        Ok(Self { serializer, util })
    }
}

impl<'a, W> serde::ser::SerializeTuple for TupleSerializer<'a, W>
where
    W: Write,
{
    type Ok = FieldType;
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        self.util.element_begin(&mut self.serializer.writer)?;
        let field_type = value.serialize(&mut *self.serializer)?;
        self.util
            .element_end(&mut self.serializer.writer, field_type)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        let built = self
            .util
            .end(&mut self.serializer.writer)
            .map_err(|msg| msg.message_prepend("Could not build tuple."))?;

        Ok(FieldType::Tuple(built.take().into()))
    }
}
