pub mod types;

use std::marker::PhantomData;
use crate::error::Error;

pub trait Filter<'f> {
  fn new(&self, filter: &'f str) -> Result<Box<Self>, Error>;
}