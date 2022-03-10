use super::Error;
use crate::ser::dummy::Dummy;
use serde::Serialize;

pub struct MapKeySerializer;

impl serde::ser::Serializer for MapKeySerializer {
    type Ok = String;
    type Error = Error;
    type SerializeSeq = Dummy<Self::Ok, Self::Error>;
    type SerializeTuple = Dummy<Self::Ok, Self::Error>;
    type SerializeTupleStruct = Dummy<Self::Ok, Self::Error>;
    type SerializeTupleVariant = Dummy<Self::Ok, Self::Error>;
    type SerializeMap = Dummy<Self::Ok, Self::Error>;
    type SerializeStruct = Dummy<Self::Ok, Self::Error>;
    type SerializeStructVariant = Dummy<Self::Ok, Self::Error>;

    fn serialize_bool(self, _v: bool) -> Result<Self::Ok, Self::Error> {
        unimplemented!("Should not be called.")
    }

    fn serialize_i8(self, _v: i8) -> Result<Self::Ok, Self::Error> {
        unimplemented!("Should not be called.")
    }

    fn serialize_i16(self, _v: i16) -> Result<Self::Ok, Self::Error> {
        unimplemented!("Should not be called.")
    }

    fn serialize_i32(self, _v: i32) -> Result<Self::Ok, Self::Error> {
        unimplemented!("Should not be called.")
    }

    fn serialize_i64(self, _v: i64) -> Result<Self::Ok, Self::Error> {
        unimplemented!("Should not be called.")
    }

    fn serialize_u8(self, _v: u8) -> Result<Self::Ok, Self::Error> {
        unimplemented!("Should not be called.")
    }

    fn serialize_u16(self, _v: u16) -> Result<Self::Ok, Self::Error> {
        unimplemented!("Should not be called.")
    }

    fn serialize_u32(self, _v: u32) -> Result<Self::Ok, Self::Error> {
        unimplemented!("Should not be called.")
    }

    fn serialize_u64(self, _v: u64) -> Result<Self::Ok, Self::Error> {
        unimplemented!("Should not be called.")
    }

    fn serialize_f32(self, _v: f32) -> Result<Self::Ok, Self::Error> {
        unimplemented!("Should not be called.")
    }

    fn serialize_f64(self, _v: f64) -> Result<Self::Ok, Self::Error> {
        unimplemented!("Should not be called.")
    }

    fn serialize_char(self, _v: char) -> Result<Self::Ok, Self::Error> {
        unimplemented!("Should not be called.")
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        // TODO: We could (only) avoid a copy here if we write directly in the output, instead of
        //       returning the raw thing.
        Ok(String::from(v))
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> {
        unimplemented!("Should not be called.")
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!("Should not be called.")
    }

    fn serialize_some<T: ?Sized>(self, _value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        unimplemented!("Should not be called.")
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!("Should not be called.")
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        unimplemented!("Should not be called.")
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        unimplemented!("Should not be called.")
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        unimplemented!("Should not be called.")
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        unimplemented!("Should not be called.")
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        unimplemented!("Should not be called.")
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        unimplemented!("Should not be called.")
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        unimplemented!("Should not be called.")
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        unimplemented!("Should not be called.")
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        unimplemented!("Should not be called.")
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        unimplemented!("Should not be called.")
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        unimplemented!("Should not be called.")
    }
}
