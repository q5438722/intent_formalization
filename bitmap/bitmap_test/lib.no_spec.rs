// Copyright(c) The Maintainers of Nanvix.
// Licensed under the MIT License.

//! # Bitmap Allocator - Implementation
//!
//! This file contains the implementation code for bitmap allocator.
//! Specification functions are in `lib.spec.rs` and proofs are in `lib.proof.rs`.


use crate::libs::raw_array::RawArray;
use crate::libs::raw_array::{
    axiom_u8_zero_is_0,
    is_zero,
};
use crate::libs::error::{
    Error,
    ErrorCode,
};
use vstd::prelude::*;

// Include specifications.
include!("lib.spec.rs");

// Include proofs.
include!("lib.proof.rs");

// Include verified tests.
include!("lib.test.rs");

//==================================================================================================
// Structures
//==================================================================================================

verus! {

///
/// # Description
///
/// A bitmap.
///
#[verifier::ext_equal]
pub struct Bitmap {
    /// Capacity of the bitmap (in bits).
    number_of_bits: usize,
    /// Number of bits set in the bitmap.
    usage: usize,
    /// Underlying bits.
    bits: RawArray<u8>,
    /// Hint: first bit index that might be free. Avoids O(n) rescans.
    next_free: usize,
}

//==================================================================================================
// Implementations
//==================================================================================================

impl Bitmap {
    ///
    /// # Description
    ///
    /// Creates a new bitmap with a given length. The bitmap is initialized with all bits set to zero.
    ///
    /// # Parameters
    ///
    /// - `number_of_bits`: Length of the bitmap in bits.
    ///
    /// # Returns
    ///
    /// Upon success, a new bitmap is returned. Upon failure, an error is returned instead.
    ///
    pub fn new(number_of_bits: usize) -> (result: Result<Self, Error>)
        
    {
        // Check if the length is invalid.
        if number_of_bits == 0 || number_of_bits >= u32::MAX as usize {
            let reason: &str = "invalid length";
            return Err(Error::new(ErrorCode::InvalidArgument, reason));
        }

        // Check if the length is not a multiple of the number of the bitmap word.
        if !number_of_bits.is_multiple_of(u8::BITS as usize) {
            let reason: &str = "length must be a multiple of 8";
            return Err(Error::new(ErrorCode::InvalidArgument, reason));
        }

        // Allocate the bitmap.
        // Note: RawArray::new() guarantees zero-initialization of the backing storage.
        let len: usize = number_of_bits / u8::BITS as usize;
        
        let array: RawArray<u8> = RawArray::new(len)?;

        let result = Self {
            number_of_bits,
            bits: array,
            usage: 0,
            next_free: 0,
        };

        

        Ok(result)
    }

    ///
    /// # Description
    ///
    /// Creates a new bitmap from a raw array. The bitmap is initialized with
    /// all bits set to zero.
    ///
    /// # Parameters
    ///
    /// - `array`: Raw array to create the bitmap from.
    ///
    /// # Returns
    ///
    /// Upon success, a new bitmap is returned. Upon failure, an error is returned instead.
    ///
    /// # Errors
    ///
    /// - `InvalidArgument` if the array length multiplied by 8 overflows `usize`.
    ///
    pub fn from_raw_array(array: RawArray<u8>) -> (result: Result<Self, Error>)
        
    {
        // TODO: remove this runtime check once all callers are verified.
        let number_of_bits: usize = match array.len().checked_mul(u8::BITS as usize) {
            Some(n) => n,
            None => {
                let reason: &str = "bitmap size overflow: array too large";
                return Err(Error::new(ErrorCode::InvalidArgument, reason));
            },
        };

        let result = Self {
            number_of_bits,
            bits: array,
            usage: 0,
            next_free: 0,
        };
        
        Ok(result)
    }

    ///
    /// # Description
    ///
    /// Returns the number of bits in the bitmap.
    ///
    /// # Returns
    ///
    /// The number of bits in the bitmap.
    ///
    pub fn number_of_bits(&self) -> (result: usize)
        
    {
        self.number_of_bits
    }

    ///
    /// # Description
    ///
    /// Allocates a bit in the bitmap.
    ///
    /// # Returns
    ///
    /// Upon success, the index of the allocated bit is returned. Upon failure, an error is returned
    /// instead.
    ///
    pub fn alloc(&mut self) -> (result: Result<usize, Error>)
        
    {
        
        self.alloc_range(1)
    }

    ///
    /// # Description
    ///
    /// Allocates a range of bits in the bitmap.
    ///
    /// # Parameters
    ///
    /// - `size`: Size of the range to allocate.
    ///
    /// # Returns
    ///
    /// Upon success, the index of the allocated range is returned. Upon failure, an error is returned
    /// instead.
    ///
    pub fn alloc_range(&mut self, size: usize) -> (result: Result<usize, Error>)
        
    {
        let ghost old_self = *self;

        // TODO: remove this runtime check once all callers are verified.
        // Check if the size is valid.
        if size == 0 || size > self.number_of_bits {
            
            let reason: &str = "invalid size";
            return Err(Error::new(ErrorCode::InvalidArgument, reason));
        }

        // Check if allocation exceeds the bitmap capacity.
        if self.usage > self.number_of_bits - size {
            
            let reason: &str = "allocation exceeds bitmap capacity";
            return Err(Error::new(ErrorCode::OutOfMemory, reason));
        }

        // Note: debug_assert_eq! is not supported by Verus, so we guard it
        // with cfg. The invariant self.inv() already proves this property.

        let initial_start: usize = self.next_free;
        let mut start: usize = initial_start;
        let mut wrapped: bool = false;
        let mut done: bool = false;

        // Traverse the bitmap, wrapping around once if needed.
        while !done
            
            
        {
            // Stop condition: exceeded the last valid starting position.
            if start > self.number_of_bits - size {
                // If we haven't wrapped yet and started past 0, retry from beginning.
                if !wrapped && initial_start > 0 {
                    
                    start = 0;
                    wrapped = true;
                } else {
                    
                    done = true;
                }
            }

            // After wrap-around, stop if we've reached the initial position.
            if !done && wrapped && start >= initial_start {
                
                done = true;
            }

            if !done {
                // Check for fast-skip path.
                let is_aligned: bool = start.is_multiple_of(u8::BITS as usize);
                if is_aligned {
                    let word: usize = start / u8::BITS as usize;
                    // Fast skip: if the starting word is full, skip to the next word.
                    if self.bits[word] == u8::MAX {
                        
                        start += u8::BITS as usize;
                        continue;
                    }
                }

                // Check if all bits in the range are free.
                let ghost start_before_inner: usize = start;
                let mut offset: usize = 0;
                let mut free: bool = true;

                // Ghost: snapshot the "checked before" region for the inner loop.
                let ghost checked_before: int = start as int;

                while offset < size
                    
                    
                    
                    
                {
                    let idx: usize = start + offset;
                    let (w, b): (usize, usize) = self.index_unchecked(idx);
                    if (self.bits[w] & (1 << b)) != 0 {
                        free = false;
                        start += offset + 1;
                        
                        break;
                    }
                    offset += 1;
                }

                if free {
                    // Found a free range at [start, start + size).
                    
                    // Allocate the range.
                    let ghost pre_alloc_self = *self;
                    let mut alloc_offset: usize = 0;

                    

                    // Verus note: `for offset in 0..size` is not supported;
                    // `self.bits[w] |= 1 << b` is not supported for mutable index.
                    while alloc_offset < size
                        
                        
                    {
                        let idx: usize = start + alloc_offset;
                        let (w, b): (usize, usize) = self.index_unchecked(idx);
                        let ghost loop_old_self = *self;

                        self.bits.set(w, self.bits[w] | (1 << b));

                        

                        alloc_offset += 1;
                    }
                    // Verus note: compound assignment on struct fields not supported.
                    self.usage = self.usage + size;
                    self.next_free = start + size;

                    

                    return Ok(start);
                }
                // !free: start was advanced past the blocked position.
                
            }
        }

        // No free range found anywhere in the bitmap.
        
        let reason: &str = "bitmap is full";
        Err(Error::new(ErrorCode::OutOfMemory, reason))
    }

    ///
    /// # Description
    ///
    /// Sets a bit at a given index in the bitmap.
    ///
    /// # Parameters
    ///
    /// - `index`: Index of the bit to set.
    ///
    /// # Returns
    ///
    /// Upon success, `Ok(())` is returned. Upon failure, an error is returned instead.
    ///
    pub fn set(&mut self, index: usize) -> (result: Result<(), Error>)
        
    {
        // Check if the bit is already set.
        if self.test(index)? {
            let reason: &str = "bit is already set";
            return Err(Error::new(ErrorCode::ResourceBusy, reason));
        }

        let (word, bit): (usize, usize) = self.index(index)?;
        let ghost old_self = *self;

        // At this point, we know:
        // - old_self.inv() holds
        // - !old_self.is_bit_set(index as int) (the bit is not set)
        

        self.bits.set(word, self.bits[word] | (1 << bit));

        

        self.usage = self.usage + 1;

        

        Ok(())
    }

    ///
    /// # Description
    ///
    /// Clears a bit at a given index in the bitmap.
    ///
    /// # Parameters
    ///
    /// - `index`: Index of the bit to clear.
    ///
    /// # Returns
    ///
    /// Upon success, `Ok(())` is returned. Upon failure, an error is returned instead.
    ///
    pub fn clear(&mut self, index: usize) -> (result: Result<(), Error>)
        
    {
        // TODO: remove this runtime check once all callers are verified.
        // Check if the bit is already cleared.
        if !self.test(index)? {
            let reason: &str = "bit is already cleared";
            return Err(Error::new(ErrorCode::BadAddress, reason));
        }

        let (word, bit): (usize, usize) = self.index(index)?;
        let ghost old_self = *self;

        // At this point, we know:
        // - old_self.inv() holds
        // - old_self.is_bit_set(index as int) (the bit is set)
        

        self.bits.set(word, self.bits[word] & !(1 << bit));

        

        self.usage = self.usage - 1;
        if index < self.next_free {
            self.next_free = index;
        }

        

        Ok(())
    }

    ///
    /// # Description
    ///
    /// Tests a bit at a given index in the bitmap.
    ///
    /// # Parameters
    ///
    /// - `index`: Index of the bit to test.
    ///
    /// # Returns
    ///
    /// Upon success, `Ok(true)` is returned if the bit is set, `Ok(false)` is returned otherwise.
    /// Upon failure, an error is returned instead.
    ///
    pub fn test(&self, index: usize) -> (result: Result<bool, Error>)
        
    {
        let (word, bit): (usize, usize) = self.index(index)?;
        Ok((self.bits[word] & (1 << bit)) != 0)
    }

    ///
    /// # Description
    ///
    /// Returns the `(word, bit)` pair of a index.
    ///
    /// # Parameters
    ///
    /// - `index`: Index of the bit.
    ///
    /// # Returns
    ///
    /// Upon success, the `(word, bit)` pair of the index is returned. Upon
    /// failure, an error is returned instead.
    ///
    fn index(&self, index: usize) -> (result: Result<(usize, usize), Error>)
        
    {
        // Check if the index is out of bounds.
        if index >= self.bits.len() * u8::BITS as usize {
            let reason: &str = "index out of bounds";
            return Err(Error::new(ErrorCode::InvalidArgument, reason));
        }

        Ok(self.index_unchecked(index))
    }

    ///
    /// # Description
    ///
    /// Returns the `(word, bit)` pair of a index without checking bounds.
    ///
    /// # Parameters
    ///
    /// - `index`: Index of the bit.
    ///
    /// # Returns
    ///
    /// The `(word, bit)` pair of the index.
    ///
    fn index_unchecked(&self, index: usize) -> (result: (usize, usize))
        
    {
        let word: usize = index / u8::BITS as usize;
        let bit: usize = index % u8::BITS as usize;
        (word, bit)
    }
}

} // verus!

