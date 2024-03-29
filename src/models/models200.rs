#[repr(u32)]
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
