use core::ops::Deref;

use elrond_wasm::{
    api::ManagedTypeApi,
    types::{ManagedBuffer, ManagedRef, ManagedVec},
};

pub fn split_buffer<M: ManagedTypeApi>(
    buffer: &ManagedBuffer<M>,
    char: u8,
) -> ManagedVec<M, ManagedBuffer<M>> {
    if buffer.len() == 0 {
        return ManagedVec::new();
    }

    let mut bytes: [u8; 512] = [0; 512];

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
    let mut bytes: [u8; 512] = [0; 512];

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
    let mut bytes: [u8; 512] = [0; 512];

    buffer.load_to_byte_array(&mut bytes);

    ManagedBuffer::new_from_bytes(&bytes[1..buffer.len()])
}

pub fn remove_first_and_last_char<M: ManagedTypeApi>(
    buffer: &ManagedBuffer<M>,
) -> ManagedBuffer<M> {
    let mut bytes: [u8; 512] = [0; 512];

    buffer.load_to_byte_array(&mut bytes);

    ManagedBuffer::new_from_bytes(&bytes[1..buffer.len() - 1])
}

pub fn hex_to_u64<M: ManagedTypeApi>(buffer: &ManagedBuffer<M>) -> Option<u64> {
    let mut bytes: [u8; 512] = [0; 512];

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

pub fn ascii_to_u64<M: ManagedTypeApi>(buffer: &ManagedBuffer<M>) -> Option<u64> {
    let mut bytes: [u8; 512] = [0; 512];

    buffer.load_to_byte_array(&mut bytes);

    let mut result: u64 = 0;

    for i in bytes {
        if i == 0 {
            break;
        }

        result *= 10;
        result += (i as char).to_digit(16)? as u64;
    }

    Some(result)
}

pub fn u64_to_hex<M: ManagedTypeApi>(val: &u64) -> ManagedBuffer<M> {
    let mut reversed_digits = ManagedVec::<M, u8>::new();
    let mut result = val.clone();

    while result > 0 {
        let digit = result % 16;
        result /= 16;

        let digit_char = match digit {
            0 => b'0',
            1 => b'1',
            2 => b'2',
            3 => b'3',
            4 => b'4',
            5 => b'5',
            6 => b'6',
            7 => b'7',
            8 => b'8',
            9 => b'9',
            10 => b'a',
            11 => b'b',
            12 => b'c',
            13 => b'd',
            14 => b'e',
            15 => b'f',
            _ => panic!("invalid digit"),
        };

        reversed_digits.push(digit_char);
    }

    if &reversed_digits.len() == &0 {
        return ManagedBuffer::<M>::new_from_bytes(b"00");
    }

    let mut o = ManagedBuffer::<M>::new();

    if &reversed_digits.len() % 2 != 0 {
        o.append_bytes(b"0");
    }

    for digit in reversed_digits.iter().rev() {
        o.append_bytes(&[digit]);
    }

    return o;
}

pub fn u64_to_ascii<M: ManagedTypeApi>(val: &u64) -> ManagedBuffer<M> {
    let mut reversed_digits = ManagedVec::<M, u8>::new();
    let mut result = val.clone();

    while result > 0 {
        let digit = result % 10;
        result /= 10;

        let digit_char = match digit {
            0 => b'0',
            1 => b'1',
            2 => b'2',
            3 => b'3',
            4 => b'4',
            5 => b'5',
            6 => b'6',
            7 => b'7',
            8 => b'8',
            9 => b'9',
            _ => panic!("invalid digit"),
        };

        reversed_digits.push(digit_char);
    }

    if &reversed_digits.len() == &0 {
        return ManagedBuffer::<M>::new_from_bytes(b"0");
    }

    let mut o = ManagedBuffer::<M>::new();

    for digit in reversed_digits.iter().rev() {
        o.append_bytes(&[digit]);
    }

    return o;
}

pub fn extract_number_from_equippable_name<M: ManagedTypeApi>(
    name: &ManagedBuffer<M>,
) -> Option<u64> {
    let buffers = split_last_occurence(name, b'#');
    let number_buffer = &buffers.1;

    return ascii_to_u64(&number_buffer);
}

pub trait ManagedBufferUtils<M: ManagedTypeApi> {
    /// Set the first character to uppercase
    fn capitalize(&self) -> ManagedBuffer<M>;
    fn equals_ignore_case(&self, other: &ManagedBuffer<M>) -> bool;
    fn append_trailing_character_if_missing(&self, character: u8) -> ManagedBuffer<M>;
    /// The replace method use new_buffer as ManagedBuffer because is it the easier way to implement
    fn replace(&self, old_buffer: &[u8], new_buffer: &ManagedBuffer<M>) -> ManagedBuffer<M>;
    fn contains(&self, to_find: &[u8]) -> bool;
    fn to_lowercase(&self) -> ManagedBuffer<M>;
}

impl<M: ManagedTypeApi> ManagedBufferUtils<M> for ManagedBuffer<M> {
    fn capitalize(&self) -> ManagedBuffer<M> {
        let mut bytes: [u8; 512] = [0; 512];

        self.load_to_byte_array(&mut bytes);

        let mut o = ManagedBuffer::<M>::new();

        // uppercase first letter
        o.append_bytes(&[bytes[0].to_ascii_uppercase()]);
        o.append_bytes(&bytes[1..self.len()]);

        return o;
    }

    fn equals_ignore_case(self: &ManagedBuffer<M>, other: &ManagedBuffer<M>) -> bool {
        if self.len() != other.len() {
            return false;
        }

        let mut self_bytes: [u8; 512] = [0; 512];
        let mut other_bytes: [u8; 512] = [0; 512];

        self.load_to_byte_array(&mut self_bytes);
        other.load_to_byte_array(&mut other_bytes);

        for i in 0..self.len() {
            if self_bytes[i].to_ascii_lowercase() != other_bytes[i].to_ascii_lowercase() {
                return false;
            }
        }

        return true;
    }

    fn replace(&self, old_buffer: &[u8], new_buffer: &ManagedBuffer<M>) -> ManagedBuffer<M> {
        let mut bytes: [u8; 512] = [0; 512];
        self.load_to_byte_array(&mut bytes);

        let mut o = ManagedBuffer::<M>::new();

        let mut elements_to_skip = 0;

        for i in 0..self.len() {
            if elements_to_skip > 0 {
                elements_to_skip -= 1;
                continue;
            }

            if bytes[i] == old_buffer[0] {
                for j in 0..old_buffer.len() {
                    // is not a match, let's continue to the next character
                    if bytes[i + j] != old_buffer[j] {
                        o.append_bytes(&[bytes[i]]);
                        break;
                    }

                    // is it a match
                    if j == old_buffer.len() - 1 {
                        o.append(new_buffer);
                        elements_to_skip = j; // skip the old buffer
                        break;
                    }
                }
            } else {
                o.append_bytes(&[bytes[i]]);
            }
        }

        return o;
    }

    fn contains(&self, to_find: &[u8]) -> bool {
        let mut bytes: [u8; 512] = [0; 512];

        self.load_to_byte_array(&mut bytes);

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
        let mut bytes: [u8; 512] = [0; 512];

        self.load_to_byte_array(&mut bytes);

        let mut o = ManagedBuffer::<M>::new();

        for i in 0..self.len() {
            o.append_bytes(&[bytes[i].to_ascii_lowercase()]);
        }

        return o;
    }

    fn append_trailing_character_if_missing(&self, character: u8) -> ManagedBuffer<M> {
        let mut bytes: [u8; 512] = [0; 512];

        self.load_to_byte_array(&mut bytes);

        let mut o = ManagedBuffer::<M>::new();

        o.append_bytes(&bytes[0..self.len()]);

        if bytes[self.len() - 1] != character {
            o.append_bytes(&[character]);
        }

        return o;
    }
}
