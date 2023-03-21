use super::*;

#[derive(Debug, Default, Clone, Copy)]
#[allow(unused)]
pub enum Evt {
    #[default]
    NONE = 0,
    NULL1 = 1,
    NULL2 = 1 << 1,
    MEVENTPowerFailure = 1 << 2,
    MEVENTUnderVoltage = 1 << 3,
    MEVENTLowPF = 1 << 4,
    MEVENTOverCurrent = 1 << 5,
    MEVENTOverVoltage = 1 << 6,
    MEVENTMissingSensor = 1 << 7,
    MEVENTReserved1 = 1 << 8,
    MEVENTReserved2 = 1 << 9,
    MEVENTReserved3 = 1 << 10,
    MEVENTReserved4 = 1 << 11,
    MEVENTReserved5 = 1 << 12,
    MEVENTReserved6 = 1 << 13,
    MEVENTReserved7 = 1 << 14,
    MEVENTReserved8 = 1 << 15,
    MEVENTOEM01 = 1 << 16,
    MEVENTOEM02 = 1 << 17,
    MEVENTOEM03 = 1 << 18,
    MEVENTOEM04 = 1 << 19,
    MEVENTOEM05 = 1 << 20,
    MEVENTOEM06 = 1 << 21,
    MEVENTOEM07 = 1 << 22,
    MEVENTOEM08 = 1 << 23,
    MEVENTOEM09 = 1 << 24,
    MEVENTOEM010 = 1 << 25,
    MEVENTOEM011 = 1 << 26,
    MEVENTOEM012 = 1 << 27,
    MEVENTOEM013 = 1 << 28,
    MEVENTOEM014 = 1 << 29,
    MEVENTOEM015 = 1 << 30,
    NULL3 = 1 << 31
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Model213 {
    pub start_addr: u16,
    pub model_number: u16,
    pub qtd: u16,
    pub a: f32,
    pub apha: f32,
    pub aphb: f32,
    pub aphc: f32,
    pub phv: f32,
    pub phvpha: f32,
    pub phvphb: f32,
    pub phvphc: f32,
    pub ppv: f32,
    pub ppvphab: f32,
    pub ppvphbc: f32,
    pub ppvphca: f32,
    pub hz: f32,
    pub w: f32,
    pub wpha: f32,
    pub wphb: f32,
    pub wphc: f32,
    pub va: f32,
    pub vapha: f32,
    pub vaphb: f32,
    pub vaphc: f32,
    pub var: f32,
    pub varpha: f32,
    pub varphb: f32,
    pub varphc: f32,
    pub pf: f32,
    pub pfpha: f32,
    pub pfphb: f32,
    pub pfphc: f32,
    pub totwhexp: f32,
    pub totwhexppha: f32,
    pub totwhexpphb: f32,
    pub totwhexpphc: f32,
    pub totwhimp: f32,
    pub totwhimppha: f32,
    pub totwhimpphb: f32,
    pub totwhimpphc: f32,
    pub totvahexp: f32,
    pub totvahexppha: f32,
    pub totvahexpphb: f32,
    pub totvahexpphc: f32,
    pub totvahimp: f32,
    pub totvahimppha: f32,
    pub totvahimpphb: f32,
    pub totvahimpphc: f32,
    pub totvarhimpq1: f32,
    pub totvarhimpq1pha: f32,
    pub totvarhimpq1phb: f32,
    pub totvarhimpq1phc: f32,
    pub totvarhimpq2: f32,
    pub totvarhimpq2pha: f32,
    pub totvarhimpq2phb: f32,
    pub totvarhimpq2phc: f32,
    pub totvarhimpq3: f32,
    pub totvarhimpq3pha: f32,
    pub totvarhimpq3phb: f32,
    pub totvarhimpq3phc: f32,
    pub totvarhimpq4: f32,
    pub totvarhimpq4pha: f32,
    pub totvarhimpq4phb: f32,
    pub totvarhimpq4phc: f32,
    pub evt: Evt
}

impl Models for Model213 {
    fn new () -> Model213 {
        let mut ret = Model213::default();
        ret.model_number = 213;
        ret.qtd = 124;
        return ret;
    }
}

impl From<Model213> for Vec<u16> {
    fn from(mut from: Model213) -> Self {
        let mut registers: Vec<u16> = vec![0; 2];
        registers[0] = from.model_number;
        registers[1] = from.qtd;
        let pointer = &mut from.a as *mut f32;

        for i in 0..61 {
            let vec_float = f32::encode(unsafe{*pointer.offset(i as isize)});
            registers.extend(vec_float);
        }

        let evt = u32::encode(from.evt as u32);
        registers.extend(evt);
        registers
    }
}
