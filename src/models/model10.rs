use super::*;

pub fn model10() -> Model {
    let mut ret = Model {
        start_addr: 0,
        end_addr: 0,
        model_number: 10,
        qtd: 4,
        data: Vec::new(),
    };
    ret.data.push(DataTypes::SunspecU16(Point { name: "St", offset: 0+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Ctl", offset: 1+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Typ", offset: 2+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Pad", offset: 3+2, length: 1, write_access: false, value: 0x8000 } ));
    
    ret
}