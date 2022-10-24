use core::cmp::Ordering;

use elrond_wasm::{
    api::{ErrorApiImpl, ManagedTypeApi},
    types::{ManagedBuffer, ManagedVec},
};

pub trait ManagedBufferUtils<M: ManagedTypeApi> {
    fn load_512_bytes(&self) -> [u8; 512];

    fn split(&self, char: u8) -> ManagedVec<M, ManagedBuffer<M>>;

    /// The replace method use new_buffer as ManagedBuffer because is it the easier way to implement    
    fn contains_char(&self, to_find: u8) -> bool;
    fn to_lowercase(&self) -> ManagedBuffer<M>;
    fn is_lowercase(&self) -> bool;
    fn get_last_char(&self) -> u8;

    /// Returns 0 if equals. Return 1 if self is after other in the alphabetically order. Returns 0 if self is before other in the alphabetically order.
    fn compare(&self, other: &Self) -> Ordering;
}

impl<M: ManagedTypeApi> ManagedBufferUtils<M> for ManagedBuffer<M> {
    fn load_512_bytes(&self) -> [u8; 512] {
        if self.len() > 512 {
            M::error_api_impl().signal_error(b"ManagedBuffer is too big");
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

    fn contains_char(&self, to_find: u8) -> bool {
        let bytes = self.load_512_bytes();

        for i in 0..self.len() {
            if bytes[i] == to_find {
                return true;
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

    fn is_lowercase(&self) -> bool {
        return self == &self.to_lowercase();
    }
}
