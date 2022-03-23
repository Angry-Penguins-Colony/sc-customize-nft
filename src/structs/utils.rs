use core::ops::Deref;

use elrond_wasm::{
    api::ManagedTypeApi,
    types::{ManagedBuffer, ManagedRef, ManagedVec},
};

pub fn split_buffer<M: ManagedTypeApi>(
    buffer: &ManagedBuffer<M>,
    char: u8,
) -> ManagedVec<M, ManagedBuffer<M>> {
    let mut bytes: [u8; 256] = [0; 256];

    buffer.load_to_byte_array(&mut bytes);

    let mut output = ManagedVec::<M, ManagedBuffer<M>>::new();

    let mut start_index = 0;

    for (i, &byte) in bytes.iter().enumerate() {
        if byte == char || i >= buffer.len() {
            let slice = ManagedBuffer::new_from_bytes(&bytes[start_index..i]);
            output.push(slice);
            start_index = i + 1;

            if i >= buffer.len() {
                break;
            }
        }
    }

    return output;
}

pub fn split_last_occurence<M: ManagedTypeApi>(
    buffer: &ManagedBuffer<M>,
    char: u8,
) -> (ManagedBuffer<M>, ManagedBuffer<M>) {
    let mut bytes: [u8; 256] = [0; 256];

    buffer.load_to_byte_array(&mut bytes);

    for i in (0..buffer.len() - 1).rev() {
        if bytes[i] == char {
            let first = ManagedBuffer::<M>::new_from_bytes(&bytes[..i]);
            let second = ManagedBuffer::<M>::new_from_bytes(&bytes[i + 1..buffer.len()]);

            return (first, second);
        }
    }

    panic!("no occurence of char {} in bytes {:?}", char, buffer);
}

pub fn remove_first_char<M: ManagedTypeApi>(buffer: &ManagedBuffer<M>) -> ManagedBuffer<M> {
    let mut bytes: [u8; 256] = [0; 256];

    buffer.load_to_byte_array(&mut bytes);

    ManagedBuffer::new_from_bytes(&bytes[1..buffer.len()])
}

pub fn remove_first_and_last_char<M: ManagedTypeApi>(
    buffer: &ManagedBuffer<M>,
) -> ManagedBuffer<M> {
    let mut bytes: [u8; 256] = [0; 256];

    buffer.load_to_byte_array(&mut bytes);

    ManagedBuffer::new_from_bytes(&bytes[1..buffer.len() - 1])
}

pub fn hex_to_u64<M: ManagedTypeApi>(buffer: &ManagedBuffer<M>) -> Option<u64> {
    let mut bytes: [u8; 256] = [0; 256];

    buffer.load_to_byte_array(&mut bytes);

    let mut result: u64 = 0;

    for i in bytes {
        if i == 0 {
            break;
        }

        result *= 16;
        result += (i as char).to_digit(16)? as u64;
    }

    Some(result)
}
