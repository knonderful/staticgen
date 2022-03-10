use super::Error;
use super::Serializer;
use crate::model::{FieldType, Structured};
use crate::ser::struct_util::StructUtil;
use serde::Serialize;
use std::borrow::Cow;
use std::io::Write;

pub struct StructVariantSerializer<'a, W> {
    serializer: &'a mut Serializer<W>,
    util: StructUtil,
    name: Cow<'static, str>,
    variant: Cow<'static, str>,
}

impl<'a, W> StructVariantSerializer<'a, W>
where
    W: Write,
{
    pub fn begin(
        serializer: &'a mut Serializer<W>,
        name: Cow<'static, str>,
        variant: Cow<'static, str>,
        len: usize,
    ) -> Result<Self, Error> {
        let writer = &mut serializer.writer;
        writer.write(&name)?;
        writer.write("::")?;
        writer.write(&variant)?;

        let util = StructUtil::begin(len, &mut serializer.writer)?;

        Ok(Self {
            serializer,
            util,
            name,
            variant,
        })
    }
}

impl<'a, W> serde::ser::SerializeStructVariant for StructVariantSerializer<'a, W>
where
    W: Write,
{
    type Ok = FieldType;
    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        self.util.element_begin(key, &mut self.serializer.writer)?;
        let field_type = value.serialize(&mut *self.serializer)?;
        self.util
            .element_end(key.into(), &mut self.serializer.writer, field_type)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        let built = self.util.end(&mut self.serializer.writer).map_err(|msg| {
            msg.message_prepend(format!("Could not build struct '{}'", self.name))
        })?;

        let structure = Structured::Struct(built);
        self.serializer
            .enums_mut()
            .merge(&self.name, &self.variant, structure)?;
        Ok(FieldType::Enum(self.name.into()))
    }
}
