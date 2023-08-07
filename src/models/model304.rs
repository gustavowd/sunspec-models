use super::*;

pub fn model304() -> Model {
    let mut ret = Model {
        start_addr: 0,
        end_addr: 0,
        model_number: 304,
        qtd: 6,
        update: false,
        data: Vec::new(),
    };
    ret.data.push(DataTypes::SunspecI32(Point { name: "Inclx", offset: 2, length: 2, write_access: false, value: -2147483648i32 } ));
    ret.data.push(DataTypes::SunspecI32(Point { name: "Incly", offset: 2+2, length: 2, write_access: false, value: -2147483648i32 } ));
    ret.data.push(DataTypes::SunspecI32(Point { name: "Inclz", offset: 4+2, length: 2, write_access: false, value: -2147483648i32 } ));
    
    ret
}