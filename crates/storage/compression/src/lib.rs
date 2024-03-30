#![allow(non_snake_case)]

use alloy_primitives::{Address, Bloom, Bytes, B256, B512, U256};
use bytes::{Buf, BufMut};

pub trait Compressor: Sized {

  // Takes a buffer which can be written to. (Ideally) returns the length written to.
  fn compress<B>(self, buffer: &mut B) -> usize
    where
      B: BufMut;

  // Takes a buffer which can be read from. Returns the object and buffer with its internal cursor
  // advanced.
  // NOTE : It'll panic if len < buffer.len( ).
  fn decompress(buffer: &[u8],
                // Can either be the buffer remaining length, or the length of the compacted type.
                len: usize) -> (Self, &[u8]);

  // Override implementation and use when dealing with fixed-size bytes.
  fn compressFixedSizeBytes<B>(self, buffer: &mut B) -> usize
    where
      B: BufMut
  { self.compress(buffer) }

  // Override implementation and use when dealing with fixed-size bytes.
  fn decompressFixedSizeBytes(buffer: &[u8], len: usize) -> (Self, &[u8]) {
    Self::decompress(buffer, len)
  }
}

macro_rules! uint_types_impl_compressor {
  ($($type_name:tt),+) => {
    $(
      impl Compressor for $type_name {
        #[inline] // Inlining is an optimization technique where the compiler replaces a function
                  // call with the actual body of the function at the call site. 
        fn compress<B>(self, buffer: &mut B) -> usize
          where
            B: BufMut
        {
          let leadingZeroBitCount= self.leading_zeros( ) as usize;
          let bytesWithLeadingZeroBits= leadingZeroBitCount / 8;

          buffer.put_slice(&self.to_be_bytes( )[bytesWithLeadingZeroBits..]);

          core::mem::size_of::<$type_name>( ) - bytesWithLeadingZeroBits
        }

        fn decompress(mut buffer: &[u8], len: usize) -> (Self, &[u8]) {
          if len == 0 { return (0, buffer)}

          const UINT_TYPE_SIZE: usize= core::mem::size_of::<$type_name>( );

          let mut uintAsBytes= [0; UINT_TYPE_SIZE];
          let bytesWithLeadingZeroBits= UINT_TYPE_SIZE - len;
          uintAsBytes[bytesWithLeadingZeroBits..].copy_from_slice(&buffer[..len]);

          buffer.advance(len);

          ($type_name::from_be_bytes(uintAsBytes), buffer)
        }
      }
    )+ // '+' means repeat the contents inside for each match.
  };
}
uint_types_impl_compressor!(u8, u64, u128);

impl Compressor for U256 {
  #[inline]
  fn compress<B>(self, buffer: &mut B) -> usize
    where
      B: BufMut,
  {
    let u256AsBytes = self.to_be_bytes::<32>( );

    let leadingZeroBitCount= self.leading_zeros( ) as usize;
    let bytesWithLeadingZeroBits= leadingZeroBitCount / 8;

    let sizeOccupiedInBuffer = 32 - bytesWithLeadingZeroBits;
    buffer.put_slice(&u256AsBytes[32 - sizeOccupiedInBuffer..]);
    sizeOccupiedInBuffer
  }

  #[inline]
  fn decompress(mut buffer: &[u8], len: usize) -> (Self, &[u8]) {
    if len == 0 {
      return (U256::ZERO, buffer)
    }

    let mut u256AsBytes = [0; 32];
    u256AsBytes[(32 - len)..].copy_from_slice(&buffer[..len]);
    buffer.advance(len);
    (U256::from_be_bytes(u256AsBytes), buffer)
  }
}

impl<T> Compressor for Vec<T>
  where
    T: Compressor
{
  // Returns 0 since we won't include it in the StructFlags.
  #[inline]
  fn compress<B>(self, buffer: &mut B) -> usize
    where
      B: BufMut
  {
    compressUsize(self.len( ), buffer);

    let mut temp: Vec<u8> = Vec::with_capacity(64);
    for element in self {
      temp.clear( );

      let bufferSizeOccupiedByElement= element.compress(&mut temp);
      compressUsize(bufferSizeOccupiedByElement, buffer);

      buffer.put_slice(&temp);
    }

    0
  }

  #[inline]
  fn decompress(buffer: &[u8], _: usize) -> (Self, &[u8]) {
    let (vecLen, mut buffer)= decompressUsize(buffer);

    let mut vec= Vec::with_capacity(vecLen);
    for _ in 0..vecLen {
      let bufferSizeOccupiedByElement;
      (bufferSizeOccupiedByElement, buffer)= decompressUsize(buffer);

      let (element, _) = T::decompress(buffer, bufferSizeOccupiedByElement);
      buffer.advance(bufferSizeOccupiedByElement);

      vec.push(element);
    }

    (vec, buffer)
  }

  #[inline]
  fn compressFixedSizeBytes<B>(self, buffer: &mut B) -> usize
    where
      B: BufMut
  {
    compressUsize(self.len( ), buffer);

    for element in self {
      element.compress(buffer);
    }

    0
  }

  #[inline]
  fn decompressFixedSizeBytes(buffer: &[u8], len: usize) -> (Self, &[u8]) {
    let (vecLen, mut buffer)= decompressUsize(buffer);

    let mut vec= Vec::with_capacity(vecLen);
    for _ in 0..vecLen {
      let element;
      (element, buffer)= T::decompress(buffer, len);

      vec.push(element);
    }

    (vec, buffer)
  }
}

impl<T> Compressor for Option<T>
  where
    T: Compressor
{
  // Returns 0 for None and 1 for Some(_).
  fn compress<B>(self, buffer: &mut B) -> usize
    where
      B: BufMut
  {
    if self.is_none( ) {
      return 0
    }

    let element= self.unwrap( );

    let mut temp: Vec<u8> = Vec::with_capacity(64);
    let bufferSizeOccupiedByElement= element.compress(&mut temp);

    compressUsize(bufferSizeOccupiedByElement, buffer);
    buffer.put_slice(&temp);

    1
  }

  fn decompress(buffer: &[u8], len: usize) -> (Self, &[u8]) {
    if len == 0 {
      return (None, buffer)
    }

    let (bufferSizeOccupiedByElement, mut buffer)= decompressUsize(buffer);

    let (element, _) = T::decompress(&buffer[..bufferSizeOccupiedByElement], bufferSizeOccupiedByElement);
    buffer.advance(bufferSizeOccupiedByElement);

    (Some(element), buffer)
  }

  #[inline]
  fn compressFixedSizeBytes<B>(self, buffer: &mut B) -> usize
    where
      B: BufMut
  {
    match self {
      Some(value) => {
        value.compress(buffer);
        1
      },

      None => 0
    }
  }

  #[inline]
  fn decompressFixedSizeBytes(buffer: &[u8], len: usize) -> (Self, &[u8]) {
    if len == 0 {
      return (None, buffer)
    }

    let (value, buffer) = T::decompress(buffer, len);
    (Some(value), buffer)
  }
}

impl Compressor for bool {
  // bool vars go directly to the StructFlags and are not written to the buffer.
  #[inline]
  fn compress<B>(self, _: &mut B) -> usize
    where
      B: BufMut
  { self as usize }

  // bool expects the real value to come in len, and does not advance the cursor.
  #[inline]
  fn decompress(buffer: &[u8], len: usize) -> (Self, &[u8]) {
    (len != 0, buffer)
  }
}

impl<const N: usize> Compressor for [u8; N] {
  #[inline]
  fn compress<B>(self, buffer: &mut B) -> usize
    where
      B: BufMut
  {
    buffer.put_slice(&self);
    N
  }

  #[inline]
  fn decompress(mut buffer: &[u8], len: usize) -> (Self, &[u8]) {
    if len == 0 {
      return ([0; N], buffer)
    }

    let decompressedValue = buffer[..N].try_into( ).unwrap( );
    buffer.advance(N);
    (decompressedValue, buffer)
  }
}

impl Compressor for Bytes {
  #[inline]
  fn compress<B>(self, buffer: &mut B) -> usize
    where
      B: BufMut
  {
    let bytesLen= self.len( );
    buffer.put(self.0);
    bytesLen
  }

  #[inline]
  fn decompress(mut buffer: &[u8], len: usize) -> (Self, &[u8]) {
    (buffer.copy_to_bytes(len).into( ), buffer)
  }
}

macro_rules! fixed_size_bytes_types_impl_compressor {
  ($($type_name:tt),+) => {
    $(
      impl Compressor for $type_name {
        #[inline]
        fn compress<B>(self, buffer: &mut B) -> usize
          where
            B: BufMut
        { self.0.compress(buffer) }

        fn decompress(buffer: &[u8], len: usize) -> (Self, &[u8]) {
          const BYTE_SIZE: usize= core::mem::size_of::<$type_name>( );
          let (value, buffer) = <[u8; BYTE_SIZE]>::decompress(buffer, len);
          (Self::from(value), buffer)
        }
      }
    )+ // '+' means repeat the contents inside for each match.
  };
}
fixed_size_bytes_types_impl_compressor!(Address, B256, B512, Bloom);

fn compressUsize<B>(mut n: usize, buffer: &mut B)
  where
    B: BufMut
{
  /*
    Let n = 300 (100101100 in binary). Since n is of type usize, before compression it takes 64
    bits (8 bytes) to represent 300.

    After compression it'll take only 2 bytes to represent 300.

    Compression process :

    (1) Write the most significant 7 bits of 300 (1001011) with 1 in the end, resulting in 0b10010111.
        The 1 in the end represents that there are remaining bits in the next byte.
    (2) Right shift 300 by 7 bits, which results in 4.
    (3) 4 is less than 0x80, so it's encoded as a single byte (0b00000100).

    The encoded bytes would be: 0b10010111 0b00000100.
  */

  while n >= 0x80 {
    buffer.put_u8((n as u8) | 0x80);
    n >>= 7;
  }

  buffer.put_u8(n as u8);
}

fn decompressUsize(buffer: &[u8]) -> (usize, &[u8]) {
  let mut value= 0;

  for i in 0..33 {
    let byte= buffer[i];
    value |= usize::from(byte & 0x7F) << (i * 7);
    if byte < 0x80 {
      return (value, &buffer[i + 1..])
    }
  }
  usizeDecompressorPanic( );
}

#[inline(never)]
#[cold] // Indicates that the function is rarely called. The function will be optimized for code
        // size rather than speed.
const fn usizeDecompressorPanic( ) -> ! {
  panic!("could not decode usize");
}
