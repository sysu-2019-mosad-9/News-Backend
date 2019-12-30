/// Re-fetch raw data and persist in db
/// Keep original data if failed
pub fn try_refetch_base_raw_data() {}

/// Ensure raw data(tab names & new titlts) exists in db
pub fn ensure_base_raw_data() {
    try_refetch_base_raw_data()
}
