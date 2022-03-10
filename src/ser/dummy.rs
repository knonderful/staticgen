use serde::ser::{
    SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple,
    SerializeTupleStruct, SerializeTupleVariant,
};
use serde::Serialize;

pub struct Dummy<Ok, Error>(Ok, Error);

impl<Ok, Error> SerializeSeq for Dummy<Ok, Error>
where
    Error: serde::ser::Error,
{
    type Ok = Ok;
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        unimplemented!("Should not be called.")
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!("Should not be called.")
    }
}

impl<Ok, Error> SerializeTuple for Dummy<Ok, Error>
where
    Error: serde::ser::Error,
{
    type Ok = Ok;
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        unimplemented!("Should not be called.")
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!("Should not be called.")
    }
}

impl<Ok, Error> SerializeTupleStruct for Dummy<Ok, Error>
where
    Error: serde::ser::Error,
{
    type Ok = Ok;
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        unimplemented!("Should not be called.")
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!("Should not be called.")
    }
}

impl<Ok, Error> SerializeTupleVariant for Dummy<Ok, Error>
where
    Error: serde::ser::Error,
{
    type Ok = Ok;
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        unimplemented!("Should not be called.")
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!("Should not be called.")
    }
}

impl<Ok, Error> SerializeMap for Dummy<Ok, Error>
where
    Error: serde::ser::Error,
{
    type Ok = Ok;
    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, _key: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        unimplemented!("Should not be called.")
    }

    fn serialize_value<T: ?Sized>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        unimplemented!("Should not be called.")
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!("Should not be called.")
    }
}

impl<Ok, Error> SerializeStruct for Dummy<Ok, Error>
where
    Error: serde::ser::Error,
{
    type Ok = Ok;
    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        _key: &'static str,
        _value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        unimplemented!("Should not be called.")
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!("Should not be called.")
    }
}

impl<Ok, Error> SerializeStructVariant for Dummy<Ok, Error>
where
    Error: serde::ser::Error,
{
    type Ok = Ok;
    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        _key: &'static str,
        _value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        unimplemented!("Should not be called.")
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!("Should not be called.")
    }
}
