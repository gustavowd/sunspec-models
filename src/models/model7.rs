use super::*;

pub fn model7() -> Model {
    let mut ret = Model {
        start_addr: 0,
        end_addr: 0,
        model_number: 7,
        qtd: 11,
        data: Vec::new(),
    };
    ret.data.push(DataTypes::SunspecU16(Point { name: "RqSeq", offset: 0+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Sts", offset: 1+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "Ts", offset: 2+2, length: 1, write_access: false, value: 0xFFFFFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Ms", offset: 4+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Seq", offset: 5+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Alm", offset: 6+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Rsrvd", offset: 7+2, length: 1, write_access: false, value: 0x8000 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Alg", offset: 8+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "N", offset: 9+2, length: 1, write_access: true, value: 0xFFFF } ));
    
    ret
}