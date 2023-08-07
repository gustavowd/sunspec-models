use super::*;

pub fn model64101() -> Model {
    let mut ret = Model {
        start_addr: 0,
        end_addr: 0,
        model_number: 64101,
        qtd: 7,
        update: false,
        data: Vec::new(),
    };
    ret.data.push(DataTypes::SunspecU16(Point { name: "Eltek_Country_Code", offset: 2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Eltek_Feeding_Phase", offset: 1+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Eltek_APD_Method", offset: 2+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Eltek_APD_Power_Ref", offset: 3+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Eltek_RPS_Method", offset: 4+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Eltek_RPS_Q_Ref", offset: 5+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "Eltek_RPS_CosPhi_Ref", offset: 6+2, length: 1, write_access: false, value: -32768i16 } ));
    
    ret
}