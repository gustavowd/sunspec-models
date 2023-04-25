use super::*;

pub fn model809() -> Model {
    let mut ret = Model {
        start_addr: 0,
        end_addr: 0,
        model_number: 809,
        qtd: 2,
        update: false,
        data: Vec::new(),
    };
    ret.data.push(DataTypes::SunspecU16(Point { name: "StackTBD", offset: 0+2, length: 1, write_access: false, value: 0xFFFF } ));
    
    ret
}