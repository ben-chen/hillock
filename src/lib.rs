use std::time::{Duration, Instant};
use std::collections::HashMap;

/// A simple stopwatch to profile execution time
pub struct StopWatch {
    last: Instant,
    data: HashMap<String, Duration>,
    keys: Vec<String>,
    enabled: bool,
    prints: bool,
}

impl StopWatch {
    /// Create a new stopwatch
    pub fn new(enabled: bool, prints: bool) -> Self {
        StopWatch {
            last: Instant::now(),
            data: HashMap::new(),
            keys: Vec::new(),
            enabled,
            prints,
        }
    }

    /// Reset the stopwatch
    pub fn reset(&mut self) {
        if !self.enabled {
            return;
        }
        self.data.clear();
        self.last = Instant::now();
    }

    /// Record the current time with a message
    pub fn tick(&mut self, msg: &str) {
        if self.prints {
            println!("{}", msg);
        }
        if !self.enabled {
            return;
        }
        let now = Instant::now();
        let elapsed = now.duration_since(self.last);
        self.last = now;
        if !self.data.contains_key(msg) {
            self.keys.push(msg.to_string());
        }
        let counter = self
            .data
            .entry(msg.to_string())
            .or_insert_with(|| Duration::new(0, 0));
        *counter += elapsed;
    }

    /// Print the breakdown of the time spent on each tick
    pub fn breakdown(&self, n: u32, unit: &str) {
        if !self.enabled {
            return;
        }
        let total_duration: Duration = self.data.values().sum();
        let total_ms = total_duration.as_millis();
        let mut top = format!("%%%%%%%%%%%%% Breakdown of {} {} %%%%%%%%%%%%%\n", n, unit);
        let mut breakdown_message = top.clone();
        for k in &self.keys {
            let duration = self.data.get(k).unwrap();
            let ms = duration.as_millis();
            let percentage = ms as f64 / total_ms as f64 * 100.0;
            breakdown_message.push_str(&StopWatch::highlight_if(
                &format!("> {} {}ms {:.1}%\n", k, ms, percentage),
                percentage > 10.0,
            ));
        }
        breakdown_message.push_str(&format!(
            "finished in {}ms ({:.2} {} per second)\n",
            total_ms,
            n as f64 * 1000.0 / total_ms as f64,
            unit
        ));
        top = StopWatch::highlight_if(&top, true);
        breakdown_message.push_str(&StopWatch::highlight_if(
            &"%".repeat(std::cmp::max(top.len() - 10, 26)),
            false,
        ));
        println!("{}", breakdown_message);
    }

    fn highlight_if(s: &str, b: bool) -> String {
        if b {
            format!("\x1B[93m{}\x1B[0m", s)
        } else {
            s.to_string()
        }
    }
}
