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

pub fn model701() -> Model {
    let mut ret = Model {
        start_addr: 0,
        end_addr: 0,
        model_number: 701,
        qtd: 153,
        data: Vec::new(),
    };
    ret.data.push(DataTypes::SunspecU16(Point { name: "ACType", offset: 2, length: 1, write_access: false, value: ACType::default() as u16 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "St", offset: 3, length: 1, write_access: false, value: State::default() as u16 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "InvSt", offset: 4, length: 1, write_access: false, value: InvState::default() as u16 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "ConnSt", offset: 5, length: 1, write_access: false, value: ConnState::default() as u16 } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "Alrm", offset: 6, length: 2, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "DERMode", offset: 8, length: 2, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "W", offset: 10, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "VA", offset: 11, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "Var", offset: 12, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "PF", offset: 13, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "A", offset: 14, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "LLV", offset: 15, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "LNV", offset: 16, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "Hz", offset: 17, length: 2, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU64(Point { name: "TotWhInj", offset: 19, length: 4, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU64(Point { name: "TotWhAbs", offset: 23, length: 4, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU64(Point { name: "TotVarhInj", offset: 27, length: 4, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU64(Point { name: "TotVarhAbs", offset: 31, length: 4, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "TmpAmb", offset: 35, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "TmpCab", offset: 36, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "TmpSnk", offset: 37, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "TmpTrns", offset: 38, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "TmpSw", offset: 39, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "TmpOt", offset: 40, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "WL1", offset: 41, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "VAL1", offset: 42, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "VarL1", offset: 43, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "PFL1", offset: 44, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "AL1", offset: 45, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "VL1L2", offset: 46, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "VL1", offset: 47, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU64(Point { name: "TotWhInjL1", offset: 48, length: 4, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU64(Point { name: "TotWhAbsL1", offset: 52, length: 4, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU64(Point { name: "TotVarhInjL1", offset: 56, length: 4, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU64(Point { name: "TotVarhAbsL1", offset: 60, length: 4, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "WL2", offset: 64, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "VAL2", offset: 65, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "VarL2", offset: 66, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "PFL2", offset: 67, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "AL2", offset: 68, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "VL2L3", offset: 69, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "VL2", offset: 70, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU64(Point { name: "TotWhInjL2", offset: 71, length: 4, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU64(Point { name: "TotWhAbsL2", offset: 75, length: 4, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU64(Point { name: "TotVarhInjL2", offset: 79, length: 4, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU64(Point { name: "TotVarhAbsL2", offset: 83, length: 4, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "WL3", offset: 87, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "VAL3", offset: 88, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "VarL3", offset: 89, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "PFL3", offset: 90, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "AL3", offset: 91, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "VL3L1", offset: 92, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "VL3", offset: 93, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU64(Point { name: "TotWhInjL3", offset: 94, length: 4, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU64(Point { name: "TotWhAbsL3", offset: 98, length: 4, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU64(Point { name: "TotVarhInjL3", offset: 102, length: 4, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU64(Point { name: "TotVarhAbsL3", offset: 106, length: 4, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "ThrotPct", offset: 110, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "ThrotSrc", offset: 111, length: 2, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "A_SF", offset: 113, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "V_SF", offset: 114, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Hz_SF", offset: 115, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "W_SF", offset: 116, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "PF_SF", offset: 117, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "VA_SF", offset: 118, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Var_SF", offset: 119, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "TotWh_SF", offset: 120, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "TotVar_SF", offset: 121, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Tmp_SF", offset: 122, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecString(Point { name: "MnAlrmInfo", offset: 123, length: 32, write_access: false, value: String::new() } ));
    ret
}
