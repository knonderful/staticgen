pub struct TestStruct3(pub u32);
pub struct TestStruct2 {
    pub u16: Option<u16>,
    pub test_struct_3: TestStruct3,
}
pub struct TestStruct {
    pub string: &'static str,
    pub test_struct_2: TestStruct2,
}
pub struct Generated1 {
    pub alpha: TestStruct2,
    pub beta: TestStruct2,
    pub gamma: TestStruct2,
}
pub struct TestData {
    pub u8: u8,
    pub u16: u16,
    pub u32: u32,
    pub u64: u64,
    pub i8: i8,
    pub i16: i16,
    pub i32: i32,
    pub i64: i64,
    pub f32: f32,
    pub f64: f64,
    pub bool: bool,
    pub char: char,
    pub string: &'static str,
    pub unit: (),
    pub tuple: (u8, (u16, u16), &'static str),
    pub test_struct: TestStruct,
    pub test_enum: TestEnum,
    pub seq: &'static [u8],
    pub option: Option<u8>,
    pub map: Generated1,
    pub enum_variants: &'static [TestEnum2],
    pub undefined_vec: &'static [()],
    pub undefined_option: Option<()>,
}
pub enum TestEnum {
    UnitVariant,
}
pub enum TestEnum2 {
    Rgba { r: u8, g: u8, b: u8, a: u8 },
    Compressed(&'static [u8]),
}
