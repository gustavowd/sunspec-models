use super::*;

pub fn model123() -> Model {
    let mut ret = Model {
        start_addr: 0,
        end_addr: 0,
        model_number: 123,
        qtd: 24,
        data: Vec::new(),
    };
    ret.data.push(DataTypes::SunspecU16(Point { name: "Conn_WinTms", offset: 0+2, length: 1, write_access: true, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Conn_RvrtTms", offset: 1+2, length: 1, write_access: true, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Conn", offset: 2+2, length: 1, write_access: true, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "WMaxLimPct", offset: 3+2, length: 1, write_access: true, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "WMaxLimPct_WinTms", offset: 4+2, length: 1, write_access: true, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "WMaxLimPct_RvrtTms", offset: 5+2, length: 1, write_access: true, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "WMaxLimPct_RmpTms", offset: 6+2, length: 1, write_access: true, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "WMaxLim_Ena", offset: 7+2, length: 1, write_access: true, value: 0 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "OutPFSet", offset: 8+2, length: 1, write_access: true, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "OutPFSet_WinTms", offset: 9+2, length: 1, write_access: true, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "OutPFSet_RvrtTms", offset: 10+2, length: 1, write_access: true, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "OutPFSet_RmpTms", offset: 11+2, length: 1, write_access: true, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "OutPFSet_Ena", offset: 12+2, length: 1, write_access: true, value: 0 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "VArWMaxPct", offset: 13+2, length: 1, write_access: true, value: 0 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "VArMaxPct", offset: 14+2, length: 1, write_access: true, value: 0 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "VArAvalPct", offset: 15+2, length: 1, write_access: true, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "VArPct_WinTms", offset: 16+2, length: 1, write_access: true, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "VArPct_RvrtTms", offset: 17+2, length: 1, write_access: true, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "VArPct_RmpTms", offset: 18+2, length: 1, write_access: true, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "VArPct_Mod", offset: 19+2, length: 1, write_access: true, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "VArPct_Ena", offset: 20+2, length: 1, write_access: true, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "WMaxLimPct_SF", offset: 21+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "OutPFSet_SF", offset: 22+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "VArPct_SF", offset: 23+2, length: 1, write_access: false, value: 0 } ));
    
    ret
}