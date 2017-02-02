use core::marker::PhantomData;
use core::ops::{Deref, DerefMut};

/// Statically allocated "Vector"
pub struct Vec<T, B>
    where B: AsMut<[T]> + AsRef<[T]>,
          T: Copy
{
    buffer: B,
    len: usize,
    _marker: PhantomData<[T]>,
}

impl<T, B> Vec<T, B>
    where B: AsMut<[T]> + AsRef<[T]>,
          T: Copy
{
    pub const fn new(buffer: B) -> Self {
        Vec {
            buffer: buffer,
            len: 0,
            _marker: PhantomData,
        }
    }

    pub fn clear(&mut self) {
        self.len = 0;
    }

    pub fn push(&mut self, elem: T) -> Result<(), ()> {
        let buffer = self.buffer.as_mut();

        if self.len < buffer.len() {
            buffer[self.len] = elem;
            self.len += 1;
            Ok(())
        } else {
            Err(())
        }
    }
}

impl<T, B> Deref for Vec<T, B>
    where B: AsMut<[T]> + AsRef<[T]>,
          T: Copy
{
    type Target = [T];

    fn deref(&self) -> &[T] {
        &self.buffer.as_ref()[..self.len]
    }
}

impl<T, B> DerefMut for Vec<T, B>
    where B: AsMut<[T]> + AsRef<[T]>,
          T: Copy
{
    fn deref_mut(&mut self) -> &mut [T] {
        &mut self.buffer.as_mut()[..self.len]
    }
}
