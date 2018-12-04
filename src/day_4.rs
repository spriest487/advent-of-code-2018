use {
    std::{
        fmt,
        collections::HashMap,
    },
    chrono::{DateTime, TimeZone, Timelike, Utc}
};

#[derive(Debug)]
enum SleepLogEvent {
    NewGuard(usize),
    FallsAsleep,
    WakesUp,
}

#[derive(Debug)]
struct SleepLog {
    timestamp: DateTime<Utc>,
    event: SleepLogEvent,
}

const DATE_FORMAT: &str = "%Y-%m-%d %H:%M";

impl SleepLog {
    fn parse(s: &str) -> Self {
        let timestamp = Utc.datetime_from_str(&s[1..17], DATE_FORMAT).unwrap();

        let event_desc = &s[19..];
        let event = match event_desc {
            "falls asleep" => SleepLogEvent::FallsAsleep,
            "wakes up" => SleepLogEvent::WakesUp,
            _ => {
                let id_str: String = event_desc[7..]
                    .chars()
                    .take_while(|c| c.is_numeric())
                    .collect();

                let id: usize = id_str.parse().unwrap();

                SleepLogEvent::NewGuard(id)
            }
        };

        Self { timestamp, event }
    }
}

impl fmt::Display for SleepLog {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let timestamp = self.timestamp.format(DATE_FORMAT);
        write!(f, "[{}] ", timestamp)?;

        match &self.event {
            SleepLogEvent::WakesUp => write!(f, "wakes up"),
            SleepLogEvent::FallsAsleep => write!(f, "falls asleep"),
            SleepLogEvent::NewGuard(id) => write!(f, "Guard #{} begins shift", id),
        }
    }
}

struct Shift {
    guard_id: usize,

    last_asleep: Option<usize>, // in minutes past midnight
    minutes_asleep: [bool; 60], // which minutes past midnight was this guard asleep
}

fn main() {
    let input = include_str!("day_4.txt");

    let mut log: Vec<_> = input.lines().map(SleepLog::parse).collect();
    log.sort_by_key(|entry| entry.timestamp);

    let shifts = log.iter().fold(Vec::new(), |mut shifts, entry| {
        match &entry.event {
            SleepLogEvent::NewGuard(id) => {
                shifts.push(Shift {
                    guard_id: *id,
                    minutes_asleep: [false; 60],
                    last_asleep: None,
                });
            }

            SleepLogEvent::FallsAsleep => {
                let current_shift = shifts.last_mut().unwrap();

                assert!(
                    current_shift.last_asleep.is_none(),
                    "guard {} should not already be asleep when falling asleep @ {}",
                    current_shift.guard_id,
                    entry.timestamp
                );
                current_shift.last_asleep = Some(entry.timestamp.minute() as usize);
            }

            SleepLogEvent::WakesUp => {
                let current_shift = shifts.last_mut().unwrap();

                let last_asleep = current_shift.last_asleep.unwrap();
                let entry_time = entry.timestamp.minute() as usize;
                for minute in last_asleep..entry_time {
                    current_shift.minutes_asleep[minute] = true;
                }

                current_shift.last_asleep = None;
            }
        };

        shifts
    });

    let mut guard_profiles = HashMap::new();
    for shift in shifts {
        let profile = guard_profiles.entry(shift.guard_id)
            .or_insert_with(|| [0usize; 60]);

        for (minute, asleep) in shift.minutes_asleep.iter().enumerate() {
            if *asleep {
                profile[minute] += 1;
            }
        }
    }

    let sleepiest_guard = guard_profiles.iter()
        .map(|(id, profile)| (id, profile.iter().cloned().sum::<usize>()))
        .max_by_key(|(_id, minutes)| *minutes)
        .map(|(id, _minutes)| id)
        .unwrap();
    let sleepiest_minute = guard_profiles.get(sleepiest_guard).unwrap().iter()
        .enumerate()
        .max_by_key(|(_minute, times_asleep)| **times_asleep)
        .map(|(minute, _)| minute)
        .unwrap();

    println!("sleepiest guard: {}, at minute {} (value: {})", sleepiest_guard,
        sleepiest_minute, sleepiest_guard * sleepiest_minute);

    let (most_freq_guard_id, most_freq_minute) = guard_profiles.iter()
        .map(|(guard_id, nap_minutes)| {
            let (sleepiest_minute, times_asleep) = nap_minutes.iter()
                .enumerate()
                .max_by_key(|(_minute, times)| **times)
                .unwrap();

            (guard_id, sleepiest_minute, times_asleep)
        })
        .max_by_key(|(_guard_id, _sleepiest_minute, times_asleep)| **times_asleep)
        .map(|(guard_id, sleepiest_minute, _)| (guard_id, sleepiest_minute))
        .unwrap();

    println!("most frequently asleep guard at minute {}: {} (value: {})", most_freq_minute,
        most_freq_guard_id, most_freq_minute * most_freq_guard_id);
}
