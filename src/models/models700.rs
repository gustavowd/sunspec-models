use super::*;

#[derive(Debug, Default, Clone, Copy)]
#[allow(unused)]
#[repr(u16)]
pub enum ACType {
    #[default]
    SINGLEPHASE = 0,
    SPLITPHASE = 1,
    THREEPHASE = 1 << 1,
    PAD=0xFFFF
}

#[derive(Debug, Default, Clone, Copy)]
#[allow(unused)]
#[repr(u16)]
pub enum State {
    #[default]
    OFF = 0,
    ON = 1,
    PAD=0xFFFF
}

#[derive(Debug, Default, Clone, Copy)]
#[allow(unused)]
#[repr(u16)]
pub enum InvState {
    #[default]
    OFF = 0,
    SLEEPING = 1,
    STARTING = 2,
    RUNNING = 3,
    THROTTLED = 4,
    SHUTTINGDOWN = 5,
    FAULT = 6,
    STANDBY = 7,
    PAD=0xFFFF
}

#[derive(Debug, Default, Clone, Copy)]
#[allow(unused)]
#[repr(u16)]
pub enum ConnState {
    #[default]
    DISCONNECTED = 0,
    CONNECTED = 1,
    PAD=0xFFFF
}

#[derive(Debug, Clone)]
pub struct Model701 {
    pub start_addr: u16,
    pub model_number: u16,
    pub qtd: u16,
    pub ac_type: ACType,
    pub st: State,
    pub inv_st: InvState,
    pub conn_st: ConnState,
    pub alrm: u32,
    pub der_mode: u32,
    pub w: i16,
    pub va: i16,
    pub var: i16,
    pub pf: i16,
    pub a: i16,
    pub llv: u16,
    pub lnv: u16,
    pub hz: u32,
    pub  tot_wh_inj: u64,
    pub  tot_wh_abs: u64,
    pub  tot_varh_inj: u64,
    pub  tot_varh_abs: u64,
    pub  tmp_amb: i16,
    pub  tmp_cab: i16,
    pub  tmp_snk: i16,
    pub  tmp_trns: i16,
    pub  tmp_sw: i16,
    pub  tmp_ot: i16,
    pub  wl1: i16,
    pub  val1: i16,
    pub  varl1: i16,
    pub  pfl1: i16,
    pub  al1: i16,
    pub  vl1l2: u16,
    pub  vl1: u16,
    pub  tot_wh_injl1: u64,
    pub  tot_wh_absl1: u64,
    pub  tot_varh_injl1: u64,
    pub  tot_varh_absl1: u64,
    pub  wl2: i16,
    pub  val2: i16,
    pub  varl2: i16,
    pub  pfl2: i16,
    pub  al2: i16,
    pub  lv2l3: u16,
    pub  vl2: u16,
    pub  tot_wh_injl2: u64,
    pub  tot_wh_absl2: u64,
    pub  tot_varh_injl2: u64,
    pub  tot_varh_absl2: u64,
    pub  wl3: i16,
    pub  val3: i16,
    pub  varl3: i16,
    pub  pfl3: i16,
    pub  al3: i16,
    pub  lv3l1: u16,
    pub  vl3: u16,
    pub  tot_wh_injl3: u64,
    pub  tot_wh_absl3: u64,
    pub  tot_varh_injl3: u64,
    pub  tot_varh_absl3: u64,
    pub  throt_pct: u16,
    pub  throt_src: u32,
    pub  a_sf: i16,
    pub  v_sf: i16,
    pub  hz_sf: i16,
    pub  w_sf: i16,
    pub  pf_sf: i16,
    pub  va_sf: i16,
    pub  var_sf: i16,
    pub  tot_wh_sf: i16,
    pub  tot_varh_sf: i16,
    pub  tmp_sf: i16,
    pub  mn_alrm_info: FixString,
}

impl Models for Model701 {
    fn new () -> Model701 {
        Model701 {
            start_addr: 0,
            model_number: 701,
            qtd: 153,
            ac_type: ACType::default(),
            st: State::default(),
            inv_st: InvState::default(),
            conn_st: ConnState::default(),
            alrm: 0,
            der_mode: 0,
            w: 0,
            va: 0,
            var: 0,
            pf: 0,
            a: 0,
            llv: 0,
            lnv: 0,
            hz: 0,
            tot_wh_inj: 0,
            tot_wh_abs: 0,
            tot_varh_inj: 0,
            tot_varh_abs: 0,
            tmp_amb: 0,
            tmp_cab: 0,
            tmp_snk: 0,
            tmp_trns: 0,
            tmp_sw: 0,
            tmp_ot: 0,
            wl1: 0,
            val1: 0,
            varl1: 0,
            pfl1: 0,
            al1: 0,
            vl1l2: 0,
            vl1: 0,
            tot_wh_injl1: 0,
            tot_wh_absl1: 0,
            tot_varh_injl1: 0,
            tot_varh_absl1: 0,
            wl2: 0,
            val2: 0,
            varl2: 0,
            pfl2: 0,
            al2: 0,
            lv2l3: 0,
            vl2: 0,
            tot_wh_injl2: 0,
            tot_wh_absl2: 0,
            tot_varh_injl2: 0,
            tot_varh_absl2: 0,
            wl3: 0,
            val3: 0,
            varl3: 0,
            pfl3: 0,
            al3: 0,
            lv3l1: 0,
            vl3: 0,
            tot_wh_injl3: 0,
            tot_wh_absl3: 0,
            tot_varh_injl3: 0,
            tot_varh_absl3: 0,
            throt_pct: 0,
            throt_src: 0,
            a_sf: 0,
            v_sf: 0,
            hz_sf: 0,
            w_sf: 0,
            pf_sf: 0,
            va_sf: 0,
            var_sf: 0,
            tot_wh_sf: 0,
            tot_varh_sf: 0,
            tmp_sf: 0,
            mn_alrm_info: FixString { length: 32, value: String::new() },
        }
    }
}

impl From<Model701> for Vec<u16> {
    fn from(mut from: Model701) -> Self {
        let mut registers: Vec<u16> = vec![0; 2];
        registers[0] = from.model_number;
        registers[1] = from.qtd;

        let pointer: *mut u16 = unsafe{mem::transmute(&mut from.ac_type)};
        for i in 0..4 {
            let vec_u16 = u16::encode(unsafe{*pointer.offset(i as isize)});
            registers.extend(vec_u16);
        }
        
        let pointer: *mut u32 = &mut from.alrm as *mut u32;
        for i in 0..2 {
            let vec_u16 = u32::encode(unsafe{*pointer.offset(i as isize)});
            registers.extend(vec_u16);
        }

        let pointer = &mut from.w as *mut i16;
        for i in 0..5 {
            let vec_i16 = i16::encode(unsafe{*pointer.offset(i as isize)});
            registers.extend(vec_i16);
        }

        let pointer = &mut from.llv as *mut u16;
        for i in 0..2 {
            let vec_u16 = u16::encode(unsafe{*pointer.offset(i as isize)});
            registers.extend(vec_u16);
        }

        registers.extend(u32::encode(from.hz));

        let pointer: *mut u64 = &mut from.tot_wh_inj as *mut u64;
        for i in 0..4 {
            let vec_u64 = u64::encode(unsafe{*pointer.offset(i as isize)});
            registers.extend(vec_u64);
        }

        let pointer = &mut from.tmp_amb as *mut i16;
        for i in 0..11 {
            let vec_i16 = i16::encode(unsafe{*pointer.offset(i as isize)});
            registers.extend(vec_i16);
        }
        
        let pointer = &mut from.vl1l2 as *mut u16;
        for i in 0..2 {
            let vec_u16 = u16::encode(unsafe{*pointer.offset(i as isize)});
            registers.extend(vec_u16);
        }

        let pointer: *mut u64 = &mut from.tot_wh_injl1 as *mut u64;
        for i in 0..4 {
            let vec_u64 = u64::encode(unsafe{*pointer.offset(i as isize)});
            registers.extend(vec_u64);
        }

        let pointer = &mut from.wl2 as *mut i16;
        for i in 0..5 {
            let vec_i16 = i16::encode(unsafe{*pointer.offset(i as isize)});
            registers.extend(vec_i16);
        }

        let pointer = &mut from.lv2l3 as *mut u16;
        for i in 0..2 {
            let vec_u16 = u16::encode(unsafe{*pointer.offset(i as isize)});
            registers.extend(vec_u16);
        }

        let pointer: *mut u64 = &mut from.tot_wh_injl2 as *mut u64;
        for i in 0..4 {
            let vec_u64 = u64::encode(unsafe{*pointer.offset(i as isize)});
            registers.extend(vec_u64);
        }

        let pointer = &mut from.wl3 as *mut i16;
        for i in 0..5 {
            let vec_i16 = i16::encode(unsafe{*pointer.offset(i as isize)});
            registers.extend(vec_i16);
        }

        let pointer = &mut from.lv3l1 as *mut u16;
        for i in 0..2 {
            let vec_u16 = u16::encode(unsafe{*pointer.offset(i as isize)});
            registers.extend(vec_u16);
        }

        let pointer: *mut u64 = &mut from.tot_wh_injl3 as *mut u64;
        for i in 0..4 {
            let vec_u64 = u64::encode(unsafe{*pointer.offset(i as isize)});
            registers.extend(vec_u64);
        }

        registers.extend(u16::encode(from.throt_pct));
        registers.extend(u32::encode(from.throt_src));

        let pointer = &mut from.a_sf as *mut i16;
        for i in 0..10 {
            let vec_i16 = i16::encode(unsafe{*pointer.offset(i as isize)});
            registers.extend(vec_i16);
        }

        registers.extend(Vec::<u16>::from(from.mn_alrm_info));

        registers
    }
}
