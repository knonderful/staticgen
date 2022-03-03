use super::{CodeWriter, FieldType, Serializer};
use crate::ser::Error;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use std::io::{Read, Write};
use std::process::ExitStatus;
use std::ptr::write;

fn create_serializer() -> Serializer<Vec<u8>> {
    Serializer::new(Vec::new())
}

fn create_string(serializer: &Serializer<Vec<u8>>) -> String {
    String::from_utf8(serializer.writer.out.clone()).unwrap()
}

fn assert_struct(field_type: &FieldType) {
    match field_type {
        FieldType::Struct(_) => {}
        variant => assert!(
            false,
            "Expected FieldType::Struct(_), but got FieldType::{:?}",
            variant
        ),
    }
}

macro_rules! test_integer {
    ($primitive_type:ty, $value:expr, $field_type_variant:ident) => {
        paste::paste! {
            #[test]
            fn [<test_ $primitive_type>]() {
                let mut serializer = create_serializer();
                let value: $primitive_type = $value;
                let field_type = value.serialize(&mut serializer).unwrap();
                assert_eq!(FieldType::$field_type_variant, field_type);
                let output = create_string(&serializer);
                assert_eq!(stringify!($value), output);
            }
        }
    };
}

test_integer!(u8, 18, U8);
test_integer!(u16, 123, U16);
test_integer!(u32, 31278, U32);
test_integer!(u64, 38128731, U64);
test_integer!(i8, -18, I8);
test_integer!(i16, -123, I16);
test_integer!(i32, -31278, I32);
test_integer!(i64, -38128731, I64);

#[derive(Serialize)]
struct Simple {
    string: String,
    static_str: &'static str,
    int_u8: u8,
    int_i64: i64,
    boolean: bool,
}

#[derive(Serialize)]
struct Container {
    name: String,
    simple_a: Simple,
    simple_b: Simple,
}

#[derive(Serialize)]
enum EnumTest {
    UnitVariant,
    StructVariant { width: usize, simple: Simple },
    TupleVariant(usize, Simple),
}

#[derive(Serialize)]
struct SimpleWithEnum {
    simple: Simple,
    enum_test: EnumTest,
}

#[derive(Serialize)]
struct SimpleWithTuple {
    simple: Simple,
    tuple: (u8, SimpleWithEnum, (String, String)),
}

#[derive(Serialize)]
struct UnitStruct;

#[derive(Serialize)]
struct TupleStruct(u8, String, Simple);

#[test]
fn test_conflicting_structs_field_count() {
    // We define another "Simple" type, which we can only do by putting it in another mod.
    pub mod temp {
        #[derive(super::Serialize)]
        pub struct Simple {
            pub a: usize,
        }
    }

    #[derive(Serialize)]
    struct Conflict {
        simple: Simple,
        temp_simple: temp::Simple,
    }

    let value = Conflict {
        simple: Simple {
            string: "Bye".to_string(),
            static_str: "cruel world",
            int_u8: 12,
            int_i64: 1276,
            boolean: false,
        },
        temp_simple: temp::Simple { a: 990011 },
    };

    let mut serializer = create_serializer();

    let result = value.serialize(&mut serializer);
    if let Err(Error::Message(msg)) = result {
        assert_eq!(msg, "Error merging struct \"Simple\". Conflicting fields: [\"boolean\", \"int_i64\", \"int_u8\", \"static_str\", \"string\"] vs [\"a\"].");
    } else {
        assert!(false, "Expected failure");
    }
}

#[test]
fn test_conflicting_structs_field_names() {
    // We define another "Simple" type, which we can only do by putting it in another mod.
    pub mod temp {
        #[derive(super::Serialize)]
        pub struct Simple {
            pub string: String,
            pub static_str: &'static str,
            pub int_u8: u8,
            pub int_i64: i64,
            pub bowlean: bool,
        }
    }

    #[derive(Serialize)]
    struct Conflict {
        simple: Simple,
        temp_simple: temp::Simple,
    }

    let value = Conflict {
        simple: Simple {
            string: "Bye".to_string(),
            static_str: "cruel world",
            int_u8: 12,
            int_i64: 1276,
            boolean: false,
        },
        temp_simple: temp::Simple {
            string: "Hello".to_string(),
            static_str: "drapes",
            int_u8: 122,
            int_i64: 1176,
            bowlean: false,
        },
    };

    let mut serializer = create_serializer();

    let result = value.serialize(&mut serializer);
    if let Err(Error::Message(msg)) = result {
        assert_eq!(msg, "Error merging struct \"Simple\". Conflicting fields: [\"boolean\", \"int_i64\", \"int_u8\", \"static_str\", \"string\"] vs [\"bowlean\", \"int_i64\", \"int_u8\", \"static_str\", \"string\"].");
    } else {
        assert!(false, "Expected failure");
    }
}

#[test]
fn test_conflicting_structs_field_types() {
    // We define another "Simple" type, which we can only do by putting it in another mod.
    pub mod temp {
        #[derive(super::Serialize)]
        pub struct Simple {
            pub string: String,
            pub static_str: &'static str,
            pub int_u8: u16,
            pub int_i64: i64,
            pub boolean: bool,
        }
    }

    #[derive(Serialize)]
    struct Conflict {
        simple: Simple,
        temp_simple: temp::Simple,
    }

    let value = Conflict {
        simple: Simple {
            string: "Bye".to_string(),
            static_str: "cruel world",
            int_u8: 12,
            int_i64: 1276,
            boolean: false,
        },
        temp_simple: temp::Simple {
            string: "Hello".to_string(),
            static_str: "drapes",
            int_u8: 122,
            int_i64: 1176,
            boolean: false,
        },
    };

    let mut serializer = create_serializer();

    let result = value.serialize(&mut serializer);
    if let Err(Error::Message(msg)) = result {
        assert_eq!(msg, "Error merging struct \"Simple\". Could not merge field \"int_u8\". Found conflicting field types: U8 vs U16.");
    } else {
        assert!(false, "Expected failure");
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct TestStruct {
    string: String,
    test_struct_2: TestStruct2,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct TestStruct2 {
    u16: Option<u16>,
    test_struct_3: TestStruct3,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct TestStruct3(u32);

#[derive(Clone, Debug, Serialize, Deserialize)]
enum TestEnum {
    UnitVariant,
    StructVariant { string: String, u8: u8 },
    TupleVariant(u16, Vec<u8>),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
enum TestEnum2 {
    Transparent,
    Rgb { r: u8, g: u8, b: u8 },
    Rgba { r: u8, g: u8, b: u8, a: u8 },
    Binary(Vec<u8>),
    Compressed(Vec<u8>),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct TestData {
    u8: u8,
    u16: u16,
    u32: u32,
    u64: u64,
    i8: i8,
    i16: i16,
    i32: i32,
    i64: i64,
    f32: f32,
    f64: f64,
    bool: bool,
    char: char,
    string: String,
    unit: (),
    tuple: (u8, (u16, u16), String),
    test_struct: TestStruct,
    test_enum: TestEnum,
    seq: Vec<u8>,
    option: Option<u8>,
    map: BTreeMap<String, TestStruct2>,
    // BTreeMap so we get predictable iteration order
    enum_variants: Vec<TestEnum2>,
    undefined_vec: Vec<u8>,
    undefined_option: Option<u8>,
}

fn format_rs_file(path: &str) -> Result<(), String> {
    // NB: We'd like to use the rustfmt-nightly as a lib for this, but that requires nightly
    // features, so instead we'll just call the rustfmt tool as a process.

    let rustfmt = toolchain_find::find_installed_component("rustfmt")
        .ok_or_else(|| String::from("This test requires 'rustfmt' on the local toolchain."))?;

    let mut process = std::process::Command::new(&rustfmt)
        .arg(path)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .map_err(|err| format!("Could not spawn process: {}", err))?;

    let out = process.wait_with_output().unwrap();
    let code = out.status.code().ok_or_else(|| String::from("Did not get exit code."))?;
    if code != 0 {
        panic!(
            "rustfmt failed with code {code}.\n===STDOUT===\n{}\n===STDERR===\n{}\n",
            String::from_utf8(out.stdout).unwrap_or_else(|_| String::from("(Not valid UTF-8)")),
            String::from_utf8(out.stderr).unwrap_or_else(|_| String::from("(Not valid UTF-8)")),
        );
    }
    Ok(())
}

#[test]
fn test_testdata() {
    use std::fs::File;

    const ACTUAL_FN_PATH: &'static str = "target/testdata_fn.rs";
    const ACTUAL_TYPES_PATH: &'static str = "target/testdata_types.rs";

    let input_file = File::open("resources/test/testdata_input.ron").unwrap();
    let input: TestData = ron::de::from_reader(input_file).unwrap();

    let (structs, enums) = {
        let mut output_fn_file = File::create(ACTUAL_FN_PATH).unwrap();

        writeln!(output_fn_file, "pub const fn test_data() -> TestData {{").unwrap();
        let mut serializer = Serializer::new(&mut output_fn_file);
        let field_type = input.serialize(&mut serializer).unwrap();
        let structs = std::mem::take(&mut serializer.structs);
        let enums = std::mem::take(&mut serializer.enums);

        assert_struct(&field_type);

        writeln!(output_fn_file, "}}").unwrap();

        (structs, enums)
    };

    let mut output_types_file = File::create(ACTUAL_TYPES_PATH).unwrap();
    structs.write(&mut output_types_file).unwrap();
    enums.write(&mut output_types_file).unwrap();

    format_rs_file(ACTUAL_FN_PATH);
    format_rs_file(ACTUAL_TYPES_PATH);

    use std::fs::read_to_string;
    assert_eq!(
        read_to_string("resources/test/testdata_fn_expected.rs").unwrap(),
        read_to_string(ACTUAL_FN_PATH).unwrap(),
    );
    assert_eq!(
        read_to_string("resources/test/testdata_types_expected.rs").unwrap(),
        read_to_string(ACTUAL_TYPES_PATH).unwrap(),
    );
}
