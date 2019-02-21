pub mod types;

use crate::error::Error;

trait Filter<'f> {
  type This;
  fn new(filter: &'f str) -> Result<Self::This, Error>;
}