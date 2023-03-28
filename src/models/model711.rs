use super::*;

pub fn model711() -> Model {
    let mut ret = Model {
        start_addr: 0,
        end_addr: 0,
        model_number: 711,
        qtd: 22,
        data: Vec::new(),
    };
    ret.data.push(DataTypes::SunspecU16(Point { name: "Ena", offset: 0+2, length: 1, write_access: true, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "AdptCtlReq", offset: 1+2, length: 1, write_access: true, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "AdptCtlRslt", offset: 2+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "NCtl", offset: 3+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "RvrtTms", offset: 4+2, length: 1, write_access: true, value: 0 } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "RvrtRem", offset: 6+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "RvrtCtl", offset: 8+2, length: 1, write_access: true, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Db_SF", offset: 9+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "K_SF", offset: 10+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "RspTms_SF", offset: 11+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "DERFreqDroop.Ctl.DbOf", offset: 12+2, length: 1, write_access: true, value: 0 } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "DERFreqDroop.Ctl.DbUf", offset: 14+2, length: 1, write_access: true, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "DERFreqDroop.Ctl.KOf", offset: 16+2, length: 1, write_access: true, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "DERFreqDroop.Ctl.KUf", offset: 17+2, length: 1, write_access: true, value: 0 } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "DERFreqDroop.Ctl.RspTms", offset: 18+2, length: 1, write_access: true, value: 0 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "DERFreqDroop.Ctl.PMin", offset: 20+2, length: 1, write_access: true, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "DERFreqDroop.Ctl.ReadOnly", offset: 21+2, length: 1, write_access: false, value: 0 } ));
    
    ret
}