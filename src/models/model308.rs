use super::*;

pub fn model308() -> Model {
    let mut ret = Model {
        start_addr: 0,
        end_addr: 0,
        model_number: 308,
        qtd: 4,
        data: Vec::new(),
    };
    ret.data.push(DataTypes::SunspecU16(Point { name: "GHI", offset: 0+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "TmpBOM", offset: 1+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "TmpAmb", offset: 2+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "WndSpd", offset: 3+2, length: 1, write_access: false, value: 0 } ));
    
    ret
}