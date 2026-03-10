use std::marker::PhantomData;

use quick_xml::NsReader;

use crate::codes_service::EfrCodesError;

#[derive(Clone, Debug)]
pub struct Rows<'a, T> {
    ns_reader: NsReader<&'a [u8]>,
    _row: PhantomData<T>,
}

impl<'a, T: Row<'a>> Rows<'a, T> {
    pub(super) fn new(ns_reader: NsReader<&'a [u8]>) -> Self {
        Self {
            ns_reader,
            _row: PhantomData,
        }
    }
}

impl<'a, T: Row<'a>> Iterator for Rows<'a, T> {
    type Item = Result<T, EfrCodesError>;

    fn next(&mut self) -> Option<Self::Item> {
        T::row(&mut self.ns_reader).transpose()
    }
}

pub trait Row<'a>: Sized {
    fn row(ns_reader: &mut NsReader<&'a [u8]>) -> Result<Option<Self>, EfrCodesError>;
}
