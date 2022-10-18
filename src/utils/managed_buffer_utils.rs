use core::cmp::Ordering;

use elrond_wasm::{
    api::ManagedTypeApi,
    types::{ManagedBuffer, ManagedVec},
};

use crate::sc_panic_self;

pub trait ManagedBufferUtils<M: ManagedTypeApi> {
    fn load_512_bytes(&self) -> [u8; 512];

    fn split(&self, char: u8) -> ManagedVec<M, ManagedBuffer<M>>;

    /// Set the first character to uppercase
    fn capitalize(&self) -> ManagedBuffer<M>;

    /// The replace method use new_buffer as ManagedBuffer because is it the easier way to implement    
    fn contains(&self, to_find: &[u8]) -> bool;
    fn to_lowercase(&self) -> ManagedBuffer<M>;
    fn get_last_char(&self) -> u8;

    /// Returns 0 if equals. Return 1 if self is after other in the alphabetically order. Returns 0 if self is before other in the alphabetically order.
    fn compare(&self, other: &Self) -> Ordering;
}

impl<M: ManagedTypeApi> ManagedBufferUtils<M> for ManagedBuffer<M> {
    fn load_512_bytes(&self) -> [u8; 512] {
        if self.len() > 512 {
            sc_panic_self!(M, "ManagedBuffer is too big");
        }

        let mut bytes: [u8; 512] = [0; 512];

        self.load_to_byte_array(&mut bytes);

        return bytes;
    }

    fn split(&self, char: u8) -> ManagedVec<M, ManagedBuffer<M>> {
        if self.len() == 0 {
            return ManagedVec::new();
        }

        let bytes = self.load_512_bytes();

        let mut output = ManagedVec::<M, ManagedBuffer<M>>::new();

        let mut start_index = 0;

        for (i, &byte) in bytes.iter().enumerate() {
            if byte == char || i >= self.len() {
                let slice = ManagedBuffer::new_from_bytes(&bytes[start_index..i]);
                output.push(slice);
                start_index = i + 1;

                if i >= self.len() {
                    break;
                }
            }
        }

        return output;
    }

    fn get_last_char(&self) -> u8 {
        let bytes = self.load_512_bytes();

        return bytes[self.len() - 1];
    }

    fn capitalize(&self) -> ManagedBuffer<M> {
        let bytes = self.load_512_bytes();

        let mut o = ManagedBuffer::<M>::new();

        // uppercase first letter
        o.append_bytes(&[bytes[0].to_ascii_uppercase()]);
        o.append_bytes(&bytes[1..self.len()]);

        return o;
    }

    fn contains(&self, to_find: &[u8]) -> bool {
        let bytes = self.load_512_bytes();

        // naive implementation of includes() algorithm
        // An upgrade could be the KMP algorithm
        for i in 0..self.len() {
            if bytes[i] == to_find[0] {
                for j in 0..to_find.len() {
                    if bytes[i + j] != to_find[j] {
                        break;
                    }

                    if j == to_find.len() - 1 {
                        return true;
                    }
                }
            }
        }

        return false;
    }

    fn to_lowercase(&self) -> ManagedBuffer<M> {
        let bytes = self.load_512_bytes();

        let mut o = ManagedBuffer::<M>::new();

        for i in 0..self.len() {
            o.append_bytes(&[bytes[i].to_ascii_lowercase()]);
        }

        return o;
    }

    fn compare(&self, other: &Self) -> Ordering {
        let a_bytes = self.load_512_bytes();
        let b_bytes = other.load_512_bytes();

        return a_bytes.cmp(&b_bytes);
    }
}
