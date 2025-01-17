#![no_std]
#![allow(deprecated)]

use serde_big_array::big_array;
use serde_derive::{Deserialize, Serialize};

big_array! { BigArray; }

#[derive(Serialize)]
struct S {
    #[serde(with = "BigArray")]
    arr: [SerOnly; 64],
}

#[derive(Clone, Copy, Serialize)]
struct SerOnly(u8);

#[derive(Deserialize)]
struct D {
    #[serde(with = "BigArray")]
    arr: [DeOnly; 64],
}

#[derive(Clone, Copy, Deserialize)]
struct DeOnly(u8);

impl PartialEq<DeOnly> for SerOnly {
    fn eq(&self, other: &DeOnly) -> bool {
        self.0 == other.0
    }
}

#[test]
fn test() {
    let s = S {
        arr: [SerOnly(1); 64],
    };
    let j = serde_json::to_string(&s).unwrap();
    let s_back = serde_json::from_str::<D>(&j).unwrap();
    assert!(&s.arr[..] == &s_back.arr[..]);
}
