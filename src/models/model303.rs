use super::*;

pub fn model303() -> Model {
    let mut ret = Model {
        start_addr: 0,
        end_addr: 0,
        model_number: 303,
        qtd: 2,
        data: Vec::new(),
    };
    ret.data.push(DataTypes::SunspecI16(Point { name: "TmpBOM", offset: 0+2, length: 1, write_access: false, value: 0 } ));
    
    ret
}