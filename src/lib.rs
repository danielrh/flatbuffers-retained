use flatbuffers::{Follow, InvalidFlatbuffer, root_unchecked, size_prefixed_root_unchecked, Verifiable, Verifier, ForwardsUOffset, VerifierOptions, SkipSizePrefix};
use std::marker::PhantomData;

pub struct FlatbufferRetained<'a, T> where T: Follow<'a>{
    data: Vec<u8>,
    phantom:PhantomData<&'a T>,
}

impl<'a, T> FlatbufferRetained<'a, T> where T : Follow<'a> + Verifiable {
   pub fn new(data:Vec<u8>) -> Result<Self, InvalidFlatbuffer> {
       let opts = VerifierOptions::default();
       let mut v = Verifier::new(&opts, &data);
       <ForwardsUOffset<T>>::run_verifier(&mut v, 0)?;
       Ok(FlatbufferRetained{
          data,
          phantom:PhantomData::default(),
       })
   }
   pub fn get(&'a self) -> <T as Follow<'a>>::Inner {
        unsafe {
            root_unchecked::<T>(&self.data)
        }
    }
}

pub struct SizePrefixedFlatbufferRetained<'a, T> where T: Follow<'a>{
    data: Vec<u8>,
    phantom:PhantomData<&'a T>,
}
impl<'a, T> SizePrefixedFlatbufferRetained<'a, T> where T : Follow<'a> + Verifiable {
   pub fn new(data:Vec<u8>) -> Result<Self, InvalidFlatbuffer> {
       let opts = VerifierOptions::default();
       let mut v = Verifier::new(&opts, &data);
       <SkipSizePrefix<ForwardsUOffset<T>>>::run_verifier(&mut v, 0)?;
       Ok(SizePrefixedFlatbufferRetained{
          data,
          phantom:PhantomData::default(),
       })
   }
   pub fn get(&'a self) -> <T as Follow<'a>>::Inner {
        unsafe {
            size_prefixed_root_unchecked::<T>(&self.data)
        }
    }
}

