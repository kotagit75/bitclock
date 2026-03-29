use crate::model::proof::ProofPool;

fn get_clamped<T: Clone>(arr: Vec<T>, index: usize) -> T {
    arr[std::cmp::min_by(index, arr.len() - 1, |a, b| a.cmp(b))].clone()
}
impl ProofPool {
    fn calc_actual_time(&self, end_time: i64) -> Option<i64> {
        let sorted_times = self.sort_pool_to_time();
        let lastest_index_before_end_time =
            sorted_times.iter().position(|time| *time < end_time)?;
        let start_time = get_clamped(sorted_times, lastest_index_before_end_time);
        Some(end_time - start_time)
    }
    pub fn calc_difficulty(&self, end_time: i64) -> usize {
        let base = 3;
        let target_time = 1000 * 60 * 10; // targetTime should be bigger
        let pool = self.sort_pool();
        let Some(old_proof) = pool.iter().find(|proof| proof.time < end_time) else {
            return base;
        };
        let Some(actual_time) = self.calc_actual_time(end_time) else {
            return old_proof.difficulty;
        };
        let rate = (target_time as f64 / actual_time as f64).clamp(0.2, 1.8);
        (old_proof.difficulty as f64 * rate).round() as usize
    }
}
