use super::*;

pub fn model64001() -> Model {
    let mut ret = Model {
        start_addr: 0,
        end_addr: 0,
        model_number: 64001,
        qtd: 71,
        update: false,
        data: Vec::new(),
    };
    ret.data.push(DataTypes::SunspecU16(Point { name: "Cmd", offset: 2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "HWRev", offset: 1+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "RSFWRev", offset: 2+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "OSFWRev", offset: 3+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecString(Point { name: "ProdRev", offset: 4+2, length: 2, write_access: false, value: String::new() } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Boots", offset: 6+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Switch", offset: 7+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Sensors", offset: 8+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Talking", offset: 9+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Status", offset: 10+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Config", offset: 11+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "LEDblink", offset: 12+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "LEDon", offset: 13+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Reserved", offset: 14+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecString(Point { name: "Loc", offset: 15+2, length: 16, write_access: false, value: String::new() } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "S1ID", offset: 31+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "S1Addr", offset: 32+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "S1OSVer", offset: 33+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecString(Point { name: "S1Ver", offset: 34+2, length: 2, write_access: false, value: String::new() } ));
    ret.data.push(DataTypes::SunspecString(Point { name: "S1Serial", offset: 36+2, length: 5, write_access: false, value: String::new() } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "S2ID", offset: 41+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "S2Addr", offset: 42+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "S2OSVer", offset: 43+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecString(Point { name: "S2Ver", offset: 44+2, length: 2, write_access: false, value: String::new() } ));
    ret.data.push(DataTypes::SunspecString(Point { name: "S2Serial", offset: 46+2, length: 5, write_access: false, value: String::new() } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "S3ID", offset: 51+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "S3Addr", offset: 52+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "S3OSVer", offset: 53+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecString(Point { name: "S3Ver", offset: 54+2, length: 2, write_access: false, value: String::new() } ));
    ret.data.push(DataTypes::SunspecString(Point { name: "S3Serial", offset: 56+2, length: 5, write_access: false, value: String::new() } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "S4ID", offset: 61+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "S4Addr", offset: 62+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "S4OSVer", offset: 63+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecString(Point { name: "S4Ver", offset: 64+2, length: 2, write_access: false, value: String::new() } ));
    ret.data.push(DataTypes::SunspecString(Point { name: "S4Serial", offset: 66+2, length: 5, write_access: false, value: String::new() } ));
    
    ret
}