// todo: after protocol serializer/deserializer
#![allow(unused)]
#![no_std]

use core::{hint::unreachable_unchecked, panic::PanicInfo};

#[cfg_attr(not(test), panic_handler)]
fn _ph(_: &PanicInfo) -> ! {
  unsafe { unreachable_unchecked() }
}

struct Parser {
  
}

impl Parser {
  pub fn new() {

  }
}
