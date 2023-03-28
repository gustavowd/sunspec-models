use super::*;

pub fn model501() -> Model {
    let mut ret = Model {
        start_addr: 0,
        end_addr: 0,
        model_number: 501,
        qtd: 31,
        data: Vec::new(),
    };
    ret.data.push(DataTypes::SunspecU16(Point { name: "Stat", offset: 0+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "StatVend", offset: 1+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "Evt", offset: 2+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "EvtVend", offset: 4+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Ctl", offset: 6+2, length: 1, write_access: true, value: 0 } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "CtlVend", offset: 7+2, length: 1, write_access: true, value: 0 } ));
    ret.data.push(DataTypes::SunspecI32(Point { name: "CtlVal", offset: 9+2, length: 1, write_access: true, value: 0 } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "Tms", offset: 11+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "OutA", offset: 13+2, length: 2, write_access: false, value: 0.0 } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "OutV", offset: 15+2, length: 2, write_access: false, value: 0.0 } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "OutWh", offset: 17+2, length: 2, write_access: false, value: 0.0 } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "OutW", offset: 19+2, length: 2, write_access: false, value: 0.0 } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "Tmp", offset: 21+2, length: 2, write_access: false, value: 0.0 } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "InA", offset: 23+2, length: 2, write_access: false, value: 0.0 } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "InV", offset: 25+2, length: 2, write_access: false, value: 0.0 } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "InWh", offset: 27+2, length: 2, write_access: false, value: 0.0 } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "InW", offset: 29+2, length: 2, write_access: false, value: 0.0 } ));
    
    ret
}