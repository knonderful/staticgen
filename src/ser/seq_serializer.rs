use super::Error;
use super::Serializer;
use crate::model::FieldType;
use serde::Serialize;
use std::io::Write;

pub struct SeqSerializer<'a, W> {
    serializer: &'a mut Serializer<W>,
    field_type: Option<FieldType>,
    expected_len: Option<usize>,
    len: usize,
}

impl<'a, W> SeqSerializer<'a, W>
where
    W: Write,
{
    pub fn begin(serializer: &'a mut Serializer<W>, len: Option<usize>) -> Result<Self, Error> {
        serializer.writer.write("&")?;
        serializer.writer.seq_begin()?;

        Ok(Self {
            serializer,
            field_type: Option::None,
            expected_len: len,
            len: 0,
        })
    }
}

impl<'a, W> serde::ser::SerializeSeq for SeqSerializer<'a, W>
where
    W: Write,
{
    type Ok = FieldType;
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        self.len += 1;

        self.serializer.writer.seq_entry_begin()?;
        let field_type = value.serialize(&mut *self.serializer)?;
        self.serializer.writer.seq_entry_end()?;

        if let Some(existing) = &mut self.field_type {
            existing
                .merge(&field_type)
                .map_err(|err| err.message_prepend("Could not merge entries in sequence."))?;
        } else {
            self.field_type = Some(field_type);
        }

        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.serializer.writer.seq_end()?;

        if let Some(expected) = self.expected_len {
            let actual = self.len;
            if actual != expected {
                return Err(Error::message(format!(
                    "Expected sequence length of {expected}, but got {actual}"
                )));
            }
        }

        Ok(FieldType::Sequence(self.field_type.map(Box::new).into()))
    }
}
