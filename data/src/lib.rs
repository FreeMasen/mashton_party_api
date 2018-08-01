extern crate postgres;
extern crate chrono;
extern crate toml;
extern crate uuid;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
use postgres::{Connection, TlsMode};
use chrono::{DateTime, FixedOffset};
use toml::from_str;
use uuid::Uuid;
lazy_static! {
    static ref CONFIG: DbConfig = from_str(include_str!("../db_config.toml")).expect("Unable to deserialize db_config.toml");
}
pub fn get_all_parties() -> Option<Vec<Party>> {
    let mut ret = vec![];
    let c = get_conn()?;
    for row in &c.query("SELECT id, name, date, snippet, description, image_path, rsvp_item
                        FROM public.party", &[]).unwrap() {
        let id: i32 = row.get(0);
        let name: String = row.get(1);
        let timestamp: DateTime<FixedOffset>  = row.get(2);
        let snippet: String = row.get(3);
        let description: String = row.get(4);
        let image_path: String = row.get(5);
        let rsvp_item: Option<String>  = row.get(6);
        let place = place_for(id, &c)?;
        let rsvp_list = rsvps_for(id, &c);
        ret.push(Party {
            id,
            name,
            date: timestamp.to_rfc3339(),
            snippet,
            description,
            rsvp_item,
            place,
            rsvp_list: rsvp_list,
            image_path,
        })
    }
    Some(ret)
}

fn place_for(id: i32, c: &Connection) -> Option<Place> {
    match c.query("SELECT id, address, city, state, zip, description, name
                                    FROM public.place
                                    WHERE party_id = $1
                                    LIMIT 1", &[&id]) {
        Ok(rows) => {
            let row = rows.iter().next()?;
            Some(Place {
                    id: row.get(0),
                    address: row.get(1),
                    city: row.get(2),
                    state: row.get(3),
                    zip: row.get(4),
                    description: row.get(5),
                    name: row.get(6),
                })
        },
        Err(e) => {
            error!("ERROR unable to get place for party_id: {}\n{:?}", id, e);
            None
        }
    }
}

fn rsvps_for(id: i32, c: &Connection) -> Option<Vec<Rsvp>> {
    match c.query("SELECT id, name, attending, bringing, message, user_id
                                    FROM public.rsvp
                                    where party_id = $1", &[&id]) {
        Ok(rows) => {
            Some(rows.iter().map(|r| {
                Rsvp {
                    id: r.get(0),
                    name: r.get(1),
                    attending: r.get(2),
                    bringing: r.get(3),
                    message: r.get(4),
                    user_id: r.get(5),
                }
            }).collect())
        },
        Err(e) => {
            error!("ERROR unable to get rsvps for party_id: {}\n{:?}", id, e);
            None
        }
    }
}

pub fn get_rsvps_for(token: &Uuid) -> Option<Vec<i32>> {
    let c = get_conn()?;
    match c.query("SELECT id
                  FROM user_rsvps
                  WHERE token = $1", &[&token]) {
        Ok(rows) => {
            Some(rows.iter().map(|r| r.get(0)).collect())
        },
        Err(e) => {
            error!("ERROR unable to get rsvp ids for user {}\n{:?}", token, e);
            None
        }
    }
}

pub fn get_user_for(invite_id: &Uuid) -> Option<User> {
    let c = get_conn()?;
    match c.query("SELECT u.id, u.name, u.token, u.email
                    FROM public.user AS u
                    JOIN public.invite AS i
                        on i.user_id = u.id
                    WHERE i.guid = $1
                    LIMIT 1", &[&invite_id]) {
        Ok(rows) => {
            let row = rows.iter().next()?;
            let id = row.get(0);
            Some(User {
                id,
                name: row.get(1),
                token: row.get(2),
                invited_to: get_user_invite_ids(id, &c)?,
                email: row.get(3),
            })
        },
        Err(e) => {
            error!(target: "db_events", "ERROR unable to get user for invite_id: {}\n{:?}", invite_id, e);
            None
        }
    }
}

fn get_user_invite_ids(user_id: i32, c: &Connection) -> Option<Vec<i32>> {
    debug!(target: "db_events", "get_user_invite_ids {}", user_id);
    match c.query("SELECT party_id
                    FROM invite
                    WHERE user_id = $1", &[&user_id]) {
        Ok(rows) => {
            debug!(target: "db_events", "got rows");
            let ret = rows.iter().map(|r| r.get(0)).collect();
            debug!(target: "db_events", "ids: {:?}", ret);
            Some(ret)
        },
        Err(e) => {
            error!(target: "db_events", "ERROR unable to get user invite ids for {}\n{:?}", user_id, e);
            None
        }
    }
}

pub fn update_rsvp(rsvp: &Rsvp) -> Option<Vec<Party>> {
    let c = get_conn()?;
    match c.execute("UPDATE rsvp
                    SET name = $1,
                    bringing = $2,
                    message = $3,
                    attending = $4
                    where id = $5",
                    &[&rsvp.name, &rsvp.bringing,
                    &rsvp.message, &rsvp.attending,
                    &rsvp.id]) {
        Ok(_) => get_all_parties(),
        Err(e) => {
            error!(target: "db_events", "Unable to update rsvp: {}\n{:?}", rsvp.id, e);
            None
        }
    }
}

pub fn get_all_users() -> Option<Vec<User>> {
    let c = get_conn()?;
    match c.query("SELECT id, name, token, email
                    FROM public.user", &[]) {
        Ok(rows) => {
            Some(rows.iter().filter_map(|u| {
                let id = u.get(0);
                let invited_to = get_user_invite_ids(id, &c)?;
                Some(User {
                    id,
                    name: u.get(1),
                    token: u.get(2),
                    invited_to,
                    email: u.get(3)
                })
            }).collect())
        },
        Err(e) => {
            error!(target: "db_events", "Unable to get users\n{:?}", e);
            None
        }
    }
}

pub fn add_party(party: Party) -> Option<()> {
    let c = get_conn()?;
    match c.execute("INSERT INTO party (name, date, snippet, description, image_path, rsvp_item)
                    VALUES ($1, $2, $3, $4, $5, $6)",
                    &[&party.name, &party.date, &party.snippet, &party.description, &party.image_path, &party.rsvp_item]) {
        Ok(_) => {
            if let Some(ref rsvps) = party.rsvp_list {
                for rsvp in rsvps {
                    add_rsvp(party.id, rsvp, &c)?;
                }
            }
            Some(())
        },
        Err(e) => {
            error!(target: "db_events", "Unable to add new party\n{:?}", e);
            None
        }
    }
}

pub fn add_rsvp(party_id: i32, rsvp: &Rsvp, c: &Connection) -> Option<()> {
    match c.execute("INSERT INTO invite (user_id, party_id)
                    VALUES ($1, $1)", &[&rsvp.user_id, &party_id]) {
        Ok(_) => (),
        Err(e) => {
            error!(target: "db_events", "Unable to add new invite for party {}\n{:?}", party_id, e);
            return None;
        }
    }
    match c.execute("INSERT INTO rsvp (name, bringing, message, party_id, user_id, attending)
                    VALUES ($1, $2, $3, $4, $5, false)",
                    &[&rsvp.name, &rsvp.bringing, &rsvp.message, &party_id, &rsvp.user_id]) {
        Ok(_) => Some(()),
        Err(e) => {
            error!(target: "db_events", "Unable to add new Rsvp for party: {}\n{:?}", party_id, e);
            None
        }
    }
}

pub fn update_party(p: Party) -> Option<()> {
    let c = get_conn()?;
    match c.execute("UPDATE party
                    SET name = $1,
                    date = $2,
                    snippet = $3,
                    description = $4,
                    image_path = $5,
                    rsvp_item = $6",
                    &[&p.name, &p.date, &p.snippet, &p.description, &p.image_path, &p.rsvp_item]) {
        Ok(_) => Some(()),
        Err(e) => {
            error!(target: "db_events", "Unable to update party {}\n{:?}", p.id, e);
            None
        }
    }
}

fn get_conn() -> Option<Connection> {
    match Connection::connect(format!("postgres://{}:{}@localhost:5432/mashton.party", CONFIG.user, CONFIG.password).as_str(), TlsMode::None) {
        Ok(c) => Some(c),
        Err(e) => {
            error!("ERROR unable to connect to postgres\n{:?}", e);
            None
        },
    }
}

#[derive(Deserialize)]
struct DbConfig {
    pub user: String,
    pub password: String,
}
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Party {
    pub id: i32,
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
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Place {
    pub id: i32,
    pub name: String,
    pub address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub zip: Option<String>,
    pub description: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Rsvp {
    pub id: i32,
    pub name: String,
    pub attending: bool,
    pub bringing: Option<String>,
    pub message: Option<String>,
    pub user_id: i32,
}
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: i32,
    pub name: String,
    pub token: Uuid,
    pub invited_to: Vec<i32>,
    pub email: String,
}