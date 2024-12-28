use crate::db::connect::DB;

#[derive(Clone)]
pub struct AppState {
    pub db:DB,
}