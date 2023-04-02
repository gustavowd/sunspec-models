use super::*;

pub fn model122() -> Model {
    let mut ret = Model {
        start_addr: 0,
        end_addr: 0,
        model_number: 122,
        qtd: 44,
        data: Vec::new(),
    };
    ret.data.push(DataTypes::SunspecU16(Point { name: "PVConn", offset: 0+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "StorConn", offset: 1+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "ECPConn", offset: 2+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU64(Point { name: "ActWh", offset: 3+2, length: 4, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU64(Point { name: "ActVAh", offset: 7+2, length: 4, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU64(Point { name: "ActVArhQ1", offset: 11+2, length: 4, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU64(Point { name: "ActVArhQ2", offset: 15+2, length: 4, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU64(Point { name: "ActVArhQ3", offset: 19+2, length: 4, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU64(Point { name: "ActVArhQ4", offset: 23+2, length: 4, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "VArAval", offset: 27+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "VArAval_SF", offset: 28+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "WAval", offset: 29+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "WAval_SF", offset: 30+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "StSetLimMsk", offset: 31+2, length: 2, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "StActCtl", offset: 33+2, length: 2, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecString(Point { name: "TmSrc", offset: 35+2, length: 4, write_access: false, value: String::new() } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "Tms", offset: 39+2, length: 2, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "RtSt", offset: 41+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Ris", offset: 42+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "Ris_SF", offset: 43+2, length: 1, write_access: false, value: 0 } ));
    
    ret
}