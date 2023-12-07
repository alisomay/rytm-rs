pub enum BdHardSynthParameter {
    Lev,
    Tun, // (64=+0)
    Dec,
    Hld,
    Swt,
    Snp,
    Wav,
    Tic,
}

// Bd Classic
pub enum BdClassicSynthParameter {
    Lev,
    Tun, // (64=+0)
    Dec,
    Hld,
    Swt,
    Swd,
    Wav,
    Tra,
}

// Sd Hard
pub enum SdHardSynthParameter {
    Lev,
    Tun, // (64=+0)
    Dec,
    Swd,
    Tic,
    Nod,
    Nol, // Unavailable
    Swt,
}

// Sd Classic
pub enum SdClassicSynthParameter {
    Lev,
    Tun, // (64=+0)
    Dec,
    Det,
    Snp,
    Nod,
    Nol, // Unavailable
    Bal, // (64=+0)
}

// Rs Hard
pub enum RsHardSynthParameter {
    Lev,
    Tun, // (64=+0)
    Dec,
    Swd,
    Tic,
    Nol, // Unavailable
    Syn,
    Swt,
}

// Rs Classic
pub enum RsClassicSynthParameter {
    Lev,
    T1, // (64=+0)
    Dec,
    Bal, // (64=+0)
    T2,  // (64=+0)
    Sym, // (64=+0)
    Nol, // Unavailable
    Tic,
}

// Cp Classic
pub enum CpClassicSynthParameter {
    Lev,
    Ton, // (0..127)
    Nod,
    Num,
    Rat,
    Nol, // Unavailable
    Rnd,
    Cpd,
}

// Bt Classic
pub enum BtClassicSynthParameter {
    Lev,
    Tun, // (64=+0)
    Dec,
    // Unavailable
    Nol,
    Snp, // (0..3)
    Swd, // (FW1.70)
         // Unavailable
}

// Xt Classic
pub enum XtClassicSynthParameter {
    Lev,
    Tun, // (64=+0)
    Dec,
    Swd,
    Swt,
    Nod,
    Nol, // Unavailable
    Ton, // (64=+0)
}

// Ch Classic
pub enum ChClassicSynthParameter {
    Lev,
    Tun, // (64=+0)
    Dec,
    Col, // (64=+0)
         // Unavailable
         // Unavailable
         // Unavailable
         // Unavailable
}

// Oh Classic
pub enum OhClassicSynthParameter {
    Lev,
    Tun, // (64=+0)
    Dec,
    Col, // (64=+0)
         // Unavailable
         // Unavailable
         // Unavailable
         // Unavailable
}

// Cy Classic
pub enum CyClassicSynthParameter {
    Lev,
    Tun, // (64=+0)
    Dec,
    Col, // (64=+0)
    Ton, // (64=+0)
         // Unavailable
         // Unavailable
         // Unavailable
}

// Cb Classic
pub enum CbClassicSynthParameter {
    Lev,
    Tun, // (64=+0)
    Dec,
    Det,
    Pw1, // (64=+0)
    Pw2, // (64=+0)
         // Unavailable
         // Unavailable
}

// Bd Fm
pub enum BdFmSynthParameter {
    Lev,
    Tun, // (64=+0)
    Dec,
    Fma,
    Swt,
    Fms,
    Fmd,
    Fmt, // (64=+0)
}

// Sd Fm
pub enum SdFmSynthParameter {
    Lev,
    Tun, // (64=+0)
    Dec,
    Fmt, // (64=+0)
    Fmd,
    Nod,
    Nol, // Unavailable
    Fma,
}

// Ut Noise
pub enum UtNoiseSynthParameter {
    Lev,
    Lpf,
    Dec,
    Hpf,
    Lpq,
    Atk,
    Swt,
    Swd, // (64=+0)
}

// Ut Impulse
pub enum UtImpulseSynthParameter {
    Lev,
    Atk,
    Dec,
    // Unavailable
    // Unavailable
    // Unavailable
    // Unavailable
    Pol, // (0 or 1)
}

// Ch Metallic
pub enum ChMetallicSynthParameter {
    Lev,
    Tun, // (64=+0)
    Dec,
    // Unavailable
    // Unavailable
    // Unavailable
    // Unavailable
    // Unavailable
}

// Oh Metallic
pub enum OhMetallicSynthParameter {
    Lev,
    Tun, // (64=+0)
    Dec,
    // Unavailable
    // Unavailable
    // Unavailable
    // Unavailable
    // Unavailable
}

// Cy Metallic
pub enum CyMetallicSynthParameter {
    Lev,
    Tun, // (64=+0)
    Dec,
    Ton, // (64=+0)
    Trd,
    // Unavailable
    // Unavailable
    // Unavailable
}

// Cb Metallic
pub enum CbMetallicSynthParameter {
    Lev,
    Tun, // (64=+0)
    Dec,
    Det,
    Pw1, // (64=+0)
    Pw2, // (64=+0)
         // Unavailable
         // Unavailable
}

// Bd Plastic
pub enum BdPlasticSynthParameter {
    Lev,
    Tun, // (64=+0)
    Dec,
    Typ,
    Mod,
    Swt,
    Swd,
    Tic,
}

// Bd Silky
pub enum BdSilkySynthParameter {
    Lev,
    Tun, // (64=+0)
    Dec,
    Hld,
    Swt,
    Swd,
    Dus,
    Clk,
}

// Sd Natural
pub enum SdNaturalSynthParameter {
    Lev,
    Tun, // (64=+0)
    Bdy,
    Dec,
    Bal, // (0..127)
    Lpf,
    Hpf,
    Res,
}

// Hh Basic
pub enum HhBasicSynthParameter {
    Lev,
    Tun, // (64=+0)
    Dec,
    Ton, // (64=+0)
    Trd,
    Rst, // (0 or 1)
         // Unavailable
         // Unavailable
}

// Cy Ride
pub enum CyRideSynthParameter {
    Lev,
    Tun, // (64=+0)
    Dec,
    Typ, // (0..3=A..D)
    Hit,
    C1,
    C2,
    C3,
}

// Bd Sharp
pub enum BdSharpSynthParameter {
    Lev,
    Tun, // (64=+0)
    Dec,
    Hld,
    Swt,
    Swd,
    Wav, // (0=sinA,1=sinB,2=asinA,3=asinB,4=triA,5=triB,6=ssawA,7=ssawB,8=sawA,9=sawB,10=sqrA,11=sqrB)
    Tic,
}

// DISABLE

// Sy Dual Vco
pub enum SyDualVcoSynthParameter {
    Lev,
    Tun, // (64=+0)
    Dec1,
    Det,
    Dec2,
    Bal, // (64=+0)
    Bnd, // (64=+0)
    Cfg, // (0..79)
}

// Sy Chip
pub enum SyChipSynthParameter {
    Lev,
    Tun, // (64=+0) (uses LSB)
    Dcy,
    Of2, // (40=-24..64=+0..88=+24)
    Of3, // (40=-24..64=+0..88=+24)
    Of4, // (40=-24..64=+0..88=+24)
    Wav, // (0=sin,1=asin,2=tri,3=ssaw,4=saw,5=sqr,6=noise,7=anm1,8=anm2,9=anm3,10=anm4,11=anm5,12=pwm+,13=pwm-,14=triB,15=+tri,16=tri+,17=triX,18=sawB,19=+saw,20=saw+,21=sawX,22=sqrB,23=+sqr,24=sqr+,25=sqrX,26=tbl1,27=tbl2,28=tbl3,29=p1%..127=p99%)
    Spd, // (0=128T,1=128,2=64T,3=128d,4=64,5=32T,6=64d,7=32,8=16T,9=32d,10=16,11=8T,12=16d,13=8,14=4T,15=8d,16=4,17=2T,18=4d,19=2,20=1T,21=2d,22=1,23=1d,24=1.0Hz,25=1.56Hz,26=1.88Hz,27=2Hz,28=3.13Hz,29=3.75Hz,30=4Hz,31=5Hz,32=6.25Hz,33=7.5Hz,34=10Hz,35=12.5Hz,36=15Hz,37=20Hz,38=25Hz,39=30Hz,40=40Hz,41=50Hz,42=60Hz,43=75Hz,44=100Hz,45=120Hz,46=150Hz,47=180Hz,48=200Hz,49=240Hz,50=250Hz,51=300Hz,52=350Hz,53=360Hz,54=400Hz,55=420Hz,56=480Hz,57=240 5Hz,58=200 5Hz,59=150 5Hz,60=120 5Hz,61=100 5Hz,62=60 5Hz,63=50 5Hz,64=30 5Hz,65=25 5Hz)
}

// Bd Acoustic
pub enum BdAcousticSynthParameter {
    Lev,
    Tun, // (64=+0) (uses LSB)
    Dec,
    Hld,
    Swt,
    Swd,
    Wav, // (0=sinA,1=sinB,2=asinA,3=asinB,4=triA,5=triB,6=ssawA,7=ssawB,8=sawA,9=sawB,10=sqrA,11=sqrB)
    Imp,
}

// Sd Acoustic
pub enum SdAcousticSynthParameter {
    Lev,
    Tun, // (64=+0) (uses LSB)
    Bdy,
    Nod,
    Nol,
    Hld,
    Swd,
    Imp,
}

// Sy Raw
pub enum SyRawSynthParameter {
    Lev,
    Tun, // (64=+0) (uses LSB)
    Dcy, // (0..126,127=inf)
    Det, // (64=+0) (uses LSB)
    Nol,
    Wav1, // (0=sin,1=asin,2=tri,3=ssaw,4=asaw,5=saw,6=ring)
    Wav2, // (0=sineA,1=ssawA,2=sineB,3=ssawB)
    Bal,  // (64=+0)
}

// Hh Lab
pub enum HhLabSynthParameter {
    Lev,
    Osc1, // (uses 8bit? LSB)
    Dec,
    Osc2, // (uses 8bit? LSB)
    Osc3, // (uses 8bit? LSB)
    Osc4, // (uses 8bit? LSB)
    Osc5, // (uses 8bit? LSB)
    Osc6, // (uses 8bit? LSB)
}
