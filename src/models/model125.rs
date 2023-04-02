use super::*;

pub fn model125() -> Model {
    let mut ret = Model {
        start_addr: 0,
        end_addr: 0,
        model_number: 125,
        qtd: 8,
        data: Vec::new(),
    };
    ret.data.push(DataTypes::SunspecU16(Point { name: "ModEna", offset: 0+2, length: 1, write_access: true, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "SigType", offset: 1+2, length: 1, write_access: true, value: 0 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "Sig", offset: 2+2, length: 1, write_access: true, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "WinTms", offset: 3+2, length: 1, write_access: true, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "RvtTms", offset: 4+2, length: 1, write_access: true, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "RmpTms", offset: 5+2, length: 1, write_access: true, value: 0 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "Sig_SF", offset: 6+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Pad", offset: 7+2, length: 1, write_access: false, value: 0 } ));
    
    ret
}