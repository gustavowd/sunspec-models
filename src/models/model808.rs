use super::*;

pub fn model808() -> Model {
    let mut ret = Model {
        start_addr: 0,
        end_addr: 0,
        model_number: 808,
        qtd: 2,
        data: Vec::new(),
    };
    ret.data.push(DataTypes::SunspecU16(Point { name: "ModuleTBD", offset: 0+2, length: 1, write_access: false, value: 0xFFFF } ));
    
    ret
}