use super::*;

pub fn model220() -> Model {
    let mut ret = Model {
        start_addr: 0,
        end_addr: 0,
        model_number: 220,
        qtd: 43,
        update: false,
        data: Vec::new(),
    };
    ret.data.push(DataTypes::SunspecI16(Point { name: "A", offset: 0+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "A_SF", offset: 1+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "PhV", offset: 2+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "V_SF", offset: 3+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "Hz", offset: 4+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "Hz_SF", offset: 5+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "W", offset: 6+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "W_SF", offset: 7+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "VA", offset: 8+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "VA_SF", offset: 9+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "VAR", offset: 10+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "VAR_SF", offset: 11+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "PF", offset: 12+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "PF_SF", offset: 13+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "TotWhExp", offset: 14+2, length: 2, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "TotWhImp", offset: 16+2, length: 2, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "TotWh_SF", offset: 18+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "TotVAhExp", offset: 19+2, length: 2, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "TotVAhImp", offset: 21+2, length: 2, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "TotVAh_SF", offset: 23+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "TotVArhImpQ1", offset: 24+2, length: 2, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "TotVArhImpQ2", offset: 26+2, length: 2, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "TotVArhExpQ3", offset: 28+2, length: 2, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "TotVArhExpQ4", offset: 30+2, length: 2, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "TotVArh_SF", offset: 32+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "Evt", offset: 33+2, length: 2, write_access: false, value: 0xFFFFFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Rsrvd", offset: 35+2, length: 1, write_access: false, value: 0x8000 } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "Ts", offset: 36+2, length: 2, write_access: false, value: 0xFFFFFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Ms", offset: 38+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Seq", offset: 39+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Alg", offset: 40+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "N", offset: 41+2, length: 1, write_access: false, value: 0xFFFF } ));
    
    ret
}