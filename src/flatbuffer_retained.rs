//! This module focuses on validation and storage of flatbuffers
//! without size-prefixes. It allows validation of Flatbuffer buffers
//! once at initialization time then safely use them, unchecked, later.

use flatbuffers::{
    root_unchecked, Follow, ForwardsUOffset, InvalidFlatbuffer, Verifiable, Verifier,
    VerifierOptions,
};
use std::marker::PhantomData;

/// This struct holds data backing an unprefixed flatbuffer.
/// It is not possible to create this struct without a valid
/// flatbuffer of type T.
#[derive(Clone, Debug)]
pub struct FlatbufferRetained<'a, T>
where
    T: Follow<'a>,
{
    /// Data represting a validated T.
    data: Vec<u8>,
    /// Phantom data to place-hold which T data was validated for.
    phantom: PhantomData<&'a T>,
}

impl<'a, T> FlatbufferRetained<'a, T>
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
    pub fn new(data: Vec<u8>) -> Result<Self, InvalidFlatbuffer> {
        let opts = VerifierOptions::default();
        let mut v = Verifier::new(&opts, &data);
        <ForwardsUOffset<T>>::run_verifier(&mut v, 0)?;
        Ok(FlatbufferRetained {
            data,
            phantom: PhantomData::default(),
        })
    }

    /// Return a valid root of type T from the flatbuffer
    /// buffer stored in self.
    pub fn get(&'a self) -> <T as Follow<'a>>::Inner {
        unsafe { root_unchecked::<T>(&self.data) }
    }
}

impl<'a, T: Follow<'a>> FlatbufferRetained<'a, T> {
    /// Return an iterator to traverse over the contained Vec.
    pub fn iter(&self) -> std::slice::Iter<u8> {
        self.data.iter()
    }

    /// Deconstruct this class and return the Vec that
    /// made up the data within it.
    pub fn take(self) -> Vec<u8> {
        self.data
    }

    /// Return a reference to the Vec that make up the data within.
    pub fn as_vec(&self) -> &Vec<u8> {
        &self.data
    }
}

impl<'a, T: Follow<'a>> Into<Vec<u8>> for FlatbufferRetained<'a, T> {
    fn into(self) -> Vec<u8> {
        self.take()
    }
}

impl<'a, 'b, T: Follow<'a>> Into<&'b Vec<u8>> for &'b FlatbufferRetained<'a, T> {
    fn into(self) -> &'b Vec<u8> {
        &self.data
    }
}

impl<'a, 'b, T: Follow<'a>> Into<&'b [u8]> for &'b FlatbufferRetained<'a, T> {
    fn into(self) -> &'b [u8] {
        &self.data[..]
    }
}

impl<'a, T: Follow<'a>> AsRef<[u8]> for FlatbufferRetained<'a, T> {
    fn as_ref(&self) -> &[u8] {
        self.data.as_ref()
    }
}

impl<'a, T: Follow<'a>> AsRef<Vec<u8>> for FlatbufferRetained<'a, T> {
    fn as_ref(&self) -> &Vec<u8> {
        &self.data
    }
}

impl<'a, T: Follow<'a>> std::borrow::Borrow<[u8]> for FlatbufferRetained<'a, T> {
    fn borrow(&self) -> &[u8] {
        self.data.borrow()
    }
}

impl<'a, T: Follow<'a>> std::ops::Deref for FlatbufferRetained<'a, T> {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        self.data.deref()
    }
}

impl<'a, T: Follow<'a>> IntoIterator for FlatbufferRetained<'a, T> {
    type Item = u8;
    type IntoIter = <Vec<u8> as IntoIterator>::IntoIter;
    fn into_iter(self) -> <Vec<u8> as IntoIterator>::IntoIter {
        self.data.into_iter()
    }
}
