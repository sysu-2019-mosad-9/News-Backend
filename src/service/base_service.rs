use crate::db::base_repository::get_latest_craw_time;

/// Re-fetch raw data and persist in db
/// Keep original data if failed
pub fn try_refetch_base_raw_data() {}

/// Ensure raw data(tab names & new titlts) exists in db
pub fn ensure_base_raw_data() {
    let latest = get_latest_craw_time().unwrap();
    if latest.elapsed().unwrap().as_secs() < 1800 {
        // last crawling was within 30 minutes
        return;
    }
    try_refetch_base_raw_data()
}
