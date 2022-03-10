use super::Error;
use crate::model::{FieldType, StructuredStruct, StructuredStructBuilder};
use crate::ser::CodeWriter;
use std::borrow::Cow;
use std::io::Write;

pub struct StructUtil {
    builder: StructuredStructBuilder,
}

impl StructUtil {
    pub fn begin<W>(len: usize, writer: &mut CodeWriter<W>) -> Result<Self, Error>
    where
        W: Write,
    {
        writer.struct_begin()?;

        Ok(Self {
            builder: StructuredStruct::builder(len),
        })
    }

    pub fn element_begin<W>(&mut self, name: &str, writer: &mut CodeWriter<W>) -> Result<(), Error>
    where
        W: Write,
    {
        writer.struct_entry_begin(name)?;
        Ok(())
    }

    pub fn element_end<W>(
        &mut self,
        name: Cow<'static, str>,
        writer: &mut CodeWriter<W>,
        field_type: FieldType,
    ) -> Result<(), Error>
    where
        W: Write,
    {
        writer.struct_entry_end()?;

        self.builder.field(name, field_type)
    }

    pub fn end<W>(self, writer: &mut CodeWriter<W>) -> Result<StructuredStruct, Error>
    where
        W: Write,
    {
        writer.struct_end()?;

        self.builder
            .build()
            .map_err(|msg| msg.message_prepend("Could not build tuple."))
    }
}
