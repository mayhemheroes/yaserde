#![no_main]
use libfuzzer_sys::fuzz_target;

use yaserde_derive::YaDeserialize;
use yaserde_derive::YaSerialize;

#[derive(Debug, YaSerialize, YaDeserialize, PartialEq)]
enum PlainEnum {
  A,
  B,
  C,
  D,
}

impl Default for PlainEnum {
  fn default() -> Self {
    Self::A
  }
}

#[derive(Debug, YaSerialize, YaDeserialize, PartialEq)]
enum Enum {
  A(u8),
  B,
  C(Vec<PlainEnum>),
  D(i64),
}

impl Default for Enum {
  fn default() -> Self {
    Self::A(0)
  }
}

#[derive(Debug, YaSerialize, YaDeserialize, PartialEq)]
enum FloatEnum {
  A(Enum),
  E(Option<f32>),
}

impl Default for FloatEnum {
  fn default() -> Self {
    Self::E(Some(0.0))
  }
}

#[derive(Debug, YaSerialize, YaDeserialize, PartialEq)]
struct Struct {
  a: u8,
  b: u64,
  c: PlainEnum,
  d: String,
}

impl Default for Struct {
  fn default() -> Self {
    Self {
      a: 2,
      b: u64::MAX,
      c: PlainEnum::default(),
      d: String::from("fuzz"),
    }
  }
}

#[derive(Debug, YaSerialize, YaDeserialize, PartialEq)]
struct FloatStruct {
  a: Struct,
  b: f64,
}

impl Default for FloatStruct {
  fn default() -> Self {
    Self {
      a: Default::default(),
      b: 3.4,
    }
  }
}

macro_rules! from_bytes {
  ($ty:ty, $data:ident) => {{
    #[cfg(feature = "debug")]
    println!("deserializing {}", stringify!($ty));

    let _: Result<$ty, _> = ::yaserde::de::from_str($data);
  }};
}

fuzz_target!(|data: &str| {
  from_bytes!(PlainEnum, data);
  from_bytes!(Enum, data);
  from_bytes!(FloatEnum, data);
  from_bytes!(Struct, data);
  from_bytes!(FloatStruct, data);
});
