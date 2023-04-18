use super::*;

pub fn model128() -> Model {
    let mut ret = Model {
        start_addr: 0,
        end_addr: 0,
        model_number: 128,
        qtd: 14,
        data: Vec::new(),
    };
    ret.data.push(DataTypes::SunspecU16(Point { name: "ArGraMod", offset: 0+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "ArGraSag", offset: 1+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "ArGraSwell", offset: 2+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "ModEna", offset: 3+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "FilTms", offset: 4+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "DbVMin", offset: 5+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "DbVMax", offset: 6+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "BlkZnV", offset: 7+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "HysBlkZnV", offset: 8+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "BlkZnTmms", offset: 9+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "HoldTmms", offset: 10+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "ArGra_SF", offset: 11+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "VRefPct_SF", offset: 12+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Pad", offset: 13+2, length: 1, write_access: false, value: 0x8000 } ));
    
    ret
}