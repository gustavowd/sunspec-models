use super::*;

const REG_NAMES: [&str; 19] = [
    "REG001", "REG002", "REG003", "REG004", "REG005", "REG006", "REG007", "REG008", "REG009", "REG010",
    "REG011", "REG012", "REG013", "REG014", "REG015", "REG016", "REG017", "REG018", "REG019"
];

pub fn model64069() -> Model {
    let mut ret = Model {
        start_addr: 0,
        end_addr: 0,
        model_number: 64069,
        qtd: 19,
        update: false,
        data: Vec::new(),
    };

    for (i, regname) in REG_NAMES.iter().enumerate() {
        ret.data.push(DataTypes::SunspecU16(Point { name: regname, offset: 2 + (i as u16), length: 1, write_access: false, value: 0xFFFF }));
    }

    ret
}