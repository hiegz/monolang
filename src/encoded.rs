use std::io::Read;
use std::marker::PhantomData;

pub struct Encoded<T, Encoding> {
    inner: T,
    _marker: PhantomData<Encoding>,
}

impl<T, Encoding> Encoded<T, Encoding> {
    pub fn new(inner: T) -> Self {
        Self {
            inner,
            _marker: PhantomData,
        }
    }

    pub fn into_inner(self) -> T {
        self.inner
    }
}

impl<T: Read, Encoding> Read for Encoded<T, Encoding> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.inner.read(buf)
    }
}
