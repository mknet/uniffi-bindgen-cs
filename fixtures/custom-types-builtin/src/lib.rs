use std::collections::HashMap;

pub struct MyString(pub String);
impl UniffiCustomTypeConverter for MyString {
    type Builtin = String;
    fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
        Ok(MyString(val))
    }
    fn from_custom(obj: Self) -> Self::Builtin {
        obj.0
    }
}

pub struct Array(pub Vec<String>);
impl UniffiCustomTypeConverter for Array {
    type Builtin = Vec<String>;
    fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
        Ok(Array(val))
    }
    fn from_custom(obj: Self) -> Self::Builtin {
        obj.0
    }
}

pub struct Table(pub HashMap<String, String>);
impl UniffiCustomTypeConverter for Table {
    type Builtin = HashMap<String, String>;
    fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
        Ok(Table(val))
    }
    fn from_custom(obj: Self) -> Self::Builtin {
        obj.0
    }
}

pub struct Boolean(pub bool);
impl UniffiCustomTypeConverter for Boolean {
    type Builtin = bool;
    fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
        Ok(Boolean(val))
    }
    fn from_custom(obj: Self) -> Self::Builtin {
        obj.0
    }
}

pub struct Int8(pub i8);
impl UniffiCustomTypeConverter for Int8 {
    type Builtin = i8;
    fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
        Ok(Int8(val))
    }
    fn from_custom(obj: Self) -> Self::Builtin {
        obj.0
    }
}

pub struct Int16(pub i16);
impl UniffiCustomTypeConverter for Int16 {
    type Builtin = i16;
    fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
        Ok(Int16(val))
    }
    fn from_custom(obj: Self) -> Self::Builtin {
        obj.0
    }
}

pub struct Int32(pub i32);
impl UniffiCustomTypeConverter for Int32 {
    type Builtin = i32;
    fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
        Ok(Int32(val))
    }
    fn from_custom(obj: Self) -> Self::Builtin {
        obj.0
    }
}

pub struct Int64(pub i64);
impl UniffiCustomTypeConverter for Int64 {
    type Builtin = i64;
    fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
        Ok(Int64(val))
    }
    fn from_custom(obj: Self) -> Self::Builtin {
        obj.0
    }
}

pub struct UInt8(pub u8);
impl UniffiCustomTypeConverter for UInt8 {
    type Builtin = u8;
    fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
        Ok(UInt8(val))
    }
    fn from_custom(obj: Self) -> Self::Builtin {
        obj.0
    }
}

pub struct UInt16(pub u16);
impl UniffiCustomTypeConverter for UInt16 {
    type Builtin = u16;
    fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
        Ok(UInt16(val))
    }
    fn from_custom(obj: Self) -> Self::Builtin {
        obj.0
    }
}

pub struct UInt32(pub u32);
impl UniffiCustomTypeConverter for UInt32 {
    type Builtin = u32;
    fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
        Ok(UInt32(val))
    }
    fn from_custom(obj: Self) -> Self::Builtin {
        obj.0
    }
}

pub struct UInt64(pub u64);
impl UniffiCustomTypeConverter for UInt64 {
    type Builtin = u64;
    fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
        Ok(UInt64(val))
    }
    fn from_custom(obj: Self) -> Self::Builtin {
        obj.0
    }
}

pub struct Float(pub f32);
impl UniffiCustomTypeConverter for Float {
    type Builtin = f32;
    fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
        Ok(Float(val))
    }
    fn from_custom(obj: Self) -> Self::Builtin {
        obj.0
    }
}

pub struct Double(pub f64);
impl UniffiCustomTypeConverter for Double {
    type Builtin = f64;
    fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
        Ok(Double(val))
    }
    fn from_custom(obj: Self) -> Self::Builtin {
        obj.0
    }
}

pub struct CustomTypesBuiltin {
    string: MyString,
    array: Array,
    table: Table,
    boolean: Boolean,
    int8: Int8,
    int16: Int16,
    int32: Int32,
    int64: Int64,
    uint8: UInt8,
    uint16: UInt16,
    uint32: UInt32,
    uint64: UInt64,
    float: Float,
    double: Double,
}

pub fn get_custom_types_builtin() -> CustomTypesBuiltin {
    CustomTypesBuiltin {
        string: MyString("Hello, world!".to_string()),
        array: Array(vec!["Hello, world!".to_string()]),
        table: Table(HashMap::from([("hello".to_string(), "world".to_string())])),
        boolean: Boolean(true),
        int8: Int8(i8::MAX),
        int16: Int16(i16::MAX),
        int32: Int32(i32::MAX),
        int64: Int64(i64::MAX),
        uint8: UInt8(u8::MAX),
        uint16: UInt16(u16::MAX),
        uint32: UInt32(u32::MAX),
        uint64: UInt64(u64::MAX),
        float: Float(f32::MAX),
        double: Double(f64::MAX),
    }
}

pub fn return_custom_types_builtin(custom_types: CustomTypesBuiltin) -> CustomTypesBuiltin {
    custom_types
}

uniffi::include_scaffolding!("custom_types_builtin");
