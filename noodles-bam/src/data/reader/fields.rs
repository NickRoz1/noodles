use std::io::{self, BufRead};

use crate::data::Field;

use super::Reader;

pub struct Fields<R>
where
    R: BufRead,
{
    reader: Reader<R>,
}

impl<R> Fields<R>
where
    R: BufRead,
{
    pub fn new(reader: Reader<R>) -> Self {
        Self { reader }
    }
}

impl<R> Iterator for Fields<R>
where
    R: BufRead,
{
    type Item = io::Result<Field>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.reader.read_field() {
            Ok(Some(field)) => Some(Ok(field)),
            Ok(None) => None,
            Err(e) => Some(Err(e)),
        }
    }
}