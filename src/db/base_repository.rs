use crate::schema::base_news;
use diesel::prelude::*;

use crate::db::connection;

use std::time::SystemTime;

pub fn get_latest_craw_time() -> Result<SystemTime, &'static str> {
    Ok(base_news::table
        .select(base_news::craw_time)
        .order(base_news::craw_time.desc())
        .first::<SystemTime>(&connection())
        .unwrap())
}
