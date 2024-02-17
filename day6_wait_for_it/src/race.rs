#[derive(Debug)]
pub struct Race {
    race_time: u64,
    record_dist: u64,
}

impl Race {
    // Find the different solutions about how long to push the button to beat the record.
    pub fn new(race_time: u64, record_dist: u64) -> Self {
        Self {
            race_time,
            record_dist,
        }
    }
    pub fn find_solutions_to_beat_record(&self) -> Vec<u64> {
        let (lower_bound, upper_bound) = self.find_solution_bounds_to_beat_record();
        (lower_bound..upper_bound).collect()
    }

    pub fn find_solution_bounds_to_beat_record(&self) -> (u64, u64) {
        let (lower_bound, upper_bound) = self.find_bounds_to_beat_record();
        (lower_bound.ceil() as u64, upper_bound.ceil() as u64)
    }
    // find the roots
    fn find_bounds_to_beat_record(&self) -> (f64, f64) {
        // This is a 2nd degree equation to solve
        // speed = Tpush
        // distance = (Trace - Tpush)*speed
        // distance = (Trace - Tpush)*Tpush
        // distance = -Tpush^2 + Trace*Tpush
        // The bounds of the solution space are the root of the equation
        // -Tpush^2 + Trace*Tpush - Drecord
        let t_race = self.race_time as f64;
        let d_record = self.record_dist as f64;
        let discriminant = t_race * t_race - 4f64 * -1f64 * -d_record; //b^2 - 4ac
        let upper_root = (-t_race - discriminant.sqrt()) / 2f64 * -1f64;
        let lower_root = (-t_race + discriminant.sqrt()) / 2f64 * -1f64;
        (lower_root, upper_root)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_solutions() {
        let race = Race::new(7, 9);
        let expected: Vec<u64> = vec![2, 3, 4, 5];
        assert_eq!(race.find_solutions_to_beat_record(), expected);
    }
}
