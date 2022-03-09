#![allow(unused)]

mod dummy;
mod map_serializer;
mod seq_serializer;
mod struct_serializer;
mod struct_util;
mod struct_variant_serializer;
mod tuple_serializer;
mod tuple_struct_serializer;
mod tuple_util;
mod tuple_variant_serializer;

#[cfg(test)]
mod test;

use crate::model::{FieldType, Structured};
use crate::ser::dummy::Dummy;
use crate::ser::map_serializer::MapSerializer;
use crate::ser::seq_serializer::SeqSerializer;
use crate::ser::struct_serializer::StructSerializer;
use crate::ser::struct_variant_serializer::StructVariantSerializer;
use crate::ser::tuple_serializer::TupleSerializer;
use crate::ser::tuple_struct_serializer::TupleStructSerializer;
use crate::ser::tuple_variant_serializer::TupleVariantSerializer;
use linked_hash_map::LinkedHashMap;
use serde::Serialize;
use std::borrow::Cow;
use std::fmt::{Debug, Display, Formatter};
use std::io::Write;

#[derive(Debug)]
pub enum Error {
    Message(Cow<'static, str>),
    Io(std::io::Error),
    Bug(String),
}

impl Error {
    pub fn message(message: impl Into<Cow<'static, str>>) -> Self {
        Error::Message(message.into())
    }

    pub fn message_prepend(self, prepend: impl Into<Cow<'static, str>>) -> Self {
        if let Error::Message(msg) = &self {
            Error::Message(format!("{} {}", prepend.into(), msg).into())
        } else {
            self
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Message(msg) => {
                write!(f, "{}", msg)
            }
            Error::Io(err) => {
                write!(f, "{}", err)
            }
            Error::Bug(msg) => {
                write!(f, "{}", msg)
            }
        }
    }
}

impl serde::ser::StdError for Error {}

impl serde::ser::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Error::message(msg.to_string())
    }
}

type CodeWriterResult = std::io::Result<()>;

pub struct CodeWriter<W> {
    out: W,
}

impl<W> CodeWriter<W> {
    pub fn new(out: W) -> Self {
        Self { out }
    }
}

impl<W> CodeWriter<W>
where
    W: Write,
{
    pub fn write(&mut self, text: &str) -> CodeWriterResult {
        self.out.write_all(text.as_bytes())
    }

    pub fn struct_begin(&mut self) -> CodeWriterResult {
        self.write(" {")
    }

    pub fn struct_end(&mut self) -> CodeWriterResult {
        self.write(" }")
    }

    pub fn struct_entry_begin(&mut self, field_name: &str) -> CodeWriterResult {
        self.write(field_name)?;
        self.write(": ")
    }

    pub fn struct_entry_end(&mut self) -> CodeWriterResult {
        self.write(", ")
    }

    pub fn tuple_begin(&mut self) -> CodeWriterResult {
        self.write("(")
    }

    pub fn tuple_end(&mut self) -> CodeWriterResult {
        self.write(")")
    }

    pub fn tuple_entry_begin(&mut self) -> CodeWriterResult {
        Ok(())
    }

    pub fn tuple_entry_end(&mut self) -> CodeWriterResult {
        self.write(", ")
    }

    pub fn seq_begin(&mut self) -> CodeWriterResult {
        self.write("[")
    }

    pub fn seq_end(&mut self) -> CodeWriterResult {
        self.write("]")
    }

    pub fn seq_entry_begin(&mut self) -> CodeWriterResult {
        Ok(())
    }

    pub fn seq_entry_end(&mut self) -> CodeWriterResult {
        self.write(", ")
    }
}

#[derive(Clone, Default, Debug)]
pub struct Structs(LinkedHashMap<Cow<'static, str>, Structured>);

struct CodeWriteContext<'a, W>
where
    W: Write,
{
    writer: &'a mut CodeWriter<W>,
    pub_fields: bool,
}

impl<'a, W> CodeWriteContext<'a, W>
where
    W: Write,
{
    fn new(writer: &'a mut CodeWriter<W>, pub_fields: bool) -> Self {
        Self { writer, pub_fields }
    }
}

trait CodeWrite {
    fn write<W>(&self, ctx: CodeWriteContext<W>) -> CodeWriterResult
    where
        W: Write;
}

impl CodeWrite for Structured {
    fn write<W>(&self, ctx: CodeWriteContext<W>) -> CodeWriterResult
    where
        W: Write,
    {
        let writer = ctx.writer;
        match self {
            Structured::Struct(inner) => {
                writer.struct_begin()?;
                for (field_name, field_type) in inner.fields() {
                    if ctx.pub_fields {
                        writer.struct_entry_begin(&format!("pub {}", field_name))?;
                    } else {
                        writer.struct_entry_begin(field_name)?;
                    }
                    field_type.write(CodeWriteContext::new(writer, ctx.pub_fields))?;
                    writer.struct_entry_end()?;
                }
                writer.struct_end()?;
            }
            Structured::Tuple(inner) => {
                writer.tuple_begin()?;
                for field_type in inner.elements() {
                    writer.tuple_entry_begin()?;
                    if ctx.pub_fields {
                        writer.write("pub ")?;
                    }
                    field_type.write(CodeWriteContext::new(writer, ctx.pub_fields))?;
                    writer.tuple_entry_end()?;
                }
                writer.tuple_end()?;
            }
            Structured::Unit => {}
        }
        Ok(())
    }
}

impl CodeWrite for FieldType {
    fn write<W>(&self, ctx: CodeWriteContext<W>) -> CodeWriterResult
    where
        W: Write,
    {
        let writer = ctx.writer;
        match self {
            FieldType::Bool => writer.write("bool"),
            FieldType::U8 => writer.write("u8"),
            FieldType::U16 => writer.write("u16"),
            FieldType::U32 => writer.write("u32"),
            FieldType::U64 => writer.write("u64"),
            FieldType::I8 => writer.write("i8"),
            FieldType::I16 => writer.write("i16"),
            FieldType::I32 => writer.write("i32"),
            FieldType::I64 => writer.write("i64"),
            FieldType::F32 => writer.write("f32"),
            FieldType::F64 => writer.write("f64"),
            FieldType::Char => writer.write("char"),
            FieldType::Str => writer.write("&'static str"),
            FieldType::Unit => writer.write("()"),
            FieldType::Struct(arg) => writer.write(arg.value()),
            FieldType::Enum(arg) => writer.write(arg.value()),
            FieldType::Tuple(arg) => {
                writer.tuple_begin()?;
                for field_type in arg.value() {
                    writer.tuple_entry_begin()?;
                    field_type.write(CodeWriteContext::new(writer, ctx.pub_fields))?;
                    writer.tuple_entry_end()?;
                }
                writer.tuple_end()
            }
            FieldType::Sequence(arg) => {
                writer.write("&'static ")?;
                writer.seq_begin()?;
                arg.value()
                    .write(CodeWriteContext::new(writer, ctx.pub_fields))?;
                writer.seq_end()
            }
            FieldType::Option(arg) => {
                writer.write("Option<")?;
                arg.value()
                    .write(CodeWriteContext::new(writer, ctx.pub_fields))?;
                writer.write(">")
            }
        }
    }
}

impl CodeWrite for Option<&FieldType> {
    fn write<W>(&self, ctx: CodeWriteContext<W>) -> CodeWriterResult
    where
        W: Write,
    {
        let field_type = self.unwrap_or(&FieldType::Unit);
        field_type.write(ctx)
    }
}

impl Structs {
    pub fn merge(&mut self, name: &Cow<'static, str>, structure: Structured) -> Result<(), Error> {
        if let Some(existing_structure) = self.0.get_mut(name) {
            existing_structure
                .merge(&structure)
                .map_err(|err| err.message_prepend(format!("Error merging struct \"{}\".", &name)))
        } else {
            self.0.insert(name.clone(), structure);
            Ok(())
        }
    }

    pub fn write(&self, out: &mut impl Write) -> std::io::Result<()> {
        let mut writer = CodeWriter::new(out);
        for (name, structure) in self.0.iter() {
            writer.write("#[derive(Clone, Debug, PartialEq)] pub struct ")?;
            writer.write(name)?;
            structure.write(CodeWriteContext::new(&mut writer, true))?;

            match structure {
                Structured::Struct(_) => {}
                Structured::Tuple(_) => writer.write(";")?,
                Structured::Unit => writer.write(";")?,
            }
        }
        Ok(())
    }
}

#[derive(Clone, Default, Debug)]
pub struct Enums(LinkedHashMap<Cow<'static, str>, LinkedHashMap<Cow<'static, str>, Structured>>);

impl Enums {
    pub fn merge(
        &mut self,
        name: &Cow<'static, str>,
        variant: &Cow<'static, str>,
        structure: Structured,
    ) -> Result<(), Error> {
        if let Some(existing_enum) = self.0.get_mut(name) {
            if let Some(existing_structure) = existing_enum.get_mut(variant) {
                existing_structure.merge(&structure).map_err(|err| {
                    err.message_prepend(format!("Error merging variant \"{}::{}\".", name, variant))
                })
            } else {
                existing_enum.insert(variant.clone(), structure);
                Ok(())
            }
        } else {
            let mut new_enum = LinkedHashMap::new();
            new_enum.insert(variant.clone(), structure);
            self.0.insert(name.clone(), new_enum);
            Ok(())
        }
    }

    pub fn write(&self, out: &mut impl Write) -> std::io::Result<()> {
        let mut writer = CodeWriter::new(out);
        for (name, variants) in self.0.iter() {
            writer.write("#[derive(Clone, Debug, PartialEq)] pub enum ")?;
            writer.write(name)?;
            writer.struct_begin()?;
            for (variant, structure) in variants {
                writer.write(variant)?;
                structure.write(CodeWriteContext::new(&mut writer, false))?;
                writer.write(", ")?;
            }
            writer.struct_end()?;
        }
        Ok(())
    }
}

pub struct Serializer<W> {
    writer: CodeWriter<W>,
    structs: Structs,
    enums: Enums,
    generated_struct_seed: usize,
}

impl<W> Serializer<W> {
    pub fn new(output: W) -> Self {
        Self {
            writer: CodeWriter::new(output),
            structs: Default::default(),
            enums: Default::default(),
            generated_struct_seed: 0,
        }
    }

    pub fn out_mut(&mut self) -> &mut W {
        &mut self.writer.out
    }

    pub fn structs(&self) -> &Structs {
        &self.structs
    }

    pub fn structs_mut(&mut self) -> &mut Structs {
        &mut self.structs
    }

    pub fn enums(&self) -> &Enums {
        &self.enums
    }

    pub fn enums_mut(&mut self) -> &mut Enums {
        &mut self.enums
    }
}

impl<W> Serializer<W>
where
    W: Write,
{
    fn write_int<I>(&mut self, value: I) -> Result<(), Error>
    where
        I: itoa::Integer,
    {
        self.writer.write(itoa::Buffer::new().format(value))?;
        Ok(())
    }

    fn write_float<F>(&mut self, value: F) -> Result<(), Error>
    where
        F: dtoa::Float,
    {
        self.writer.write(dtoa::Buffer::new().format(value))?;
        Ok(())
    }
}

impl<'a, W> serde::ser::Serializer for &'a mut Serializer<W>
where
    W: Write,
{
    type Ok = FieldType;
    type Error = Error;
    type SerializeSeq = SeqSerializer<'a, W>;
    type SerializeTuple = TupleSerializer<'a, W>;
    type SerializeTupleStruct = TupleStructSerializer<'a, W>;
    type SerializeTupleVariant = TupleVariantSerializer<'a, W>;
    type SerializeMap = MapSerializer<'a, W>;
    type SerializeStruct = StructSerializer<'a, W>;
    type SerializeStructVariant = StructVariantSerializer<'a, W>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        let out = if v { "true" } else { "false" };
        self.writer.write(out)?;
        Ok(FieldType::Bool)
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.write_int(v)?;
        Ok(FieldType::I8)
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.write_int(v)?;
        Ok(FieldType::I16)
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.write_int(v)?;
        Ok(FieldType::I32)
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.write_int(v)?;
        Ok(FieldType::I64)
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.write_int(v)?;
        Ok(FieldType::U8)
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.write_int(v)?;
        Ok(FieldType::U16)
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.write_int(v)?;
        Ok(FieldType::U32)
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.write_int(v)?;
        Ok(FieldType::U64)
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.write_float(v)?;
        Ok(FieldType::F32)
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        self.write_float(v)?;
        Ok(FieldType::F64)
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        let string = format!("'{v}'");
        self.writer.write(string.as_str())?;
        Ok(FieldType::Char)
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.writer.write("\"")?;
        self.writer.write(v)?;
        self.writer.write("\"")?;
        Ok(FieldType::Str)
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        use serde::ser::SerializeSeq;
        let mut seq = self.serialize_seq(Some(v.len()))?;
        for byte in v {
            seq.serialize_element(byte)?;
        }
        seq.end()
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.writer.write("None")?;
        Ok(FieldType::Option(None.into()))
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.writer.write("Some(")?;
        let field_type = value.serialize(&mut *self)?;
        self.writer.write(")")?;
        Ok(FieldType::Option(Some(Box::new(field_type)).into()))
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        self.writer.write("()")?;
        Ok(FieldType::Unit)
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.writer.write(name)?;

        let name = Cow::Borrowed(name);
        let structure = Structured::Unit;
        self.structs_mut().merge(&name, structure)?;

        Ok(FieldType::Struct(name.into()))
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.writer.write(name)?;
        self.writer.write("::")?;
        self.writer.write(variant)?;

        let structure = Structured::Unit;
        self.enums.merge(&name.into(), &variant.into(), structure)?;
        Ok(FieldType::Enum(Cow::Borrowed(name).into()))
    }

    fn serialize_newtype_struct<T>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        use serde::ser::SerializeTupleStruct as _;
        let mut ser = self.serialize_tuple_struct(name, 1)?;
        ser.serialize_field(value)?;
        ser.end()
    }

    fn serialize_newtype_variant<T>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        use serde::ser::SerializeTupleVariant as _;
        let mut ser = self.serialize_tuple_variant(name, variant_index, variant, 1)?;
        ser.serialize_field(value)?;
        ser.end()
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        SeqSerializer::begin(self, len)
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        TupleSerializer::begin(self, len)
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        TupleStructSerializer::begin(self, name.into(), len)
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        TupleVariantSerializer::begin(self, name.into(), variant.into(), len)
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        self.generated_struct_seed += 1;
        let name = format!("Generated{}", self.generated_struct_seed);
        MapSerializer::begin(self, name.into(), len)
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        StructSerializer::begin(self, name.into(), len)
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        StructVariantSerializer::begin(self, name.into(), variant.into(), len)
    }
}
