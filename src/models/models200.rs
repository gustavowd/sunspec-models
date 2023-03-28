use super::*;

#[derive(Debug, Default, Clone, Copy)]
#[allow(unused)]
pub enum Evt {
    #[default]
    NONE = 0,
    NULL1 = 1,
    NULL2 = 1 << 1,
    MEVENTPowerFailure = 1 << 2,
    MEVENTUnderVoltage = 1 << 3,
    MEVENTLowPF = 1 << 4,
    MEVENTOverCurrent = 1 << 5,
    MEVENTOverVoltage = 1 << 6,
    MEVENTMissingSensor = 1 << 7,
    MEVENTReserved1 = 1 << 8,
    MEVENTReserved2 = 1 << 9,
    MEVENTReserved3 = 1 << 10,
    MEVENTReserved4 = 1 << 11,
    MEVENTReserved5 = 1 << 12,
    MEVENTReserved6 = 1 << 13,
    MEVENTReserved7 = 1 << 14,
    MEVENTReserved8 = 1 << 15,
    MEVENTOEM01 = 1 << 16,
    MEVENTOEM02 = 1 << 17,
    MEVENTOEM03 = 1 << 18,
    MEVENTOEM04 = 1 << 19,
    MEVENTOEM05 = 1 << 20,
    MEVENTOEM06 = 1 << 21,
    MEVENTOEM07 = 1 << 22,
    MEVENTOEM08 = 1 << 23,
    MEVENTOEM09 = 1 << 24,
    MEVENTOEM010 = 1 << 25,
    MEVENTOEM011 = 1 << 26,
    MEVENTOEM012 = 1 << 27,
    MEVENTOEM013 = 1 << 28,
    MEVENTOEM014 = 1 << 29,
    MEVENTOEM015 = 1 << 30,
    NULL3 = 1 << 31
}

pub fn model213() -> Model {
    let mut ret = Model {
        start_addr: 0,
        end_addr: 0,
        model_number: 213,
        qtd: 124,
        data: Vec::new(),
    };
    ret.data.push(DataTypes::SunspecF32(Point { name: "A", offset: 2, length: 2, write_access: false, value: 0.0 } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "AphA", offset: 4, length: 2, write_access: false, value: 0.0 } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "AphB", offset: 6, length: 2, write_access: false, value: 0.0 } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "AphC", offset: 8, length: 2, write_access: false, value: 0.0 } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "PhV", offset: 10, length: 2, write_access: false, value: 0.0 } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "PhVphA", offset: 12, length: 2, write_access: false, value: 0.0 } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "PhVphB", offset: 14, length: 2, write_access: false, value: 0.0 } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "PhVphC", offset: 16, length: 2, write_access: false, value: 0.0 } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "PPV", offset: 18, length: 2, write_access: false, value: 0.0 } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "PPVphAB", offset: 20, length: 2, write_access: false, value: 0.0 } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "PPVphBC", offset: 22, length: 2, write_access: false, value: 0.0 } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "PPVphCA", offset: 24, length: 2, write_access: false, value: 0.0 } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "Hz", offset: 26, length: 2, write_access: false, value: 0.0 } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "W", offset: 28, length: 2, write_access: false, value: 0.0 } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "WphA", offset: 30, length: 2, write_access: false, value: 0.0 } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "WphB", offset: 32, length: 2, write_access: false, value: 0.0 } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "WphC", offset: 34, length: 2, write_access: false, value: 0.0 } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "VA", offset: 36, length: 2, write_access: false, value: 0.0 } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "VAphA", offset: 38, length: 2, write_access: false, value: 0.0 } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "VAphB", offset: 40, length: 2, write_access: false, value: 0.0 } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "VAphC", offset: 42, length: 2, write_access: false, value: 0.0 } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "VAR", offset: 44, length: 2, write_access: false, value: 0.0 } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "VARphA", offset: 46, length: 2, write_access: false, value: 0.0 } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "VARphB", offset: 48, length: 2, write_access: false, value: 0.0 } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "VARphC", offset: 50, length: 2, write_access: false, value: 0.0 } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "PF", offset: 52, length: 2, write_access: false, value: 0.0 } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "PFphA", offset: 54, length: 2, write_access: false, value: 0.0 } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "PFphB", offset: 56, length: 2, write_access: false, value: 0.0 } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "PFphC", offset: 58, length: 2, write_access: false, value: 0.0 } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "TotWhExp", offset: 60, length: 2, write_access: false, value: 0.0 } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "TotWhExpphA", offset: 62, length: 2, write_access: false, value: 0.0 } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "TotWhExpphB", offset: 64, length: 2, write_access: false, value: 0.0 } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "TotWhExpphC", offset: 66, length: 2, write_access: false, value: 0.0 } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "TotWhImp", offset: 68, length: 2, write_access: false, value: 0.0 } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "TotWhImpPhA", offset: 70, length: 2, write_access: false, value: 0.0 } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "TotWhImpPhB", offset: 72, length: 2, write_access: false, value: 0.0 } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "TotWhImpPhC", offset: 74, length: 2, write_access: false, value: 0.0 } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "TotVAhExp", offset: 76, length: 2, write_access: false, value: 0.0 } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "TotVAhExpPhA", offset: 78, length: 2, write_access: false, value: 0.0 } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "TotVAhExpPhB", offset: 80, length: 2, write_access: false, value: 0.0 } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "TotVAhExpPhC", offset: 82, length: 2, write_access: false, value: 0.0 } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "TotVAhImp", offset: 84, length: 2, write_access: false, value: 0.0 } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "TotVAhImpPhA", offset: 86, length: 2, write_access: false, value: 0.0 } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "TotVAhImpPhB", offset: 88, length: 2, write_access: false, value: 0.0 } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "TotVAhImpPhC", offset: 90, length: 2, write_access: false, value: 0.0 } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "TotVArhImpQ1", offset: 92, length: 2, write_access: false, value: 0.0 } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "TotVArhImpQ1phA", offset: 94, length: 2, write_access: false, value: 0.0 } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "TotVArhImpQ1phB", offset: 96, length: 2, write_access: false, value: 0.0 } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "TotVArhImpQ1phC", offset: 98, length: 2, write_access: false, value: 0.0 } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "TotVArhImpQ2", offset: 100, length: 2, write_access: false, value: 0.0 } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "TotVArhImpQ2phA", offset: 102, length: 2, write_access: false, value: 0.0 } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "TotVArhImpQ2phB", offset: 104, length: 2, write_access: false, value: 0.0 } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "TotVArhImpQ2phC", offset: 106, length: 2, write_access: false, value: 0.0 } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "TotVArhImpQ3", offset: 108, length: 2, write_access: false, value: 0.0 } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "TotVArhImpQ3phA", offset: 110, length: 2, write_access: false, value: 0.0 } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "TotVArhImpQ3phB", offset: 112, length: 2, write_access: false, value: 0.0 } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "TotVArhImpQ3phC", offset: 114, length: 2, write_access: false, value: 0.0 } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "TotVArhImpQ4", offset: 116, length: 2, write_access: false, value: 0.0 } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "TotVArhImpQ4phB", offset: 118, length: 2, write_access: false, value: 0.0 } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "TotVArhImpQ4phC", offset: 120, length: 2, write_access: false, value: 0.0 } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "TotVArhImpQ4phC", offset: 120, length: 2, write_access: false, value: 0.0 } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "Evt", offset: 122, length: 2, write_access: false, value: Evt::default() as u32 } ));
    ret
}
