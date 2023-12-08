pub struct ParameterLock {
    syn_parameter_0: u16,
    syn_parameter_1: u16,
    syn_parameter_2: u16,
    syn_parameter_3: u16,
    syn_parameter_4: u16,
    syn_parameter_5: u16,
    syn_parameter_6: u16,
    syn_parameter_7: u16,

    smp_tune: i8,
    smp_fine: i8,
    smp_number: u8,
    smp_bit_reduction: u8,
    smp_start: u16,
    smp_end: u16,
    smp_loop_switch: bool,
    smp_level: u8,

    flt_attack: u8,
    flt_sustain: u8,
    flt_decay: u8,
    flt_release: u8,
    flt_frequency: u8,
    flt_resonance: u8,
    flt_type: FilterType,
    flt_env_depth: i8,

    amp_attack: u8,
    amp_hold: u8,
    amp_decay: u8,
    amp_drive: u8,
    amp_delay: u8,
    amp_reverb: u8,
    amp_pan: i8,
    amp_volume: u8,

    lfo_speed: u8,
    lfo_multiplier: LfoMultiplier,
    lfo_fade: u8,
    lfo_dest: LfoDestination,
    lfo_wav: LfoWaveform,
    lfo_start_phase: u8,
    lfo_mode: LfoMode,
    lfo_depth: i8,
}

pub struct TrigFxTrackParameterLock {
    // TODO:
}

// #define AR_PLOCK_TYPE_UNUSED        (0xFFu)

// #define AR_PLOCK_TYPE_MP0           (0x00u)  /* <syn> first machine parameter.  Also see e.g. AR_M_BDCLASSIC_P* */
// #define AR_PLOCK_TYPE_MP1           (0x01u)  /* <syn> second machine parameter                                  */
// #define AR_PLOCK_TYPE_MP2           (0x02u)  /* ..                                                              */
// #define AR_PLOCK_TYPE_MP3           (0x03u)  /* ..                                                              */
// #define AR_PLOCK_TYPE_MP4           (0x04u)  /* ..                                                              */
// #define AR_PLOCK_TYPE_MP5           (0x05u)  /* ..                                                              */
// #define AR_PLOCK_TYPE_MP6           (0x06u)  /* ..                                                              */
// #define AR_PLOCK_TYPE_MP7           (0x07u)  /* <syn> 8th machine parameter                                     */
// #define AR_PLOCK_TYPE_SMP_TUNE      (0x08u)  /* <sample> tune (0x28=-24, 0x40=+0, 0x58=+24) */
// #define AR_PLOCK_TYPE_SMP_FINE      (0x09u)  /* <sample> fine (0x00=-64, 0x40=+0, 0x7F=+63) */
// #define AR_PLOCK_TYPE_SMP_NR        (0x0Au)  /* <sample> nr (0(off), 1..127)                */
// #define AR_PLOCK_TYPE_SMP_BITRDC    (0x0Bu)  /* <sample> bitreduction (0..127)              */
// #define AR_PLOCK_TYPE_SMP_START     (0x0Cu)  /* <sample> start (0..120)                     */
// #define AR_PLOCK_TYPE_SMP_END       (0x0Du)  /* <sample> end (0..120)                       */
// #define AR_PLOCK_TYPE_SMP_LOOPSW    (0x0Eu)  /* <sample> loopsw (0..1)                      */
// #define AR_PLOCK_TYPE_SMP_LEVEL     (0x0Fu)  /* <sample> level (0..127)                     */
// #define AR_PLOCK_TYPE_FLT_ATTACK    (0x10u)  /* <filter> attacktime (0..127)                                  */
// #define AR_PLOCK_TYPE_FLT_SUSTAIN   (0x11u)  /* <filter> sustainlevel (0..127)                                */
// #define AR_PLOCK_TYPE_FLT_DECAY     (0x12u)  /* <filter> decaytime (0..127)                                   */
// #define AR_PLOCK_TYPE_FLT_RELEASE   (0x13u)  /* <filter> releasetime (0..127)                                 */
// #define AR_PLOCK_TYPE_FLT_FREQ      (0x14u)  /* <filter> frequency (0..127)                                   */
// #define AR_PLOCK_TYPE_FLT_RESO      (0x15u)  /* <filter> resonance (0..127)                                   */
// #define AR_PLOCK_TYPE_FLT_TYPE      (0x16u)  /* <filter> type (0=lp2, 1=lp1, 2=bp, 3=hp1, 4=hp2, 5=bs, 6=pk). */
//                                              /*                See AR_FLT_TYPE_xxx                            */
// #define AR_PLOCK_TYPE_FLT_ENV       (0x17u)  /* <filter> envdepth (0(-64)..64(0)..127(+63))                   */
// #define AR_PLOCK_TYPE_AMP_ATTACK    (0x18u)  /* <amp> attacktime (0..127)                 */
// #define AR_PLOCK_TYPE_AMP_HOLD      (0x19u)  /* <amp> holdtime (0..127)                   */
// #define AR_PLOCK_TYPE_AMP_DECAY     (0x1Au)  /* <amp> decaytime (0..126,127=inf)          */
// #define AR_PLOCK_TYPE_AMP_DRIVE     (0x1Bu)  /* <amp> overdrive (0..127)                  */
// #define AR_PLOCK_TYPE_AMP_DELAY     (0x1Cu)  /* <amp> delaysend (0..127)                  */
// #define AR_PLOCK_TYPE_AMP_REVERB    (0x1Du)  /* <amp> reverbsend (0..127)                 */
// #define AR_PLOCK_TYPE_AMP_PAN       (0x1Eu)  /* <amp> pan (0(left)..64(ctr)..127(right))  */
// #define AR_PLOCK_TYPE_AMP_VOLUME    (0x1Fu)  /* <amp> volume (0..127)                     */
// #define AR_PLOCK_TYPE_UNKNOWN_20    (0x20u)  /* (todo?) */
// #define AR_PLOCK_TYPE_LFO_SPEED     (0x21u)  /* <lfo> speed (0(-63),64(0),127(+63))                               */
// #define AR_PLOCK_TYPE_LFO_MULTIPLY  (0x22u)  /* <lfo> multiplier (0=1, .., 0xb=2k)                                */
// #define AR_PLOCK_TYPE_LFO_FADE      (0x23u)  /* <lfo> fade (0(-63),64(0),127(+63))                                */
// #define AR_PLOCK_TYPE_LFO_DEST      (0x24u)  /* <lfo> dest (0=off, .., 0x29=reverbsend) (see AR_LFO_DEST_xxx)     */
// #define AR_PLOCK_TYPE_LFO_WAVEFORM  (0x25u)  /* <lfo> waveform (0=tri, 1=sin, 2=sqr, 3=saw, 4=exp, 5=rmp, 6=rnd). */
//                                              /*                 See AR_LFO_WAVEFORM_xxx                           */
// #define AR_PLOCK_TYPE_LFO_PHASE     (0x26u)  /* <lfo> startphase (0..127)                                         */
// #define AR_PLOCK_TYPE_LFO_TRIGMODE  (0x27u)  /* <lfo> trigmode (0=fre, 1=trg, 2=hld, 3=one, 4=hlf)                */
//                                              /*                 See AR_LFO_TRIGMODE_xxx                           */
// #define AR_PLOCK_TYPE_LFO_DEPTH     (0x28u)  /* <lfo> depth (0..127)                                              */
