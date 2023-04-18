use super::*;

pub fn model120() -> Model {
    let mut ret = Model {
        start_addr: 0,
        end_addr: 0,
        model_number: 120,
        qtd: 26,
        data: Vec::new(),
    };
    ret.data.push(DataTypes::SunspecU16(Point { name: "DERTyp", offset: 0+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "WRtg", offset: 1+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "WRtg_SF", offset: 2+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "VARtg", offset: 3+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "VARtg_SF", offset: 4+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "VArRtgQ1", offset: 5+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "VArRtgQ2", offset: 6+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "VArRtgQ3", offset: 7+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "VArRtgQ4", offset: 8+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "VArRtg_SF", offset: 9+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "ARtg", offset: 10+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "ARtg_SF", offset: 11+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "PFRtgQ1", offset: 12+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "PFRtgQ2", offset: 13+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "PFRtgQ3", offset: 14+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "PFRtgQ4", offset: 15+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "PFRtg_SF", offset: 16+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "WHRtg", offset: 17+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "WHRtg_SF", offset: 18+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "AhrRtg", offset: 19+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "AhrRtg_SF", offset: 20+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "MaxChaRte", offset: 21+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "MaxChaRte_SF", offset: 22+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "MaxDisChaRte", offset: 23+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "MaxDisChaRte_SF", offset: 24+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Pad", offset: 25+2, length: 1, write_access: false, value: 0x8000 } ));
    
    ret
}