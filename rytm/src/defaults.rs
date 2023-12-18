#![allow(unused)]

pub const PATTERN_MAX_COUNT: usize = 128;
pub const POOL_SOUND_MAX_COUNT: usize = 128;
pub const KIT_MAX_COUNT: usize = 128;
pub const GLOBAL_MAX_COUNT: usize = 4;
pub const TRACK_MAX_COUNT: usize = 12;
pub const VOICE_MAX_COUNT: usize = 8;

use std::sync::{Arc, Mutex};

use crate::object::{
    pattern::{track::Track, Trig},
    *,
};

pub fn default_work_buffer_sounds() -> [Sound; TRACK_MAX_COUNT] {
    [
        Sound::try_default(0).unwrap(),
        Sound::try_default(1).unwrap(),
        Sound::try_default(2).unwrap(),
        Sound::try_default(3).unwrap(),
        Sound::try_default(4).unwrap(),
        Sound::try_default(5).unwrap(),
        Sound::try_default(6).unwrap(),
        Sound::try_default(7).unwrap(),
        Sound::try_default(8).unwrap(),
        Sound::try_default(9).unwrap(),
        Sound::try_default(10).unwrap(),
        Sound::try_default(11).unwrap(),
    ]
}

pub fn default_tracks(
    owner_pattern_index: usize,
    is_owner_pattern_work_buffer: bool,
    fx_track_ref: Option<Arc<Mutex<Track>>>,
) -> [Track; TRACK_MAX_COUNT] {
    [
        Track::try_default(
            0,
            owner_pattern_index,
            is_owner_pattern_work_buffer,
            fx_track_ref.clone(),
        )
        .unwrap(),
        Track::try_default(
            1,
            owner_pattern_index,
            is_owner_pattern_work_buffer,
            fx_track_ref.clone(),
        )
        .unwrap(),
        Track::try_default(
            2,
            owner_pattern_index,
            is_owner_pattern_work_buffer,
            fx_track_ref.clone(),
        )
        .unwrap(),
        Track::try_default(
            3,
            owner_pattern_index,
            is_owner_pattern_work_buffer,
            fx_track_ref.clone(),
        )
        .unwrap(),
        Track::try_default(
            4,
            owner_pattern_index,
            is_owner_pattern_work_buffer,
            fx_track_ref.clone(),
        )
        .unwrap(),
        Track::try_default(
            5,
            owner_pattern_index,
            is_owner_pattern_work_buffer,
            fx_track_ref.clone(),
        )
        .unwrap(),
        Track::try_default(
            6,
            owner_pattern_index,
            is_owner_pattern_work_buffer,
            fx_track_ref.clone(),
        )
        .unwrap(),
        Track::try_default(
            7,
            owner_pattern_index,
            is_owner_pattern_work_buffer,
            fx_track_ref.clone(),
        )
        .unwrap(),
        Track::try_default(
            8,
            owner_pattern_index,
            is_owner_pattern_work_buffer,
            fx_track_ref.clone(),
        )
        .unwrap(),
        Track::try_default(
            9,
            owner_pattern_index,
            is_owner_pattern_work_buffer,
            fx_track_ref.clone(),
        )
        .unwrap(),
        Track::try_default(
            10,
            owner_pattern_index,
            is_owner_pattern_work_buffer,
            fx_track_ref.clone(),
        )
        .unwrap(),
        Track::try_default(
            11,
            owner_pattern_index,
            is_owner_pattern_work_buffer,
            fx_track_ref,
        )
        .unwrap(),
    ]
}

pub fn default_globals() -> [Global; GLOBAL_MAX_COUNT] {
    [
        Global::try_default(0).unwrap(),
        Global::try_default(1).unwrap(),
        Global::try_default(2).unwrap(),
        Global::try_default(3).unwrap(),
    ]
}

#[allow(clippy::too_many_lines)]
pub fn default_pool_sounds() -> [Sound; POOL_SOUND_MAX_COUNT] {
    [
        Sound::try_default(0).unwrap(),
        Sound::try_default(1).unwrap(),
        Sound::try_default(2).unwrap(),
        Sound::try_default(3).unwrap(),
        Sound::try_default(4).unwrap(),
        Sound::try_default(5).unwrap(),
        Sound::try_default(6).unwrap(),
        Sound::try_default(7).unwrap(),
        Sound::try_default(8).unwrap(),
        Sound::try_default(9).unwrap(),
        Sound::try_default(10).unwrap(),
        Sound::try_default(11).unwrap(),
        Sound::try_default(12).unwrap(),
        Sound::try_default(13).unwrap(),
        Sound::try_default(14).unwrap(),
        Sound::try_default(15).unwrap(),
        Sound::try_default(16).unwrap(),
        Sound::try_default(17).unwrap(),
        Sound::try_default(18).unwrap(),
        Sound::try_default(19).unwrap(),
        Sound::try_default(20).unwrap(),
        Sound::try_default(21).unwrap(),
        Sound::try_default(22).unwrap(),
        Sound::try_default(23).unwrap(),
        Sound::try_default(24).unwrap(),
        Sound::try_default(25).unwrap(),
        Sound::try_default(26).unwrap(),
        Sound::try_default(27).unwrap(),
        Sound::try_default(28).unwrap(),
        Sound::try_default(29).unwrap(),
        Sound::try_default(30).unwrap(),
        Sound::try_default(31).unwrap(),
        Sound::try_default(32).unwrap(),
        Sound::try_default(33).unwrap(),
        Sound::try_default(34).unwrap(),
        Sound::try_default(35).unwrap(),
        Sound::try_default(36).unwrap(),
        Sound::try_default(37).unwrap(),
        Sound::try_default(38).unwrap(),
        Sound::try_default(39).unwrap(),
        Sound::try_default(40).unwrap(),
        Sound::try_default(41).unwrap(),
        Sound::try_default(42).unwrap(),
        Sound::try_default(43).unwrap(),
        Sound::try_default(44).unwrap(),
        Sound::try_default(45).unwrap(),
        Sound::try_default(46).unwrap(),
        Sound::try_default(47).unwrap(),
        Sound::try_default(48).unwrap(),
        Sound::try_default(49).unwrap(),
        Sound::try_default(50).unwrap(),
        Sound::try_default(51).unwrap(),
        Sound::try_default(52).unwrap(),
        Sound::try_default(53).unwrap(),
        Sound::try_default(54).unwrap(),
        Sound::try_default(55).unwrap(),
        Sound::try_default(56).unwrap(),
        Sound::try_default(57).unwrap(),
        Sound::try_default(58).unwrap(),
        Sound::try_default(59).unwrap(),
        Sound::try_default(60).unwrap(),
        Sound::try_default(61).unwrap(),
        Sound::try_default(62).unwrap(),
        Sound::try_default(63).unwrap(),
        Sound::try_default(64).unwrap(),
        Sound::try_default(65).unwrap(),
        Sound::try_default(66).unwrap(),
        Sound::try_default(67).unwrap(),
        Sound::try_default(68).unwrap(),
        Sound::try_default(69).unwrap(),
        Sound::try_default(70).unwrap(),
        Sound::try_default(71).unwrap(),
        Sound::try_default(72).unwrap(),
        Sound::try_default(73).unwrap(),
        Sound::try_default(74).unwrap(),
        Sound::try_default(75).unwrap(),
        Sound::try_default(76).unwrap(),
        Sound::try_default(77).unwrap(),
        Sound::try_default(78).unwrap(),
        Sound::try_default(79).unwrap(),
        Sound::try_default(80).unwrap(),
        Sound::try_default(81).unwrap(),
        Sound::try_default(82).unwrap(),
        Sound::try_default(83).unwrap(),
        Sound::try_default(84).unwrap(),
        Sound::try_default(85).unwrap(),
        Sound::try_default(86).unwrap(),
        Sound::try_default(87).unwrap(),
        Sound::try_default(88).unwrap(),
        Sound::try_default(89).unwrap(),
        Sound::try_default(90).unwrap(),
        Sound::try_default(91).unwrap(),
        Sound::try_default(92).unwrap(),
        Sound::try_default(93).unwrap(),
        Sound::try_default(94).unwrap(),
        Sound::try_default(95).unwrap(),
        Sound::try_default(96).unwrap(),
        Sound::try_default(97).unwrap(),
        Sound::try_default(98).unwrap(),
        Sound::try_default(99).unwrap(),
        Sound::try_default(100).unwrap(),
        Sound::try_default(101).unwrap(),
        Sound::try_default(102).unwrap(),
        Sound::try_default(103).unwrap(),
        Sound::try_default(104).unwrap(),
        Sound::try_default(105).unwrap(),
        Sound::try_default(106).unwrap(),
        Sound::try_default(107).unwrap(),
        Sound::try_default(108).unwrap(),
        Sound::try_default(109).unwrap(),
        Sound::try_default(110).unwrap(),
        Sound::try_default(111).unwrap(),
        Sound::try_default(112).unwrap(),
        Sound::try_default(113).unwrap(),
        Sound::try_default(114).unwrap(),
        Sound::try_default(115).unwrap(),
        Sound::try_default(116).unwrap(),
        Sound::try_default(117).unwrap(),
        Sound::try_default(118).unwrap(),
        Sound::try_default(119).unwrap(),
        Sound::try_default(120).unwrap(),
        Sound::try_default(121).unwrap(),
        Sound::try_default(122).unwrap(),
        Sound::try_default(123).unwrap(),
        Sound::try_default(124).unwrap(),
        Sound::try_default(125).unwrap(),
        Sound::try_default(126).unwrap(),
        Sound::try_default(127).unwrap(),
    ]
}

// TODO: Once these are identified remove these helpers:
pub const fn default_perf_ctl_array() -> [u8; 48 * 4] {
    [
        255, 255, 0, 255, 255, 255, 0, 255, 255, 255, 0, 255, 255, 255, 0, 255, 255, 255, 0, 255,
        255, 255, 0, 255, 255, 255, 0, 255, 255, 255, 0, 255, 255, 255, 0, 255, 255, 255, 0, 255,
        255, 255, 0, 255, 255, 255, 0, 255, 255, 255, 0, 255, 255, 255, 0, 255, 255, 255, 0, 255,
        255, 255, 0, 255, 255, 255, 0, 255, 255, 255, 0, 255, 255, 255, 0, 255, 255, 255, 0, 255,
        255, 255, 0, 255, 255, 255, 0, 255, 255, 255, 0, 255, 255, 255, 0, 255, 255, 255, 0, 255,
        255, 255, 0, 255, 255, 255, 0, 255, 255, 255, 0, 255, 255, 255, 0, 255, 255, 255, 0, 255,
        255, 255, 0, 255, 255, 255, 0, 255, 255, 255, 0, 255, 255, 255, 0, 255, 255, 255, 0, 255,
        255, 255, 0, 255, 255, 255, 0, 255, 255, 255, 0, 255, 255, 255, 0, 255, 255, 255, 0, 255,
        255, 255, 0, 255, 255, 255, 0, 255, 255, 255, 0, 255, 255, 255, 0, 255, 255, 255, 0, 255,
        255, 255, 0, 255, 255, 255, 0, 255, 255, 255, 0, 255,
    ]
}

pub const fn default_scene_ctl_array() -> [u8; 48 * 4] {
    [
        255, 255, 0, 255, 255, 255, 0, 255, 255, 255, 0, 255, 255, 255, 0, 255, 255, 255, 0, 255,
        255, 255, 0, 255, 255, 255, 0, 255, 255, 255, 0, 255, 255, 255, 0, 255, 255, 255, 0, 255,
        255, 255, 0, 255, 255, 255, 0, 255, 255, 255, 0, 255, 255, 255, 0, 255, 255, 255, 0, 255,
        255, 255, 0, 255, 255, 255, 0, 255, 255, 255, 0, 255, 255, 255, 0, 255, 255, 255, 0, 255,
        255, 255, 0, 255, 255, 255, 0, 255, 255, 255, 0, 255, 255, 255, 0, 255, 255, 255, 0, 255,
        255, 255, 0, 255, 255, 255, 0, 255, 255, 255, 0, 255, 255, 255, 0, 255, 255, 255, 0, 255,
        255, 255, 0, 255, 255, 255, 0, 255, 255, 255, 0, 255, 255, 255, 0, 255, 255, 255, 0, 255,
        255, 255, 0, 255, 255, 255, 0, 255, 255, 255, 0, 255, 255, 255, 0, 255, 255, 255, 0, 255,
        255, 255, 0, 255, 255, 255, 0, 255, 255, 255, 0, 255, 255, 255, 0, 255, 255, 255, 0, 255,
        255, 255, 0, 255, 255, 255, 0, 255, 255, 255, 0, 255,
    ]
}

pub fn default_trig_array(track_index: usize) -> [Trig; 64] {
    [
        Trig::try_default(0, track_index).unwrap(),
        Trig::try_default(1, track_index).unwrap(),
        Trig::try_default(2, track_index).unwrap(),
        Trig::try_default(3, track_index).unwrap(),
        Trig::try_default(4, track_index).unwrap(),
        Trig::try_default(5, track_index).unwrap(),
        Trig::try_default(6, track_index).unwrap(),
        Trig::try_default(7, track_index).unwrap(),
        Trig::try_default(8, track_index).unwrap(),
        Trig::try_default(9, track_index).unwrap(),
        Trig::try_default(10, track_index).unwrap(),
        Trig::try_default(11, track_index).unwrap(),
        Trig::try_default(12, track_index).unwrap(),
        Trig::try_default(13, track_index).unwrap(),
        Trig::try_default(14, track_index).unwrap(),
        Trig::try_default(15, track_index).unwrap(),
        Trig::try_default(16, track_index).unwrap(),
        Trig::try_default(17, track_index).unwrap(),
        Trig::try_default(18, track_index).unwrap(),
        Trig::try_default(19, track_index).unwrap(),
        Trig::try_default(20, track_index).unwrap(),
        Trig::try_default(21, track_index).unwrap(),
        Trig::try_default(22, track_index).unwrap(),
        Trig::try_default(23, track_index).unwrap(),
        Trig::try_default(24, track_index).unwrap(),
        Trig::try_default(25, track_index).unwrap(),
        Trig::try_default(26, track_index).unwrap(),
        Trig::try_default(27, track_index).unwrap(),
        Trig::try_default(28, track_index).unwrap(),
        Trig::try_default(29, track_index).unwrap(),
        Trig::try_default(30, track_index).unwrap(),
        Trig::try_default(31, track_index).unwrap(),
        Trig::try_default(32, track_index).unwrap(),
        Trig::try_default(33, track_index).unwrap(),
        Trig::try_default(34, track_index).unwrap(),
        Trig::try_default(35, track_index).unwrap(),
        Trig::try_default(36, track_index).unwrap(),
        Trig::try_default(37, track_index).unwrap(),
        Trig::try_default(38, track_index).unwrap(),
        Trig::try_default(39, track_index).unwrap(),
        Trig::try_default(40, track_index).unwrap(),
        Trig::try_default(41, track_index).unwrap(),
        Trig::try_default(42, track_index).unwrap(),
        Trig::try_default(43, track_index).unwrap(),
        Trig::try_default(44, track_index).unwrap(),
        Trig::try_default(45, track_index).unwrap(),
        Trig::try_default(46, track_index).unwrap(),
        Trig::try_default(47, track_index).unwrap(),
        Trig::try_default(48, track_index).unwrap(),
        Trig::try_default(49, track_index).unwrap(),
        Trig::try_default(50, track_index).unwrap(),
        Trig::try_default(51, track_index).unwrap(),
        Trig::try_default(52, track_index).unwrap(),
        Trig::try_default(53, track_index).unwrap(),
        Trig::try_default(54, track_index).unwrap(),
        Trig::try_default(55, track_index).unwrap(),
        Trig::try_default(56, track_index).unwrap(),
        Trig::try_default(57, track_index).unwrap(),
        Trig::try_default(58, track_index).unwrap(),
        Trig::try_default(59, track_index).unwrap(),
        Trig::try_default(60, track_index).unwrap(),
        Trig::try_default(61, track_index).unwrap(),
        Trig::try_default(62, track_index).unwrap(),
        Trig::try_default(63, track_index).unwrap(),
    ]
}
