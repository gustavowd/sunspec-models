use super::*;

pub fn model601() -> Model {
    let mut ret = Model {
        start_addr: 0,
        end_addr: 0,
        model_number: 601,
        qtd: 48,
        update: false,
        data: Vec::new(),
    };
    ret.data.push(DataTypes::SunspecString(Point { name: "Nam", offset: 0+2, length: 8, write_access: false, value: String::new() } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Typ", offset: 8+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecString(Point { name: "DtLoc", offset: 9+2, length: 5, write_access: false, value: String::new() } ));
    ret.data.push(DataTypes::SunspecString(Point { name: "TmLoc", offset: 14+2, length: 3, write_access: false, value: String::new() } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Day", offset: 17+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecI32(Point { name: "GlblElCtl", offset: 18+2, length: 2, write_access: true, value: -2147483648i32 } ));
    ret.data.push(DataTypes::SunspecI32(Point { name: "GlblAzCtl", offset: 20+2, length: 2, write_access: true, value: -2147483648i32 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "GlblCtl", offset: 22+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "GlblAlm", offset: 23+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "Dgr_SF", offset: 24+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "N", offset: 25+2, length: 1, write_access: false, value: 0xFFFF } ));
    
    ret
}