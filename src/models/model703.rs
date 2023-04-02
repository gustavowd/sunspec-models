use super::*;

pub fn model703() -> Model {
    let mut ret = Model {
        start_addr: 0,
        end_addr: 0,
        model_number: 703,
        qtd: 17,
        data: Vec::new(),
    };
    ret.data.push(DataTypes::SunspecU16(Point { name: "ES", offset: 0+2, length: 1, write_access: true, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "ESVHi", offset: 1+2, length: 1, write_access: true, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "ESVLo", offset: 2+2, length: 1, write_access: true, value: 0 } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "ESHzHi", offset: 3+2, length: 1, write_access: true, value: 0 } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "ESHzLo", offset: 5+2, length: 1, write_access: true, value: 0 } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "ESDlyTms", offset: 7+2, length: 1, write_access: true, value: 0 } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "ESRndTms", offset: 9+2, length: 1, write_access: true, value: 0 } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "ESRmpTms", offset: 11+2, length: 1, write_access: true, value: 0 } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "ESDlyRemTms", offset: 13+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "V_SF", offset: 15+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "Hz_SF", offset: 16+2, length: 1, write_access: false, value: 0 } ));
    
    ret
}