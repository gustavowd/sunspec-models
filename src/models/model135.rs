use super::*;

pub fn model135() -> Model {
    let mut ret = Model {
        start_addr: 0,
        end_addr: 0,
        model_number: 135,
        qtd: 60,
        update: false,
        data: Vec::new(),
    };
    ret.data.push(DataTypes::SunspecU16(Point { name: "ActCrv", offset: 2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "ModEna", offset: 1+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "WinTms", offset: 2+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "RvrtTms", offset: 3+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "RmpTms", offset: 4+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "NCrv", offset: 5+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "NPt", offset: 6+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "Tms_SF", offset: 7+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "Hz_SF", offset: 8+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Pad", offset: 9+2, length: 1, write_access: false, value: 0x8000 } ));
    
    ret
}