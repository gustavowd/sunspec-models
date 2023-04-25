use super::*;

pub fn model121() -> Model {
    let mut ret = Model {
        start_addr: 0,
        end_addr: 0,
        model_number: 121,
        qtd: 30,
        update: false,
        data: Vec::new(),
    };
    ret.data.push(DataTypes::SunspecU16(Point { name: "WMax", offset: 0+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "VRef", offset: 1+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "VRefOfs", offset: 2+2, length: 1, write_access: true, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "VMax", offset: 3+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "VMin", offset: 4+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "VAMax", offset: 5+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "VArMaxQ1", offset: 6+2, length: 1, write_access: true, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "VArMaxQ2", offset: 7+2, length: 1, write_access: true, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "VArMaxQ3", offset: 8+2, length: 1, write_access: true, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "VArMaxQ4", offset: 9+2, length: 1, write_access: true, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "WGra", offset: 10+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "PFMinQ1", offset: 11+2, length: 1, write_access: true, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "PFMinQ2", offset: 12+2, length: 1, write_access: true, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "PFMinQ3", offset: 13+2, length: 1, write_access: true, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "PFMinQ4", offset: 14+2, length: 1, write_access: true, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "VArAct", offset: 15+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "ClcTotVA", offset: 16+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "MaxRmpRte", offset: 17+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "ECPNomHz", offset: 18+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "ConnPh", offset: 19+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "WMax_SF", offset: 20+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "VRef_SF", offset: 21+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "VRefOfs_SF", offset: 22+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "VMinMax_SF", offset: 23+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "VAMax_SF", offset: 24+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "VArMax_SF", offset: 25+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "WGra_SF", offset: 26+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "PFMin_SF", offset: 27+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "MaxRmpRte_SF", offset: 28+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "ECPNomHz_SF", offset: 29+2, length: 1, write_access: false, value: -32768i16 } ));
    
    ret
}