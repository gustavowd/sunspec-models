use super::*;

pub fn model133() -> Model {
    let mut ret = Model {
        start_addr: 0,
        end_addr: 0,
        model_number: 133,
        qtd: 66,
        update: false,
        data: Vec::new(),
    };
    ret.data.push(DataTypes::SunspecU32(Point { name: "ActSchd", offset: 0+2, length: 2, write_access: true, value: 0xFFFFFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "ModEna", offset: 2+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "NSchd", offset: 3+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "NPts", offset: 4+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Pad", offset: 5+2, length: 1, write_access: false, value: 0x8000 } ));
    
    ret
}