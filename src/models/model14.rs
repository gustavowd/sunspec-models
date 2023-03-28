use super::*;

pub fn model14() -> Model {
    let mut ret = Model {
        start_addr: 0,
        end_addr: 0,
        model_number: 14,
        qtd: 52,
        data: Vec::new(),
    };
    ret.data.push(DataTypes::SunspecString(Point { name: "Nam", offset: 0+2, length: 4, write_access: true, value: String::new() } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Cap", offset: 4+2, length: 1, write_access: true, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Cfg", offset: 5+2, length: 1, write_access: true, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Typ", offset: 6+2, length: 1, write_access: true, value: 0 } ));
    ret.data.push(DataTypes::SunspecString(Point { name: "Addr", offset: 7+2, length: 20, write_access: true, value: String::new() } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Port", offset: 27+2, length: 1, write_access: true, value: 0 } ));
    ret.data.push(DataTypes::SunspecString(Point { name: "User", offset: 28+2, length: 12, write_access: true, value: String::new() } ));
    ret.data.push(DataTypes::SunspecString(Point { name: "Pw", offset: 40+2, length: 12, write_access: true, value: String::new() } ));
    
    ret
}