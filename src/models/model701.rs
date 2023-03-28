use super::*;

pub fn model701() -> Model {
    let mut ret = Model {
        start_addr: 0,
        end_addr: 0,
        model_number: 701,
        qtd: 153,
        data: Vec::new(),
    };
    ret.data.push(DataTypes::SunspecU16(Point { name: "ACType", offset: 0+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "St", offset: 1+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "InvSt", offset: 2+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "ConnSt", offset: 3+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "Alrm", offset: 4+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "DERMode", offset: 6+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "W", offset: 8+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "VA", offset: 9+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "Var", offset: 10+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "PF", offset: 11+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "A", offset: 12+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "LLV", offset: 13+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "LNV", offset: 14+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "Hz", offset: 15+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU64(Point { name: "TotWhInj", offset: 17+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU64(Point { name: "TotWhAbs", offset: 21+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU64(Point { name: "TotVarhInj", offset: 25+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU64(Point { name: "TotVarhAbs", offset: 29+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "TmpAmb", offset: 33+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "TmpCab", offset: 34+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "TmpSnk", offset: 35+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "TmpTrns", offset: 36+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "TmpSw", offset: 37+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "TmpOt", offset: 38+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "WL1", offset: 39+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "VAL1", offset: 40+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "VarL1", offset: 41+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "PFL1", offset: 42+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "AL1", offset: 43+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "VL1L2", offset: 44+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "VL1", offset: 45+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU64(Point { name: "TotWhInjL1", offset: 46+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU64(Point { name: "TotWhAbsL1", offset: 50+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU64(Point { name: "TotVarhInjL1", offset: 54+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU64(Point { name: "TotVarhAbsL1", offset: 58+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "WL2", offset: 62+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "VAL2", offset: 63+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "VarL2", offset: 64+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "PFL2", offset: 65+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "AL2", offset: 66+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "VL2L3", offset: 67+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "VL2", offset: 68+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU64(Point { name: "TotWhInjL2", offset: 69+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU64(Point { name: "TotWhAbsL2", offset: 73+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU64(Point { name: "TotVarhInjL2", offset: 77+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU64(Point { name: "TotVarhAbsL2", offset: 81+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "WL3", offset: 85+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "VAL3", offset: 86+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "VarL3", offset: 87+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "PFL3", offset: 88+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "AL3", offset: 89+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "VL3L1", offset: 90+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "VL3", offset: 91+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU64(Point { name: "TotWhInjL3", offset: 92+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU64(Point { name: "TotWhAbsL3", offset: 96+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU64(Point { name: "TotVarhInjL3", offset: 100+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU64(Point { name: "TotVarhAbsL3", offset: 104+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "ThrotPct", offset: 108+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "ThrotSrc", offset: 109+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "A_SF", offset: 111+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "V_SF", offset: 112+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Hz_SF", offset: 113+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "W_SF", offset: 114+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "PF_SF", offset: 115+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "VA_SF", offset: 116+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Var_SF", offset: 117+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "TotWh_SF", offset: 118+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "TotVarh_SF", offset: 119+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Tmp_SF", offset: 120+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecString(Point { name: "MnAlrmInfo", offset: 121+2, length: 32, write_access: false, value: String::new() } ));
    
    ret
}