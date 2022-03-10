use super::Error;
use crate::model::{FieldType, StructuredTuple, StructuredTupleBuilder};
use crate::ser::CodeWriter;
use std::io::Write;

pub struct TupleUtil {
    builder: StructuredTupleBuilder,
}

impl TupleUtil {
    pub fn begin<W>(len: usize, writer: &mut CodeWriter<W>) -> Result<Self, Error>
    where
        W: Write,
    {
        writer.tuple_begin()?;

        Ok(Self {
            builder: StructuredTuple::builder(len),
        })
    }

    pub fn element_begin<W>(&mut self, writer: &mut CodeWriter<W>) -> Result<(), Error>
    where
        W: Write,
    {
        writer.tuple_entry_begin()?;
        Ok(())
    }

    pub fn element_end<W>(
        &mut self,
        writer: &mut CodeWriter<W>,
        field_type: FieldType,
    ) -> Result<(), Error>
    where
        W: Write,
    {
        writer.tuple_entry_end()?;

        self.builder.element(field_type)
    }

    pub fn end<W>(self, writer: &mut CodeWriter<W>) -> Result<StructuredTuple, Error>
    where
        W: Write,
    {
        writer.tuple_end()?;

        self.builder
            .build()
            .map_err(|msg| msg.message_prepend("Could not build tuple."))
    }
}
