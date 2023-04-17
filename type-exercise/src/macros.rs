#![allow(unused)]

macro_rules! for_all_types {
    ($cb:ident) => {
        $cb! {
            { Int32, I32, I32Array, I32ArrayBuilder, i32, i32 },
            { Float64, F64, F64Array, F64ArrayBuilder, f64, f64 },
            { String, String, StringArray, StringArrayBuilder, String, &'a str}
        }
    };
}

pub(crate) use for_all_types;

macro_rules! for_all_primitive_types {
    ($cb:ident) => {
        $cb! {
            { Int32, I32, I32Array, I32ArrayBuilder, i32, i32 },
            { Float64, F64, F64Array, F64ArrayBuilder, f64, f64 }
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
