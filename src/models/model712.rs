use super::*;

pub fn model712() -> Model {
    let mut ret = Model {
        start_addr: 0,
        end_addr: 0,
        model_number: 712,
        qtd: 18,
        update: false,
        data: Vec::new(),
    };
    ret.data.push(DataTypes::SunspecU16(Point { name: "Ena", offset: 2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "AdptCrvReq", offset: 1+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "AdptCrvRslt", offset: 2+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "NPt", offset: 3+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "NCrv", offset: 4+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "RvrtTms", offset: 5+2, length: 2, write_access: true, value: 0xFFFFFFFF } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "RvrtRem", offset: 7+2, length: 2, write_access: false, value: 0xFFFFFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "RvrtCrv", offset: 9+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "W_SF", offset: 10+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "DeptRef_SF", offset: 11+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "DERWattVar.Crv.ActPt", offset: 12+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "DERWattVar.Crv.DeptRef", offset: 13+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "DERWattVar.Crv.Pri", offset: 14+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "DERWattVar.Crv.ReadOnly", offset: 15+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "DERWattVar.Crv.Pt.W", offset: 16+2, length: 1, write_access: true, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "DERWattVar.Crv.Pt.Var", offset: 17+2, length: 1, write_access: true, value: -32768i16 } ));
    
    ret
}