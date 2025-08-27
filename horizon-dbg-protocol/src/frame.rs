use zerocopy::{FromBytes, Immutable, IntoBytes};

#[derive(Clone, Copy, FromBytes, IntoBytes, Immutable)]
#[repr(C)]
pub struct Header {
  pub length: u16,
  pub key: u16,
}
