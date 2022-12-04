//! This library allows you to validate Flatbuffer buffers once
//! at initialization time then safely use them, unchecked, later.

use flatbuffers::{
    root_unchecked, size_prefixed_root_unchecked, Follow, ForwardsUOffset, InvalidFlatbuffer,
    SkipSizePrefix, Verifiable, Verifier, VerifierOptions,
};
use std::marker::PhantomData;

/// This struct holds data backing an unprefixed flatbuffer.
/// It is not possible to create this struct without a valid
/// flatbuffer of type T.
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

    /// Deconstruct this class and return the Vec that
    /// made up the data within it.
    pub fn take(self) -> Vec<u8> {
        self.data
    }
}

/// This struct holds data backing an size-prefixed flatbuffer.
/// It is not possible to create this struct without a valid
/// flatbuffer of type T.
pub struct SizePrefixedFlatbufferRetained<'a, T: Follow<'a>> {
    data: Vec<u8>,
    phantom: PhantomData<&'a T>,
}

impl<'a, T> SizePrefixedFlatbufferRetained<'a, T>
where
    T: Follow<'a> + Verifiable,
{
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
    pub fn new(data: Vec<u8>) -> Result<Self, InvalidFlatbuffer> {
        let opts = VerifierOptions::default();
        let mut v = Verifier::new(&opts, &data);
        <SkipSizePrefix<ForwardsUOffset<T>>>::run_verifier(&mut v, 0)?;
        Ok(SizePrefixedFlatbufferRetained {
            data,
            phantom: PhantomData::default(),
        })
    }

    /// Return a valid root of type T from the flatbuffer
    /// buffer stored in self.
    pub fn get(&'a self) -> <T as Follow<'a>>::Inner {
        unsafe { size_prefixed_root_unchecked::<T>(&self.data) }
    }

    /// Deconstruct this class and return the Vec that
    /// made up the data within it.
    pub fn take(self) -> Vec<u8> {
        self.data
    }
}

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

    /// Deconstruct this class and return the Vec that
    /// made up the data within it.
    pub fn take(self) -> Vec<u8> {
        match self {
            Retained::Unprefixed(a) => a.take(),
            Retained::SizePrefixed(a) => a.take(),
        }
    }
}
