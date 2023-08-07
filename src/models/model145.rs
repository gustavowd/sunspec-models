use super::*;

pub fn model145() -> Model {
    let mut ret = Model {
        start_addr: 0,
        end_addr: 0,
        model_number: 145,
        qtd: 8,
        update: false,
        data: Vec::new(),
    };
    ret.data.push(DataTypes::SunspecU16(Point { name: "NomRmpUpRte", offset: 2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "NomRmpDnRte", offset: 1+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "EmgRmpUpRte", offset: 2+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "EmgRmpDnRte", offset: 3+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "ConnRmpUpRte", offset: 4+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "ConnRmpDnRte", offset: 5+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "AGra", offset: 6+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "Rmp_SF", offset: 7+2, length: 1, write_access: false, value: -32768i16 } ));
    
    ret
}