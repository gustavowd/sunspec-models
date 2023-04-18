use super::*;

pub fn model203() -> Model {
    let mut ret = Model {
        start_addr: 0,
        end_addr: 0,
        model_number: 203,
        qtd: 105,
        data: Vec::new(),
    };
    ret.data.push(DataTypes::SunspecI16(Point { name: "A", offset: 0+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "AphA", offset: 1+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "AphB", offset: 2+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "AphC", offset: 3+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "A_SF", offset: 4+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "PhV", offset: 5+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "PhVphA", offset: 6+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "PhVphB", offset: 7+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "PhVphC", offset: 8+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "PPV", offset: 9+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "PhVphAB", offset: 10+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "PhVphBC", offset: 11+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "PhVphCA", offset: 12+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "V_SF", offset: 13+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "Hz", offset: 14+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "Hz_SF", offset: 15+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "W", offset: 16+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "WphA", offset: 17+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "WphB", offset: 18+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "WphC", offset: 19+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "W_SF", offset: 20+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "VA", offset: 21+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "VAphA", offset: 22+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "VAphB", offset: 23+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "VAphC", offset: 24+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "VA_SF", offset: 25+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "VAR", offset: 26+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "VARphA", offset: 27+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "VARphB", offset: 28+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "VARphC", offset: 29+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "VAR_SF", offset: 30+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "PF", offset: 31+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "PFphA", offset: 32+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "PFphB", offset: 33+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "PFphC", offset: 34+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "PF_SF", offset: 35+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "TotWhExp", offset: 36+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "TotWhExpPhA", offset: 38+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "TotWhExpPhB", offset: 40+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "TotWhExpPhC", offset: 42+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "TotWhImp", offset: 44+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "TotWhImpPhA", offset: 46+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "TotWhImpPhB", offset: 48+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "TotWhImpPhC", offset: 50+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "TotWh_SF", offset: 52+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "TotVAhExp", offset: 53+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "TotVAhExpPhA", offset: 55+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "TotVAhExpPhB", offset: 57+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "TotVAhExpPhC", offset: 59+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "TotVAhImp", offset: 61+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "TotVAhImpPhA", offset: 63+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "TotVAhImpPhB", offset: 65+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "TotVAhImpPhC", offset: 67+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "TotVAh_SF", offset: 69+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "TotVArhImpQ1", offset: 70+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "TotVArhImpQ1PhA", offset: 72+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "TotVArhImpQ1PhB", offset: 74+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "TotVArhImpQ1PhC", offset: 76+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "TotVArhImpQ2", offset: 78+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "TotVArhImpQ2PhA", offset: 80+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "TotVArhImpQ2PhB", offset: 82+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "TotVArhImpQ2PhC", offset: 84+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "TotVArhExpQ3", offset: 86+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "TotVArhExpQ3PhA", offset: 88+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "TotVArhExpQ3PhB", offset: 90+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "TotVArhExpQ3PhC", offset: 92+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "TotVArhExpQ4", offset: 94+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "TotVArhExpQ4PhA", offset: 96+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "TotVArhExpQ4PhB", offset: 98+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "TotVArhExpQ4PhC", offset: 100+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "TotVArh_SF", offset: 102+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "Evt", offset: 103+2, length: 1, write_access: false, value: 0xFFFFFFFF } ));
    
    ret
}