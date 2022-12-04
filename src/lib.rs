use flatbuffers::{Follow, InvalidFlatbuffer, root, root_unchecked, Verifiable};
use std::marker::PhantomData;
pub struct FlatbufferRetained<T> where for<'a> T: Follow<'a>{
    data: Vec<u8>,
    phantom:PhantomData<T>,
}
impl<T> FlatbufferRetained<T> where for<'a> T : Follow<'a> + Verifiable {
   pub fn new(data:Vec<u8>) -> Result<Self, InvalidFlatbuffer> {
       root::<T>(&data)?;
       Ok(FlatbufferRetained{
          data,
          phantom:PhantomData::default(),
       })
   }
   pub fn get<'a>(&'a self) -> <T as Follow<'a>>::Inner {
        unsafe {
            root_unchecked::<T>(&self.data)
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
