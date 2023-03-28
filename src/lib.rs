pub mod models;
pub mod utils;
pub mod types;


#[cfg(test)]
mod tests {
    use crate::models::{SunspecModels, Models, Model, SunspecID};
    use crate::types::{SunspecTypes, DataTypes};

    #[test]
    fn convert_model() {
        let mut new_model = Models::new();

        new_model.models.push(Model::new(1));
        new_model.models.push(Model::new(213));
        new_model.models.push(Model::new(65535));
        
        new_model.models[0].update_data("Mn", &DataTypes::new_string("Manufactor teste"));
        new_model.models[0].update_data("Md", &DataTypes::new_string("Model"));
        new_model.models[0].update_data("Ver", &DataTypes::new_string("ABCD1234"));
        new_model.models[0].update_data("Opt", &DataTypes::new_string("Options"));
    
        new_model.models[1].update_data("A", &DataTypes::new_f32(12.5));
        new_model.models[1].update_data("Hz", &DataTypes::new_f32(60.05));
        new_model.models[1].update_data("PF", &DataTypes::new_f32(0.92));
    
        new_model.compute_addr();

        let mut registers: Vec<u16> = new_model.id.into();
        for model in new_model.models.iter() {
            let model_tmp = model.clone();
            registers.extend(Vec::<u16>::from(model_tmp));
        }

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
