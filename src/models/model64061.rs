use super::*;

const REG_NAMES: [&str; 127] = [
    "REG001", "REG002", "REG003", "REG004", "REG005", "REG006", "REG007", "REG008", "REG009", "REG010",
    "REG011", "REG012", "REG013", "REG014", "REG015", "REG016", "REG017", "REG018", "REG019", "REG020",
    "REG021", "REG022", "REG023", "REG024", "REG025", "REG026", "REG027", "REG028", "REG029", "REG030",
    "REG031", "REG032", "REG033", "REG034", "REG035", "REG036", "REG037", "REG038", "REG039", "REG040",
    "REG041", "REG042", "REG043", "REG044", "REG045", "REG046", "REG047", "REG048", "REG049", "REG050",
    "REG051", "REG052", "REG053", "REG054", "REG055", "REG056", "REG057", "REG058", "REG059", "REG060",
    "REG061", "REG062", "REG063", "REG064", "REG065", "REG066", "REG067", "REG068", "REG069", "REG070",
    "REG071", "REG072", "REG073", "REG074", "REG075", "REG076", "REG077", "REG078", "REG079", "REG080",
    "REG081", "REG082", "REG083", "REG084", "REG085", "REG086", "REG087", "REG088", "REG089", "REG090",
    "REG091", "REG092", "REG093", "REG094", "REG095", "REG096", "REG097", "REG098", "REG099", "REG100",
    "REG101", "REG102", "REG103", "REG104", "REG105", "REG106", "REG107", "REG108", "REG109", "REG110",
    "REG111", "REG112", "REG113", "ALM01", "ALM02", "ALM03", "ALM04", "REG118", "REG119", "REG120",
    "REG121", "REG122", "REG123", "REG124", "REG125", "REG126", "REG127"
];

pub fn model64061() -> Model {
    let mut ret = Model {
        start_addr: 0,
        end_addr: 0,
        model_number: 64061,
        qtd: 127,
        update: false,
        data: Vec::new(),
    };

    for (i, regname) in REG_NAMES.iter().enumerate() {
        ret.data.push(DataTypes::SunspecU16(Point { name: regname, offset: 2 + (i as u16), length: 1, write_access: false, value: 0xFFFF }));
    }

    ret
}