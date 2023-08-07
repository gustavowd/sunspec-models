use super::*;

pub fn model160() -> Model {
    let mut ret = Model {
        start_addr: 0,
        end_addr: 0,
        model_number: 160,
        qtd: 28,
        update: false,
        data: Vec::new(),
    };
    ret.data.push(DataTypes::SunspecI16(Point { name: "DCA_SF", offset: 2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "DCV_SF", offset: 1+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "DCW_SF", offset: 2+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "DCWH_SF", offset: 3+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "Evt", offset: 4+2, length: 2, write_access: false, value: 0xFFFFFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "N", offset: 6+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "TmsPer", offset: 7+2, length: 1, write_access: false, value: 0xFFFF } ));
    
    ret
}