#![no_std]

use postcard_rpc::Key;

extern crate alloc;

pub mod breakpoints;
pub mod frame;
pub mod result;
pub mod subscription;

pub use postcard;
pub use postcard_rpc;
pub use postcard_schema;
pub use serde;
pub use zerocopy;

pub enum Error {
  Decoding(postcard::Error),
  Failed, //todo
  NoHandler,
}

pub struct Keygen {
  pub key: Key,
  pub endpoint: &'static str,
}

#[doc(hidden)]
#[macro_export]
macro_rules! keygen {
  (const $name: ident: $type: ty = $endpoint: literal) => {
    pub const $name: $crate::Keygen = {
      $crate::Keygen {
        key: $crate::postcard_rpc::Key::for_path::<$type>($endpoint),
        endpoint: $endpoint,
      }
    };
  };
}

#[doc(hidden)]
// https://www.reddit.com/r/rust/comments/qw18oa/comment/hl05kuj/ thanks alice <3
pub const fn sort_arr<const N: usize, H>(mut arr: [(Key, H); N]) -> [(Key, H); N] {
  loop {
    let mut swapped = false;
    let mut i = 1;
    while i < arr.len() {
      if Key::const_cmp(&arr[i - 1].0, &arr[i].0) {
        arr.swap(i - 1, i);
        swapped = true;
      }
      i += 1;
    }
    if !swapped {
      break;
    }
  }
  arr
}

#[doc(hidden)]
#[macro_export]
macro_rules! gen_func {
  ($handler: ident, $type: ty, $handler_func: ident) => {{
    let generated_function: fn(
      &[u8],
      &mut dyn $handler,
    ) -> ::core::result::Result<(), $crate::Error> = {
      |bytes, handler| {
        let (value, _bytes_read): ($type, usize) =
          $crate::postcard::from_bytes(bytes).map_err($crate::Error::Decoding)?;

        $handler::$handler_func(handler, value).map_err(|_| $crate::Error::Failed)
      }
    };

    generated_function
  }};
}

#[macro_export]
macro_rules! gen_handler {
  (
    pub enum $enum: ident;
    $handler_func_name: ident(key: u16, bytes: &[u8], handler: &mut dyn $handler: ident) {
      $($func_name: ident($type: ident) = $key: expr),*,
    }
  ) => {
    pub trait $handler {
      $(
        fn $func_name(&mut self, value: $type) -> Result<(), ()>; // todo: error type
      )*
    }


    pub fn $handler_func_name(key: u16, bytes: &[u8], handler: &mut impl $handler) -> Result<(), $crate::Error> {
      type Handler = fn(&[u8], &mut dyn $handler) -> Result<(), $crate::Error>;
      const fn one_wrap<T>() -> usize { 1 }
      static SORTED_ARRAY: [(::postcard_rpc::Key, Handler); 0 $(+ one_wrap::<$type>())*] = sort_arr([
        $((
          ($key).key,
          gen_func!($handler, $type, $func_name)
        )),*
      ]);

      let entry = SORTED_ARRAY
        .binary_search_by_key(&key, |value| u16::from_le_bytes($crate::postcard_rpc::Key2::from_key8(value.0).to_bytes()))
        .map_err(|_| $crate::Error::NoHandler)?;
      SORTED_ARRAY[entry].1(bytes, handler)?;

      Ok(())
    }

    pub enum $enum {
      $(
        $type($type)
      ),*
    }
  };
}
