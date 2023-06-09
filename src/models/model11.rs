use super::*;

pub fn model11() -> Model {
    let mut ret = Model {
        start_addr: 0,
        end_addr: 0,
        model_number: 11,
        qtd: 13,
        update: false,
        data: Vec::new(),
    };
    ret.data.push(DataTypes::SunspecU16(Point { name: "Spd", offset: 0+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "CfgSt", offset: 1+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "St", offset: 2+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecString(Point { name: "MAC", offset: 3+2, length: 3, write_access: false, value: String::new() } ));
    ret.data.push(DataTypes::SunspecString(Point { name: "Nam", offset: 7+2, length: 4, write_access: true, value: String::new() } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Ctl", offset: 11+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "FrcSpd", offset: 12+2, length: 1, write_access: true, value: 0xFFFF } ));
    
    ret
}