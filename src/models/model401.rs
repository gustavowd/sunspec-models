use super::*;

pub fn model401() -> Model {
    let mut ret = Model {
        start_addr: 0,
        end_addr: 0,
        model_number: 401,
        qtd: 22,
        data: Vec::new(),
    };
    ret.data.push(DataTypes::SunspecI16(Point { name: "DCA_SF", offset: 0+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "DCAhr_SF", offset: 1+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "DCV_SF", offset: 2+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "DCAMax", offset: 3+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "N", offset: 4+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "Evt", offset: 5+2, length: 1, write_access: false, value: 0xFFFFFFFF } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "EvtVnd", offset: 7+2, length: 1, write_access: false, value: 0xFFFFFFFF } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "DCA", offset: 9+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "DCAhr", offset: 10+2, length: 1, write_access: false, value: 0xFFFFFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "DCV", offset: 12+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "Tmp", offset: 13+2, length: 1, write_access: false, value: -32768i16 } ));
    
    ret
}