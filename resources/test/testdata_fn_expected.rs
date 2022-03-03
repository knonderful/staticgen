pub const fn test_data() -> TestData {
    TestData {
        u8: 123,
        u16: 12345,
        u32: 1234567,
        u64: 1234567890,
        i8: -123,
        i16: -12345,
        i32: -1234567,
        i64: -1234567890,
        f32: 12.345,
        f64: -12.234567,
        bool: true,
        char: 'a',
        string: "Test String",
        unit: (),
        tuple: (11, (1234, 5678), "Hello"),
        test_struct: TestStruct {
            string: "Test Struct",
            test_struct_2: TestStruct2 {
                u16: None,
                test_struct_3: TestStruct3(1122334455),
            },
        },
        test_enum: TestEnum::UnitVariant,
        seq: &[9, 8, 7, 6, 5, 4, 3, 2, 1],
        option: Some(12),
        map: Generated1 {
            alpha: TestStruct2 {
                u16: Some(11),
                test_struct_3: TestStruct3(999),
            },
            beta: TestStruct2 {
                u16: Some(22),
                test_struct_3: TestStruct3(888),
            },
            gamma: TestStruct2 {
                u16: Some(33),
                test_struct_3: TestStruct3(777),
            },
        },
        enum_variants: &[
            TestEnum2::Rgba {
                r: 2,
                g: 3,
                b: 4,
                a: 255,
            },
            TestEnum2::Rgba {
                r: 12,
                g: 13,
                b: 14,
                a: 255,
            },
            TestEnum2::Compressed(&[1, 2, 3, 4, 5, 6]),
        ],
        undefined_vec: &[],
        undefined_option: None,
    }
}
