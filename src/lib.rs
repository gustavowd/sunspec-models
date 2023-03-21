pub mod models;
pub mod utils;
pub mod types;


#[cfg(test)]
mod tests {
    use crate::models::{self, Models, SunspecID};

    #[test]
    fn convert_model_213() {
        struct Models {
            id: models::SunspecID,
            model1: models::Model1,
            model213: models::models200::Model213,
            model_end: models::ModelEnd,
        }
        impl Models {
            fn new () -> Models {
                Models {
                    id: models::SunspecID::new(),
                    model1: models::Model1::new(),
                    model213: models::models200::Model213::new(),
                    model_end: models::ModelEnd::new()
                }
            }
        }

        let mut new_model = Models::new();

        new_model.model1.manufacturer.value = "Manufactor".to_string();
        new_model.model1.model.value = "Model".to_string();
        new_model.model1.serial_number.value = "ABCD1234".to_string();
        new_model.model1.options.value = "Options".to_string();
    
        new_model.model213.a = 12.5;
        new_model.model213.hz = 60.05;
        new_model.model213.pf = 0.92;
        let mut registers: Vec<u16> = new_model.id.into();
        registers.extend(Vec::<u16>::from(new_model.model1));
        registers.extend(Vec::<u16>::from(new_model.model213));
        registers.extend(Vec::<u16>::from(new_model.model_end));
        println!("{:?}", registers);
        assert_eq!(registers[72], 16712);
    }

    #[test]
    fn convert_sunspec_id() {
        let id_vec: Vec<u16> = vec![21365, 28243];

        let id: SunspecID = id_vec.into();

        println!("{:?}", id);
        assert_eq!(id.id[0], 'S' as u8);
        assert_eq!(id.id[1], 'u' as u8);
        assert_eq!(id.id[2], 'n' as u8);
        assert_eq!(id.id[3], 'S' as u8);
    }
}
