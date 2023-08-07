use super::*;

pub fn model63002() -> Model {
    let mut ret = Model {
        start_addr: 0,
        end_addr: 0,
        model_number: 63002,
        qtd: 8,
        update: false,
        data: Vec::new(),
    };
    ret.data.push(DataTypes::SunspecI16(Point { name: "sunssf_1", offset: 2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "int16_1", offset: 1+2, length: 1, write_access: true, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "int16_2", offset: 2+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "sunssf_2", offset: 3+2, length: 1, write_access: false, value: -32768i16 } ));
    
    ret
}