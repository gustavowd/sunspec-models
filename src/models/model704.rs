use super::*;

pub fn model704() -> Model {
    let mut ret = Model {
        start_addr: 0,
        end_addr: 0,
        model_number: 704,
        qtd: 65,
        update: false,
        data: Vec::new(),
    };
    ret.data.push(DataTypes::SunspecU16(Point { name: "PFWInjEna", offset: 0+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "PFWInjEnaRvrt", offset: 1+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "PFWInjRvrtTms", offset: 2+2, length: 1, write_access: true, value: 0xFFFFFFFF } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "PFWInjRvrtRem", offset: 4+2, length: 1, write_access: false, value: 0xFFFFFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "PFWAbsEna", offset: 6+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "PFWAbsEnaRvrt", offset: 7+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "PFWAbsRvrtTms", offset: 8+2, length: 1, write_access: true, value: 0xFFFFFFFF } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "PFWAbsRvrtRem", offset: 10+2, length: 1, write_access: false, value: 0xFFFFFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "WMaxLimPctEna", offset: 12+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "WMaxLimPct", offset: 13+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "WMaxLimPctRvrt", offset: 14+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "WMaxLimPctEnaRvrt", offset: 15+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "WMaxLimPctRvrtTms", offset: 16+2, length: 1, write_access: true, value: 0xFFFFFFFF } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "WMaxLimPctRvrtRem", offset: 18+2, length: 1, write_access: false, value: 0xFFFFFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "WSetEna", offset: 20+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "WSetMod", offset: 21+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecI32(Point { name: "WSet", offset: 22+2, length: 1, write_access: true, value: -2147483648i32 } ));
    ret.data.push(DataTypes::SunspecI32(Point { name: "WSetRvrt", offset: 24+2, length: 1, write_access: true, value: -2147483648i32 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "WSetPct", offset: 26+2, length: 1, write_access: true, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "WSetPctRvrt", offset: 27+2, length: 1, write_access: true, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "WSetEnaRvrt", offset: 28+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "WSetRvrtTms", offset: 29+2, length: 1, write_access: true, value: 0xFFFFFFFF } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "WSetRvrtRem", offset: 31+2, length: 1, write_access: false, value: 0xFFFFFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "VarSetEna", offset: 33+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "VarSetMod", offset: 34+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "VarSetPri", offset: 35+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecI32(Point { name: "VarSet", offset: 36+2, length: 1, write_access: true, value: -2147483648i32 } ));
    ret.data.push(DataTypes::SunspecI32(Point { name: "VarSetRvrt", offset: 38+2, length: 1, write_access: true, value: -2147483648i32 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "VarSetPct", offset: 40+2, length: 1, write_access: true, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "VarSetPctRvrt", offset: 41+2, length: 1, write_access: true, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "VarSetEnaRvrt", offset: 42+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "VarSetRvrtTms", offset: 43+2, length: 1, write_access: true, value: 0xFFFFFFFF } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "VarSetRvrtRem", offset: 45+2, length: 1, write_access: false, value: 0xFFFFFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "WRmp", offset: 47+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "WRmpRef", offset: 48+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "VarRmp", offset: 49+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "AntiIslEna", offset: 50+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "PF_SF", offset: 51+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "WMaxLimPct_SF", offset: 52+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "WSet_SF", offset: 53+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "WSetPct_SF", offset: 54+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "VarSet_SF", offset: 55+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "VarSetPct_SF", offset: 56+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "DERCtlAC.PFWInj.PF", offset: 57+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "DERCtlAC.PFWInj.Ext", offset: 58+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "DERCtlAC.PFWInjRvrt.PF", offset: 59+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "DERCtlAC.PFWInjRvrt.Ext", offset: 60+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "DERCtlAC.PFWAbs.PF", offset: 61+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "DERCtlAC.PFWAbs.Ext", offset: 62+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "DERCtlAC.PFWAbsRvrt.PF", offset: 63+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "DERCtlAC.PFWAbsRvrt.Ext", offset: 64+2, length: 1, write_access: true, value: 0xFFFF } ));
    
    ret
}