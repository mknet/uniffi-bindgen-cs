/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use paste::paste;

uniffi::setup_scaffolding!();

macro_rules! define_parse_function {
    ($type:ty) => {
        paste! {
            #[uniffi::export]
            fn [<to_string_ $type>](value: $type) -> String {
                value.to_string()
            }

            #[uniffi::export]
            fn [<parse_ $type>](value: String) -> $type {
                value.parse::<$type>().unwrap()
            }
        }
    };
}

// Special implementations for f32 and f64 to match C# formatting
#[uniffi::export]
fn to_string_f32(value: f32) -> String {
    // Use the same formatting as C#'s float.ToString()
    value.to_string()
}

#[uniffi::export]
fn parse_f32(value: String) -> f32 {
    value.parse::<f32>().unwrap()
}

#[uniffi::export]
fn to_string_f64(value: f64) -> String {
    // Use the same formatting as C#'s double.ToString()
    value.to_string()
}

#[uniffi::export]
fn parse_f64(value: String) -> f64 {
    value.parse::<f64>().unwrap()
}

define_parse_function!(i8);
define_parse_function!(i16);
define_parse_function!(i32);
define_parse_function!(i64);
define_parse_function!(u8);
define_parse_function!(u16);
define_parse_function!(u32);
define_parse_function!(u64);
