use super::*;

pub fn model402() -> Model {
    let mut ret = Model {
        start_addr: 0,
        end_addr: 0,
        model_number: 402,
        qtd: 33,
        data: Vec::new(),
    };
    ret.data.push(DataTypes::SunspecU16(Point { name: "DCA_SF", offset: 0+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "DCAhr_SF", offset: 1+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "DCV_SF", offset: 2+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "DCW_SF", offset: 3+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "DCWh_SF", offset: 4+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "DCAMax", offset: 5+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "N", offset: 6+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "Evt", offset: 7+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "EvtVnd", offset: 9+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "DCA", offset: 11+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "DCAhr", offset: 12+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "DCV", offset: 14+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "Tmp", offset: 15+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "DCW", offset: 16+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "DCPR", offset: 17+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "DCWh", offset: 18+2, length: 1, write_access: false, value: 0 } ));
    
    ret
}