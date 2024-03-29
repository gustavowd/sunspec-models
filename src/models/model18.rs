use super::*;

pub fn model18() -> Model {
    let mut ret = Model {
        start_addr: 0,
        end_addr: 0,
        model_number: 18,
        qtd: 22,
        update: false,
        data: Vec::new(),
    };
    ret.data.push(DataTypes::SunspecString(Point { name: "Nam", offset: 2, length: 4, write_access: true, value: String::new() } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "IMEI", offset: 4+2, length: 2, write_access: true, value: 0xFFFFFFFF } ));
    ret.data.push(DataTypes::SunspecString(Point { name: "APN", offset: 6+2, length: 4, write_access: true, value: String::new() } ));
    ret.data.push(DataTypes::SunspecString(Point { name: "Num", offset: 10+2, length: 6, write_access: true, value: String::new() } ));
    ret.data.push(DataTypes::SunspecString(Point { name: "Pin", offset: 16+2, length: 6, write_access: true, value: String::new() } ));
    
    ret
}