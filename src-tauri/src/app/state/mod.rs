use std::{sync::{Arc, self}, collections::HashMap};
use tokio::sync::Mutex;

use sea_orm::DbConn;

use super::models::slide::View;

pub struct DB {
    pub conn: DbConn,
}

pub struct BibleDB {
    pub conns: Mutex<HashMap<String, Arc<Mutex<DbConn>>>>
}

pub struct Live {
    pub view: sync::Mutex<Option<View>>
}
