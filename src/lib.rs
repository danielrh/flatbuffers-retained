//! This library allows you to validate Flatbuffer buffers once
//! at initialization time then safely use them, unchecked, later.

use flatbuffers::{Follow, InvalidFlatbuffer, Verifiable};

mod flatbuffer_retained;
mod size_prefixed;
pub use flatbuffer_retained::FlatbufferRetained;
pub use size_prefixed::SizePrefixedFlatbufferRetained;

#[derive(Debug, Clone)]
pub enum Retained<'a, T: Follow<'a>> {
    Unprefixed(FlatbufferRetained<'a, T>),
    SizePrefixed(SizePrefixedFlatbufferRetained<'a, T>),
}
impl<'a, T> Retained<'a, T>
where
    T: Follow<'a> + Verifiable,
{
    /// Make a new FlatbufferRetained class of type T from
    /// an unprefixed byte buffer.
    ///
    /// # Arguments
    ///
    /// * `data` - The data that represents a flatbuffer of type T.
    ///            not prefixed by size.
    ///
    /// # Errors
    ///
    /// May return any InvalidFlatbuffer error from run_verifier
    /// when it parses the data.
    pub fn new_unprefixed(data: Vec<u8>) -> Result<Self, InvalidFlatbuffer> {
        Ok(Retained::Unprefixed(FlatbufferRetained::<T>::new(data)?))
    }
    /// Make a new FlatbufferRetained class of type T from
    /// a size-prefixed byte buffer.
    ///
    /// # Arguments
    ///
    /// * `data` - The data that represents a flatbuffer of type T,
    ///            prefixed by size.
    ///
    /// # Errors
    ///
    /// May return any InvalidFlatbuffer error from run_verifier
    /// when it parses the data.
    pub fn new_size_prefixed(data: Vec<u8>) -> Result<Self, InvalidFlatbuffer> {
        Ok(Retained::SizePrefixed(
            SizePrefixedFlatbufferRetained::<T>::new(data)?,
        ))
    }

    /// Return a valid root of type T from the flatbuffer
    /// buffer stored in self.
    pub fn get(&'a self) -> <T as Follow<'a>>::Inner {
        match self {
            Retained::Unprefixed(ref a) => a.get(),
            Retained::SizePrefixed(ref a) => a.get(),
        }
    }
}
impl<'a, T: Follow<'a>> Retained<'a, T> {
    /// Return an iterator to traverse over the contained Vec.
    pub fn iter(&self) -> std::slice::Iter<u8> {
        match self {
            Retained::Unprefixed(a) => a.iter(),
            Retained::SizePrefixed(a) => a.iter(),
        }
    }

    /// Deconstruct this class and return the Vec that
    /// made up the data within it.
    pub fn take(self) -> Vec<u8> {
        match self {
            Retained::Unprefixed(a) => a.take(),
            Retained::SizePrefixed(a) => a.take(),
        }
    }

    /// Return a reference to the Vec that make up the data within.
    pub fn as_vec(&self) -> &Vec<u8> {
        match self {
            Retained::Unprefixed(a) => a.as_vec(),
            Retained::SizePrefixed(a) => a.as_vec(),
        }
    }
}

impl<'a, T: Follow<'a>> Into<Vec<u8>> for Retained<'a, T> {
    fn into(self) -> Vec<u8> {
        match self {
            Retained::Unprefixed(a) => a.take(),
            Retained::SizePrefixed(a) => a.take(),
        }
    }
}

impl<'a, 'b, T: Follow<'a>> Into<&'b [u8]> for &'b Retained<'a, T> {
    fn into(self) -> &'b [u8] {
        match self {
            &Retained::Unprefixed(ref a) => a.into(),
            &Retained::SizePrefixed(ref a) => a.into(),
        }
    }
}

impl<'a, T: Follow<'a>> AsRef<[u8]> for Retained<'a, T> {
    fn as_ref(&self) -> &[u8] {
        match self {
            &Retained::Unprefixed(ref a) => a.as_ref(),
            &Retained::SizePrefixed(ref a) => a.as_ref(),
        }
    }
}

impl<'a, T: Follow<'a>> std::borrow::Borrow<[u8]> for Retained<'a, T> {
    fn borrow(&self) -> &[u8] {
        match self {
            &Retained::Unprefixed(ref a) => a.borrow(),
            &Retained::SizePrefixed(ref a) => a.borrow(),
        }
    }
}

impl<'a, T: Follow<'a>> std::ops::Deref for Retained<'a, T> {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        match self {
            &Retained::Unprefixed(ref a) => a.deref(),
            &Retained::SizePrefixed(ref a) => a.deref(),
        }
    }
}

impl<'a, T: Follow<'a>> IntoIterator for Retained<'a, T> {
    type Item = u8;
    type IntoIter = <Vec<u8> as IntoIterator>::IntoIter;
    fn into_iter(self) -> <Vec<u8> as IntoIterator>::IntoIter {
        match self {
            Retained::Unprefixed(a) => a.into_iter(),
            Retained::SizePrefixed(a) => a.into_iter(),
        }
    }
}
