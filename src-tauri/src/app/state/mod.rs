use std::{
    collections::HashMap,
    sync::{self, Arc},
};
use tokio::sync::Mutex;

use sea_orm::DbConn;

use super::models::slide::View;

pub struct DB {
    pub conn: DbConn,
}

pub struct BibleDB {
    pub conns: Mutex<HashMap<String, Arc<Mutex<DbConn>>>>,
}

impl BibleDB {
    pub fn new() -> Self {
        Self {
            conns: Mutex::new(HashMap::new()),
        }
    }
}

pub struct Live {
    pub view: sync::Mutex<Option<View>>,
}

impl Live {
    pub fn new() -> Self {
        Self {
            view: sync::Mutex::new(None),
        }
    }
}

pub struct SongDB {
    pub conns: Mutex<HashMap<String, Arc<Mutex<DbConn>>>>,
}

impl SongDB {
    pub fn new() -> Self {
        Self {
            conns: Mutex::new(HashMap::new()),
        }
    }
}
