use super::*;

pub fn model403(modules_number: u16) -> Model {
    let mut ret = Model {
        start_addr: 0,
        end_addr: 0,
        model_number: 403,
        qtd: 16,
        update: false,
        data: Vec::new(),
    };
    ret.data.push(DataTypes::SunspecI16(Point { name: "DCA_SF", offset: 2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "DCAhr_SF", offset: 1+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "DCV_SF", offset: 2+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "DCAMax", offset: 3+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "N", offset: 4+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "Evt", offset: 5+2, length: 2, write_access: false, value: 0xFFFFFFFF } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "EvtVnd", offset: 7+2, length: 2, write_access: false, value: 0xFFFFFFFF } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "DCA", offset: 9+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "DCAhr", offset: 10+2, length: 2, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "DCV", offset: 12+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "Tmp", offset: 13+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "InDCA_SF", offset: 14+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "InDCAhr_SF", offset: 15+2, length: 1, write_access: false, value: -32768i16 } ));
    
    for i in 0..modules_number {
        ret.data.push(DataTypes::SunspecU16(Point { name: "InID", offset: 18 + i*8, length: 1, write_access: false, value: 0xFFFF }));
        ret.data.push(DataTypes::SunspecU32(Point { name: "InEvt", offset: 19 + i*8, length: 2, write_access: false, value: 0xFFFFFFFF }));
        ret.data.push(DataTypes::SunspecU32(Point { name: "InEvtVnd", offset: 21 + i*8, length: 2, write_access: false, value: 0xFFFFFFFF }));
        ret.data.push(DataTypes::SunspecU16(Point { name: "InDCAc", offset: 23 + i*8, length: 1, write_access: false, value: 0xFFFF }));        
        ret.data.push(DataTypes::SunspecU32(Point { name: "InDCAhr", offset: 24 + i*8, length: 2, write_access: false, value: 0xFFFFFFFF }));
    }
    ret.qtd += modules_number * 8; // 8 registers per module

    ret
}