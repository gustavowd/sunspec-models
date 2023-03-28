use super::*;

pub fn model801() -> Model {
    let mut ret = Model {
        start_addr: 0,
        end_addr: 0,
        model_number: 801,
        qtd: 1,
        data: Vec::new(),
    };
    ret.data.push(DataTypes::SunspecU16(Point { name: "DEPRECATED", offset: 0+2, length: 1, write_access: false, value: 0 } ));
    
    ret
}