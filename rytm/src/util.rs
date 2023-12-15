//! Internal utilities for the library.

#![allow(unused)]
#![allow(clippy::similar_names)]
// All casts in this file are intended or safe within the context of this library.
//
// One can change `allow` to `warn` to review them if necessary.
#![allow(
    clippy::cast_lossless,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]

use crate::{
    error::ConversionError,
    object::{pattern::MicroTime, sound::types::MachineType},
};
use rytm_sys::{s_u16_t, s_u16_t__bindgen_ty_1};

pub const fn to_s_u16_t_union_a(value: u16) -> s_u16_t {
    let msb = (value >> 8) as u8;
    let lsb = (value & 0xFF) as u8;
    s_u16_t { a: [msb, lsb] }
}

pub const fn to_s_u16_t_union_v(value: u16) -> s_u16_t {
    s_u16_t { v: value }
}

pub const fn to_s_u16_t_union_b(value: u16) -> s_u16_t {
    let msb = (value >> 8) as u8;
    let lsb = (value & 0xFF) as u8;
    s_u16_t {
        b: s_u16_t__bindgen_ty_1 { hi: msb, lo: lsb },
    }
}

pub const fn to_s_u16_t_union_b_from_u8_as_msb(value: u8) -> s_u16_t {
    s_u16_t {
        b: s_u16_t__bindgen_ty_1 { hi: value, lo: 0 },
    }
}

pub const fn to_s_u16_t_union_b_from_u8_as_lsb(value: u8) -> s_u16_t {
    s_u16_t {
        b: s_u16_t__bindgen_ty_1 { hi: 0, lo: value },
    }
}

pub fn assemble_u32_from_u8_array(array: &[u8]) -> u32 {
    let mut result = 0;
    for (i, byte) in array.iter().enumerate() {
        result |= (*byte as u32) << (8 * i);
    }
    result
}

pub fn break_u32_into_u8_array(value: u32) -> [u8; 4] {
    let mut result = [0u8; 4];
    for (i, byte) in result.iter_mut().enumerate() {
        *byte = (value >> (8 * i)) as u8;
    }
    result
}

#[allow(clippy::missing_safety_doc)]
pub unsafe fn from_s_u16_t(value: s_u16_t) -> u16 {
    let msb = value.b.hi as u16;
    let lsb = value.b.lo as u16;
    (msb << 8) | lsb
}

/// Checks if the given machine is compatible for the given track.
pub fn is_machine_compatible_for_track(track_index: usize, machine: MachineType) -> bool {
    let compatible_machines = unsafe { rytm_sys::ar_sound_compatible_machines };
    let compatible_machines_for_track = compatible_machines[track_index];

    let mut compatible_machines_for_track_size = 0;
    loop {
        unsafe {
            let return_id = rytm_sys::ar_sound_get_machine_id_by_track_and_list_idx(
                track_index as u32,
                compatible_machines_for_track_size,
            );
            if return_id == -1 {
                break;
            }
            compatible_machines_for_track_size += 1;
        }
    }

    let compatible_machines_for_track_slice = unsafe {
        std::slice::from_raw_parts(
            compatible_machines_for_track,
            compatible_machines_for_track_size as usize,
        )
    };

    compatible_machines_for_track_slice.contains(&((machine as u8) as i32))
}

pub fn decode_micro_timing_byte(micro_timing_value: i8) -> Result<MicroTime, ConversionError> {
    match micro_timing_value {
        -92 => Ok(MicroTime::N23B384),
        -88 => Ok(MicroTime::N11B192),
        -84 => Ok(MicroTime::N7B128),
        -80 => Ok(MicroTime::N5B96),
        -76 => Ok(MicroTime::N19B384),
        -72 => Ok(MicroTime::N3B64),
        -68 => Ok(MicroTime::N17B384),
        -64 => Ok(MicroTime::N1B24),
        -60 => Ok(MicroTime::N5B128),
        -56 => Ok(MicroTime::N7B192),
        -52 => Ok(MicroTime::N13B384),
        -48 => Ok(MicroTime::N32nd),
        -44 => Ok(MicroTime::N11B384),
        -40 => Ok(MicroTime::N5B192),
        -36 => Ok(MicroTime::N3B128),
        -32 => Ok(MicroTime::N1B48),
        -28 => Ok(MicroTime::N7B384),
        -24 => Ok(MicroTime::N64th),
        -20 => Ok(MicroTime::N5B384),
        -16 => Ok(MicroTime::N1B96),
        -12 => Ok(MicroTime::N1B128),
        -8 => Ok(MicroTime::N1B192),
        -4 => Ok(MicroTime::N1B384),
        0 => Ok(MicroTime::OnGrid),
        4 => Ok(MicroTime::P1B384),
        8 => Ok(MicroTime::P1B192),
        12 => Ok(MicroTime::P1B128),
        16 => Ok(MicroTime::P1B96),
        20 => Ok(MicroTime::P5B384),
        24 => Ok(MicroTime::P64th),
        28 => Ok(MicroTime::P7B384),
        32 => Ok(MicroTime::P1B48),
        36 => Ok(MicroTime::P3B128),
        40 => Ok(MicroTime::P5B192),
        44 => Ok(MicroTime::P11B384),
        48 => Ok(MicroTime::P32nd),
        52 => Ok(MicroTime::P13B384),
        56 => Ok(MicroTime::P7B192),
        60 => Ok(MicroTime::P5B128),
        64 => Ok(MicroTime::P1B24),
        68 => Ok(MicroTime::P17B384),
        72 => Ok(MicroTime::P3B64),
        76 => Ok(MicroTime::P19B384),
        80 => Ok(MicroTime::P5B96),
        84 => Ok(MicroTime::P7B128),
        88 => Ok(MicroTime::P11B192),
        92 => Ok(MicroTime::P23B384),
        _ => Err(ConversionError::Range {
            value: micro_timing_value.to_string(),
            type_name: "MicroTime".into(),
        }),
    }
}

pub const fn encode_micro_timing_byte(micro_timing: MicroTime) -> i8 {
    match micro_timing {
        MicroTime::N23B384 => -92,
        MicroTime::N11B192 => -88,
        MicroTime::N7B128 => -84,
        MicroTime::N5B96 => -80,
        MicroTime::N19B384 => -76,
        MicroTime::N3B64 => -72,
        MicroTime::N17B384 => -68,
        MicroTime::N1B24 => -64,
        MicroTime::N5B128 => -60,
        MicroTime::N7B192 => -56,
        MicroTime::N13B384 => -52,
        MicroTime::N32nd => -48,
        MicroTime::N11B384 => -44,
        MicroTime::N5B192 => -40,
        MicroTime::N3B128 => -36,
        MicroTime::N1B48 => -32,
        MicroTime::N7B384 => -28,
        MicroTime::N64th => -24,
        MicroTime::N5B384 => -20,
        MicroTime::N1B96 => -16,
        MicroTime::N1B128 => -12,
        MicroTime::N1B192 => -8,
        MicroTime::N1B384 => -4,
        MicroTime::OnGrid => 0,
        MicroTime::P1B384 => 4,
        MicroTime::P1B192 => 8,
        MicroTime::P1B128 => 12,
        MicroTime::P1B96 => 16,
        MicroTime::P5B384 => 20,
        MicroTime::P64th => 24,
        MicroTime::P7B384 => 28,
        MicroTime::P1B48 => 32,
        MicroTime::P3B128 => 36,
        MicroTime::P5B192 => 40,
        MicroTime::P11B384 => 44,
        MicroTime::P32nd => 48,
        MicroTime::P13B384 => 52,
        MicroTime::P7B192 => 56,
        MicroTime::P5B128 => 60,
        MicroTime::P1B24 => 64,
        MicroTime::P17B384 => 68,
        MicroTime::P3B64 => 72,
        MicroTime::P19B384 => 76,
        MicroTime::P5B96 => 80,
        MicroTime::P7B128 => 84,
        MicroTime::P11B192 => 88,
        MicroTime::P23B384 => 92,
    }
}

pub fn scale_f32_to_u16(
    input: f32,
    input_min: f32,
    input_max: f32,
    output_min: u16,
    output_max: u16,
) -> u16 {
    let input_range = input_max - input_min;
    let output_range = output_max as f32 - output_min as f32;
    let scale_factor = output_range / input_range;

    let normalized_input = input - input_min;
    let scaled_input = normalized_input.mul_add(scale_factor, output_min as f32);
    scaled_input.round() as u16
}

pub fn scale_u16_to_f32(
    input: u16,
    input_min: u16,
    input_max: u16,
    output_min: f32,
    output_max: f32,
) -> f32 {
    let input_range = input_max as f32 - input_min as f32;
    let output_range = output_max - output_min;
    let scale_factor = output_range / input_range;

    let normalized_input = input as f32 - input_min as f32;
    normalized_input.mul_add(scale_factor, output_min)
}

// Helper function to decode synth parameter float minus plus scaling.
pub fn get_u16_min_max_from_float_range(min: f32, max: f32) -> (u16, u16) {
    // Given example ranges
    let example_float_min = -32.0;
    let example_float_max = 32.0;
    let example_u16_min = 8192u16;
    let example_u16_max = 24576u16;

    // Calculate the scale factor based on the example
    let example_float_range = example_float_max - example_float_min;
    let example_u16_range = example_u16_max as f32 - example_u16_min as f32;
    let scale_factor = example_u16_range / example_float_range;

    // Apply the scale factor to the given range
    let scaled_min = ((min - example_float_min) * scale_factor) as u16 + example_u16_min;
    let scaled_max = ((max - example_float_min) * scale_factor) as u16 + example_u16_min;

    (scaled_min, scaled_max)
}

pub const fn u8_to_i8_midpoint_of_u8_input_range(value: u8, range_start: u8, range_end: u8) -> i8 {
    let midpoint = ((range_start as i16 + range_end as i16 + 1) / 2);
    (value as i16 - midpoint) as i8
}

pub const fn i8_to_u8_midpoint_of_u8_input_range(value: i8, range_start: u8, range_end: u8) -> u8 {
    let midpoint = ((range_start as i16 + range_end as i16 + 1) / 2);
    (value as i16 + midpoint) as u8
}

/// Like `std::slice::partition`, but stable.
///
/// It preserves the order of the partitioned elements.
pub fn stable_partition<T, F>(v: &mut [T], mut predicate: F)
where
    F: FnMut(&T) -> bool,
{
    // 'left' will point to the start of the range where the predicate is false.
    let mut left = 0;

    // Iterate over the slice.s
    while left < v.len() {
        // Move 'left' forward until we find an element where the predicate is false.
        while left < v.len() && predicate(&v[left]) {
            left += 1;
        }

        // If 'left' is at the end, we're done.
        if left == v.len() {
            break;
        }

        // 'right' will point to the next element where the predicate is true.
        let mut right = left + 1;

        // Move 'right' forward to find the next true element.
        while right < v.len() && !predicate(&v[right]) {
            right += 1;
        }

        // If 'right' is at the end, we're done.
        if right == v.len() {
            break;
        }

        // Swap the elements at 'left' and 'right'.
        v.swap(left, right);

        // Now that we've moved a true element to the 'left' position, increment 'left'.
        left += 1;
    }
}
