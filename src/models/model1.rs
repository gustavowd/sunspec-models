use super::*;

pub fn model1() -> Model {
    let mut ret = Model {
        start_addr: 0,
        end_addr: 0,
        model_number: 1,
        qtd: 66,
        data: Vec::new(),
    };
    ret.data.push(DataTypes::SunspecString(Point { name: "Mn", offset: 0+2, length: 16, write_access: false, value: String::new() } ));
    ret.data.push(DataTypes::SunspecString(Point { name: "Md", offset: 16+2, length: 16, write_access: false, value: String::new() } ));
    ret.data.push(DataTypes::SunspecString(Point { name: "Opt", offset: 32+2, length: 8, write_access: false, value: String::new() } ));
    ret.data.push(DataTypes::SunspecString(Point { name: "Vr", offset: 40+2, length: 8, write_access: false, value: String::new() } ));
    ret.data.push(DataTypes::SunspecString(Point { name: "SN", offset: 48+2, length: 16, write_access: false, value: String::new() } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "DA", offset: 64+2, length: 1, write_access: true, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Pad", offset: 65+2, length: 1, write_access: false, value: 0 } ));
    
    ret
}