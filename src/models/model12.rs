use super::*;

pub fn model12() -> Model {
    let mut ret = Model {
        start_addr: 0,
        end_addr: 0,
        model_number: 12,
        qtd: 98,
        update: false,
        data: Vec::new(),
    };
    ret.data.push(DataTypes::SunspecString(Point { name: "Nam", offset: 0+2, length: 4, write_access: true, value: String::new() } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "CfgSt", offset: 4+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "ChgSt", offset: 5+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Cap", offset: 6+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Cfg", offset: 7+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Ctl", offset: 8+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecString(Point { name: "Addr", offset: 9+2, length: 8, write_access: true, value: String::new() } ));
    ret.data.push(DataTypes::SunspecString(Point { name: "Msk", offset: 17+2, length: 8, write_access: true, value: String::new() } ));
    ret.data.push(DataTypes::SunspecString(Point { name: "Gw", offset: 25+2, length: 8, write_access: true, value: String::new() } ));
    ret.data.push(DataTypes::SunspecString(Point { name: "DNS1", offset: 33+2, length: 8, write_access: true, value: String::new() } ));
    ret.data.push(DataTypes::SunspecString(Point { name: "DNS2", offset: 41+2, length: 8, write_access: true, value: String::new() } ));
    ret.data.push(DataTypes::SunspecString(Point { name: "NTP1", offset: 49+2, length: 12, write_access: true, value: String::new() } ));
    ret.data.push(DataTypes::SunspecString(Point { name: "NTP2", offset: 61+2, length: 12, write_access: true, value: String::new() } ));
    ret.data.push(DataTypes::SunspecString(Point { name: "DomNam", offset: 73+2, length: 12, write_access: true, value: String::new() } ));
    ret.data.push(DataTypes::SunspecString(Point { name: "HostNam", offset: 85+2, length: 12, write_access: true, value: String::new() } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Pad", offset: 97+2, length: 1, write_access: false, value: 0x8000 } ));
    
    ret
}