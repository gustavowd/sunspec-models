use super::*;

pub fn model306() -> Model {
    let mut ret = Model {
        start_addr: 0,
        end_addr: 0,
        model_number: 306,
        qtd: 4,
        data: Vec::new(),
    };
    ret.data.push(DataTypes::SunspecU16(Point { name: "GHI", offset: 0+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "A", offset: 1+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "V", offset: 2+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Tmp", offset: 3+2, length: 1, write_access: false, value: 0 } ));
    
    ret
}