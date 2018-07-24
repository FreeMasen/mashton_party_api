use postgres::{Conection, TlsMode};
const CONFIG_STR: &str = include_str!("config.toml");
pub fn get_all_parties() -> Vec<Party> {
    unimplemented!()
}


pub struct Party {
    pub id: usize,
    pub name: String,
    /// ISO Date String
    pub date: String,
    pub place: Place,
    pub snippet: String,
    pub description: String,
    pub image_path: String,
    pub rsvp_list: Option<Vec<Rsvp>>,
    pub rsvp_item: Option<String>,
}

pub struct Place {
    pub name: String,
    pub address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub zip: Option<String>,
    pub description: Option<String>,
}

pub struct Rsvp {
    id: usize,
    name: String,
    attending: bool,
    bringing: Option<String>,
    message: Option<String>,
}