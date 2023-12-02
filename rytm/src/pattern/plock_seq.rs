use rytm_sys::ar_plock_seq_t;

#[derive(Clone, Copy, Debug)]
pub struct PlockSeq {
    plock_type: u8,
    track_nr: u8,
    data: [u8; 64],
}

impl Default for PlockSeq {
    fn default() -> Self {
        Self {
            plock_type: 0,
            track_nr: 0,
            data: [0; 64],
        }
    }
}

impl From<&ar_plock_seq_t> for PlockSeq {
    fn from(raw: &ar_plock_seq_t) -> Self {
        Self {
            plock_type: raw.plock_type,
            track_nr: raw.track_nr,
            data: raw.data,
        }
    }
}

impl From<&PlockSeq> for ar_plock_seq_t {
    fn from(plock_seq: &PlockSeq) -> Self {
        Self {
            plock_type: plock_seq.plock_type,
            track_nr: plock_seq.track_nr,
            data: plock_seq.data,
        }
    }
}
