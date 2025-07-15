use super::*;

pub fn model160(modules_number: u16) -> Model {
    let mut ret = Model {
        start_addr: 0,
        end_addr: 0,
        model_number: 160,
        qtd: 8,
        update: false,
        data: Vec::new(),
    };
    ret.data.push(DataTypes::SunspecI16(Point { name: "DCA_SF", offset: 2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "DCV_SF", offset: 1+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "DCW_SF", offset: 2+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "DCWH_SF", offset: 3+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "Evt", offset: 4+2, length: 2, write_access: false, value: 0xFFFFFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "N", offset: 6+2, length: 1, write_access: false, value: modules_number } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "TmsPer", offset: 7+2, length: 1, write_access: false, value: 0xFFFF } ));

    for i in 0..modules_number {
        ret.data.push(DataTypes::SunspecU16(Point { name: "ID", offset: 10 + i*20, length: 1, write_access: false, value: 0xFFFF }));
        ret.data.push(DataTypes::SunspecString(Point { name: "IDStr", offset: 11 + i*20, length: 8, write_access: false, value: String::new() }));
        ret.data.push(DataTypes::SunspecU16(Point { name: "DCA", offset: 19 + i*20, length: 1, write_access: false, value: 0xFFFF }));
        ret.data.push(DataTypes::SunspecU16(Point { name: "DCV", offset: 20 + i*20, length: 1, write_access: false, value: 0xFFFF }));
        ret.data.push(DataTypes::SunspecU16(Point { name: "DCW", offset: 21 + i*20, length: 1, write_access: false, value: 0xFFFF }));
        ret.data.push(DataTypes::SunspecU32(Point { name: "DCWH", offset: 22 + i*20, length: 2, write_access: false, value: 0xFFFF }));
        ret.data.push(DataTypes::SunspecU32(Point { name: "Tms", offset: 24 + i*20, length: 2, write_access: false, value: 0xFFFF }));
        ret.data.push(DataTypes::SunspecI16(Point { name: "Tmp", offset: 26 + i*20, length: 1, write_access: false, value: -32768i16 }));
        ret.data.push(DataTypes::SunspecU16(Point { name: "DCSt", offset: 27 + i*20, length: 1, write_access: false, value: 0xFFFF }));
        ret.data.push(DataTypes::SunspecU32(Point { name: "DCEvt", offset: 28 + i*20, length: 2, write_access: false, value: 0xFFFF }));
    }
    ret.qtd += modules_number * 20; // 20 registers per module
    
    ret
}