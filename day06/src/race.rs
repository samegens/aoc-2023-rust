pub struct Race {
    milliseconds: i64,
    record: i64
}

impl Race {
    pub fn new(milliseconds: i64, record: i64) -> Race {
        Race { milliseconds, record }
    }

    pub fn get_nr_options_that_beat_record(&self) -> i64 {
        (1..self.milliseconds)
            .map(|i| self.get_distance_when_waiting(i))
            .filter(|n| *n > self.record)
            .count() as i64
    }

    fn get_distance_when_waiting(&self, milliseconds: i64) -> i64 {
        let speed = milliseconds;
        (self.milliseconds - milliseconds) * speed
    }
}
