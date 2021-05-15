use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct ConfModel {
    host: String,
    user: String,
    pass: String
}

impl std::default::Default for ConfModel {
    fn default() -> Self {
        ConfModel { host: "".into(), user: "".into(), pass: "".into() }
    }
}

pub fn load() -> Result<ConfModel, ::std::io::Error> {
    Ok(confy::load("tij")?)
}

pub fn reset() -> Result<(), ::std::io::Error> {
    edit(ConfModel::default())
}

pub fn edit(conf: ConfModel) -> Result<(), ::std::io::Error> {
   Ok(confy::store("tig", conf)?)
}
