use super::*;

pub fn model213() -> Model {
    let mut ret = Model {
        start_addr: 0,
        end_addr: 0,
        model_number: 213,
        qtd: 124,
        update: false,
        data: Vec::new(),
    };
    ret.data.push(DataTypes::SunspecF32(Point { name: "A", offset: 2, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "AphA", offset: 2+2, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "AphB", offset: 4+2, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "AphC", offset: 6+2, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "PhV", offset: 8+2, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "PhVphA", offset: 10+2, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "PhVphB", offset: 12+2, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "PhVphC", offset: 14+2, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "PPV", offset: 16+2, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "PPVphAB", offset: 18+2, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "PPVphBC", offset: 20+2, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "PPVphCA", offset: 22+2, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "Hz", offset: 24+2, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "W", offset: 26+2, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "WphA", offset: 28+2, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "WphB", offset: 30+2, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "WphC", offset: 32+2, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "VA", offset: 34+2, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "VAphA", offset: 36+2, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "VAphB", offset: 38+2, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "VAphC", offset: 40+2, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "VAR", offset: 42+2, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "VARphA", offset: 44+2, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "VARphB", offset: 46+2, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "VARphC", offset: 48+2, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "PF", offset: 50+2, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "PFphA", offset: 52+2, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "PFphB", offset: 54+2, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "PFphC", offset: 56+2, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "TotWhExp", offset: 58+2, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "TotWhExpPhA", offset: 60+2, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "TotWhExpPhB", offset: 62+2, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "TotWhExpPhC", offset: 64+2, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "TotWhImp", offset: 66+2, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "TotWhImpPhA", offset: 68+2, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "TotWhImpPhB", offset: 70+2, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "TotWhImpPhC", offset: 72+2, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "TotVAhExp", offset: 74+2, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "TotVAhExpPhA", offset: 76+2, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "TotVAhExpPhB", offset: 78+2, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "TotVAhExpPhC", offset: 80+2, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "TotVAhImp", offset: 82+2, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "TotVAhImpPhA", offset: 84+2, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "TotVAhImpPhB", offset: 86+2, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "TotVAhImpPhC", offset: 88+2, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "TotVArhImpQ1", offset: 90+2, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "TotVArhImpQ1phA", offset: 92+2, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "TotVArhImpQ1phB", offset: 94+2, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "TotVArhImpQ1phC", offset: 96+2, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "TotVArhImpQ2", offset: 98+2, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "TotVArhImpQ2phA", offset: 100+2, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "TotVArhImpQ2phB", offset: 102+2, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "TotVArhImpQ2phC", offset: 104+2, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "TotVArhExpQ3", offset: 106+2, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "TotVArhExpQ3phA", offset: 108+2, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "TotVArhExpQ3phB", offset: 110+2, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "TotVArhExpQ3phC", offset: 112+2, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "TotVArhExpQ4", offset: 114+2, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "TotVArhExpQ4phA", offset: 116+2, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "TotVArhExpQ4phB", offset: 118+2, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "TotVArhExpQ4phC", offset: 120+2, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "Evt", offset: 122+2, length: 2, write_access: false, value: 0xFFFFFFFF } ));
    
    ret
}