use super::*;

pub fn model127() -> Model {
    let mut ret = Model {
        start_addr: 0,
        end_addr: 0,
        model_number: 127,
        qtd: 10,
        data: Vec::new(),
    };
    ret.data.push(DataTypes::SunspecU16(Point { name: "WGra", offset: 0+2, length: 1, write_access: true, value: 0 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "HzStr", offset: 1+2, length: 1, write_access: true, value: 0 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "HzStop", offset: 2+2, length: 1, write_access: true, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "HysEna", offset: 3+2, length: 1, write_access: true, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "ModEna", offset: 4+2, length: 1, write_access: true, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "HzStopWGra", offset: 5+2, length: 1, write_access: true, value: 0 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "WGra_SF", offset: 6+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "HzStrStop_SF", offset: 7+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "RmpIncDec_SF", offset: 8+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Pad", offset: 9+2, length: 1, write_access: false, value: 0 } ));
    
    ret
}