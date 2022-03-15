use jfs::Store;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub document: String,
    pub email: String,
    pub name: String,
    key: String,
}

#[derive(Serialize, Deserialize)]
struct Prefs {
    api_key: String,
    user: User,
}

pub fn db() -> Store {
    match dirs::config_dir() {
        Some(path) => {
            let cfg_path = [path.display().to_string(), "prometeo.json".to_string()].join("/");
            let mut cfg = jfs::Config::default();
            cfg.single = true;

            let res = jfs::Store::new_with_cfg(cfg_path, cfg);

            match res {
                Ok(db) => db,
                Err(e) => panic!("Error: {:?}", e),
            }
        }
        None => panic!("Impossible to get your config dir!"),
    }
}

pub fn get_api_key() -> Option<String> {
    match db().get("API_KEY") {
        Ok(api_key) => api_key,
        Err(_) => None,
    }
}

pub fn get_user() -> Option<User> {
    match db().get::<User>("user") {
        Ok(user) => Some(user),
        Err(_) => None,
    }
}

pub fn set_pref(key: String, value: String) {
    match db().save_with_id(&value, &key) {
        Err(e) => panic!("Error {:?}", e),
        Ok(_) => (),
    }
}
