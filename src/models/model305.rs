use super::*;

pub fn model305() -> Model {
    let mut ret = Model {
        start_addr: 0,
        end_addr: 0,
        model_number: 305,
        qtd: 36,
        data: Vec::new(),
    };
    ret.data.push(DataTypes::SunspecString(Point { name: "Tm", offset: 0+2, length: 6, write_access: false, value: String::new() } ));
    ret.data.push(DataTypes::SunspecString(Point { name: "Date", offset: 6+2, length: 4, write_access: false, value: String::new() } ));
    ret.data.push(DataTypes::SunspecString(Point { name: "Loc", offset: 10+2, length: 20, write_access: false, value: String::new() } ));
    ret.data.push(DataTypes::SunspecI32(Point { name: "Lat", offset: 30+2, length: 1, write_access: false, value: -2147483648i32 } ));
    ret.data.push(DataTypes::SunspecI32(Point { name: "Long", offset: 32+2, length: 1, write_access: false, value: -2147483648i32 } ));
    ret.data.push(DataTypes::SunspecI32(Point { name: "Alt", offset: 34+2, length: 1, write_access: false, value: -2147483648i32 } ));
    
    ret
}