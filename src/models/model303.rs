use super::*;

pub fn model303() -> Model {
    let mut ret = Model {
        start_addr: 0,
        end_addr: 0,
        model_number: 303,
        qtd: 1,
        update: false,
        data: Vec::new(),
    };
    ret.data.push(DataTypes::SunspecI16(Point { name: "TmpBOM", offset: 2, length: 1, write_access: false, value: -32768i16 } ));
    
    ret
}