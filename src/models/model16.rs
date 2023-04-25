use super::*;

pub fn model16() -> Model {
    let mut ret = Model {
        start_addr: 0,
        end_addr: 0,
        model_number: 16,
        qtd: 52,
        update: false,
        data: Vec::new(),
    };
    ret.data.push(DataTypes::SunspecString(Point { name: "Nam", offset: 0+2, length: 4, write_access: true, value: String::new() } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Cfg", offset: 4+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Ctl", offset: 5+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecString(Point { name: "Addr", offset: 6+2, length: 8, write_access: true, value: String::new() } ));
    ret.data.push(DataTypes::SunspecString(Point { name: "Msk", offset: 14+2, length: 8, write_access: true, value: String::new() } ));
    ret.data.push(DataTypes::SunspecString(Point { name: "Gw", offset: 22+2, length: 8, write_access: true, value: String::new() } ));
    ret.data.push(DataTypes::SunspecString(Point { name: "DNS1", offset: 30+2, length: 8, write_access: true, value: String::new() } ));
    ret.data.push(DataTypes::SunspecString(Point { name: "DNS2", offset: 38+2, length: 8, write_access: true, value: String::new() } ));
    ret.data.push(DataTypes::SunspecU64(Point { name: "MAC", offset: 46+2, length: 1, write_access: false, value: 0xFFFFFFFFFFFFFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "LnkCtl", offset: 50+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Pad", offset: 51+2, length: 1, write_access: false, value: 0x8000 } ));
    
    ret
}