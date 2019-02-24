pub mod types;

use std::marker::PhantomData;
use crate::error::Error;

pub enum Object<'o, P, F: Filter<'o>> {
  Type,
  Instance(F),
  Phantom(PhantomData<&'o P>)
}

pub trait Filter<'f> {
  fn new(filter: &'f str) -> Result<Box<Self>, Error>;
}