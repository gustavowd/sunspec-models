use super::*;

pub fn model19() -> Model {
    let mut ret = Model {
        start_addr: 0,
        end_addr: 0,
        model_number: 19,
        qtd: 30,
        data: Vec::new(),
    };
    ret.data.push(DataTypes::SunspecString(Point { name: "Nam", offset: 0+2, length: 4, write_access: true, value: String::new() } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "Rte", offset: 4+2, length: 1, write_access: true, value: 0xFFFFFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Bits", offset: 6+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Pty", offset: 7+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Dup", offset: 8+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Flw", offset: 9+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Auth", offset: 10+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecString(Point { name: "UsrNam", offset: 11+2, length: 12, write_access: false, value: String::new() } ));
    ret.data.push(DataTypes::SunspecString(Point { name: "Pw", offset: 23+2, length: 6, write_access: false, value: String::new() } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Pad", offset: 29+2, length: 1, write_access: false, value: 0x8000 } ));
    
    ret
}