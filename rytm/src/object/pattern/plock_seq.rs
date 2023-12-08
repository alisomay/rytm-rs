use derivative::Derivative;
use rytm_sys::ar_plock_seq_t;

#[derive(Clone, Copy, Debug)]
pub struct PlockSeq {
    pub plock_type: u8,
    pub track_nr: u8,
    pub data: [u8; 64],
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

#[derive(Derivative, Clone, Copy)]
#[derivative(Debug)]
pub struct PlockSeqCollection {
    #[derivative(Debug = "ignore")]
    inner: [rytm_sys::ar_plock_seq_t; 72],
}

impl Default for PlockSeqCollection {
    fn default() -> Self {
        Self {
            inner: [rytm_sys::ar_plock_seq_t::default(); 72],
        }
    }
}

impl From<[rytm_sys::ar_plock_seq_t; 72]> for PlockSeqCollection {
    fn from(raws: [rytm_sys::ar_plock_seq_t; 72]) -> Self {
        Self { inner: raws }
    }
}

impl From<PlockSeqCollection> for [rytm_sys::ar_plock_seq_t; 72] {
    fn from(plock_seq_collection: PlockSeqCollection) -> Self {
        plock_seq_collection.inner
    }
}
