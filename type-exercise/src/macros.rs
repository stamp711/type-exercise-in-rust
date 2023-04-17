#![allow(unused)]

macro_rules! for_all_types {
    ($cb:ident) => {
        $cb! {
            { Int32, I32, I32Array, I32ArrayBuilder, i32, i32 },
            { Int64, I64, I64Array, I64ArrayBuilder, i64, i64 },
            { Float64, F64, F64Array, F64ArrayBuilder, f64, f64 },
            { Bool, Bool, BoolArray, BoolArrayBuilder, bool, bool },
            { String, String, StringArray, StringArrayBuilder, String, &'a str}
        }
    };
}

pub(crate) use for_all_types;

macro_rules! for_all_primitive_types {
    ($cb:ident) => {
        $cb! {
            { Int32, I32, I32Array, I32ArrayBuilder, i32, i32 },
            { Int64, I64, I64Array, I64ArrayBuilder, i64, i64 },
            { Float64, F64, F64Array, F64ArrayBuilder, f64, f64 },
            { Bool, Bool, BoolArray, BoolArrayBuilder, bool, bool }
        }
    };
}

pub(crate) use for_all_primitive_types;

macro_rules! for_all_composite_types {
    ($cb:ident) => {
        $cb! {
            { String, String, StringArray, StringArrayBuilder, String, &'a str}
        }
    };
}

pub(crate) use for_all_composite_types;
