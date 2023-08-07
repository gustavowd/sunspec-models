use super::*;

pub fn model8() -> Model {
    let mut ret = Model {
        start_addr: 0,
        end_addr: 0,
        model_number: 8,
        qtd: 3,
        update: false,
        data: Vec::new(),
    };
    ret.data.push(DataTypes::SunspecU16(Point { name: "Fmt", offset: 2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "N", offset: 1+2, length: 1, write_access: false, value: 0xFFFF } ));
    
    ret
}