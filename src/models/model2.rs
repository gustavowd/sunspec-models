use super::*;

pub fn model2() -> Model {
    let mut ret = Model {
        start_addr: 0,
        end_addr: 0,
        model_number: 2,
        qtd: 14,
        data: Vec::new(),
    };
    ret.data.push(DataTypes::SunspecU16(Point { name: "AID", offset: 0+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "N", offset: 1+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "UN", offset: 2+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "St", offset: 3+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "StVnd", offset: 4+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "Evt", offset: 5+2, length: 1, write_access: false, value: 0xFFFFFFFF } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "EvtVnd", offset: 7+2, length: 1, write_access: false, value: 0xFFFFFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Ctl", offset: 9+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "CtlVnd", offset: 10+2, length: 1, write_access: false, value: 0xFFFFFFFF } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "CtlVl", offset: 12+2, length: 1, write_access: false, value: 0xFFFFFFFF } ));
    
    ret
}