#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::fs;
use std::collections::HashMap;

use regex::Regex;

#[derive(Debug)]
struct Event {
    date_time: String,
    guard: i16,
    event: String,
    minute: i8,
    original_event: String,
}

#[derive(Debug)]
struct SleepTracker {
    guard: i16,
    is_sleeping: bool,
    sleep_minutes: HashMap<i8, i32>,
    sleep_started: i8,
    total_sleep: i32,
}

type Events = Vec<Event>;

fn main() {
    let input = fs::read_to_string("input/input.txt")
        .expect("Couldn't read input file");
    let events = parse_events(&input);
    let mut sleep_trackers: HashMap<i16, SleepTracker> = HashMap::new();

    for event in events {
        // event.guard
        let tracker = sleep_trackers.entry(event.guard).or_insert(SleepTracker {
            guard: event.guard,
            is_sleeping: false,
            sleep_minutes: HashMap::new(),
            sleep_started: 0,
            total_sleep: 0,
        });
        let was_sleeping = tracker.is_sleeping;
        let is_sleeping = event.event == "falls asleep";

        if was_sleeping && !is_sleeping {
            // let sleep_minutes = tracker.sleep_minutes.entry(event.minute).or_insert(0);
            let spans: Vec<Vec<i8>> = {
                if tracker.sleep_started > event.minute {
                    vec![vec![tracker.sleep_started, 60], vec![0, event.minute]]
                } else {
                    vec![vec![tracker.sleep_started, event.minute]]
                }
            };
            let mut minutes_slept: i8 = 0;

            for span in spans {
                // span[0]..span[1]
                for minute in span[0]..span[1] {
                    let sleep_minutes = tracker.sleep_minutes.entry(minute).or_insert(0);

                    *sleep_minutes += 1;
                }

                minutes_slept += span[1] - span[0];
            }

            tracker.is_sleeping = false;
            tracker.total_sleep += minutes_slept as i32;
        } else if is_sleeping && !was_sleeping {
            tracker.is_sleeping = true;
            tracker.sleep_started = event.minute;
        }
    }

    let mut trackers: Vec<SleepTracker> = {
        let mut tracker_list: Vec<SleepTracker> = vec![];

        for (_, tracker) in sleep_trackers {
            tracker_list.push(tracker);
        }

        tracker_list
    };

    trackers.sort_by(|a, b| a.total_sleep.cmp(&b.total_sleep));

    let sleeper = &trackers[trackers.len() - 1];
    let sleeper_minutes = &sleeper.sleep_minutes;
    let mut most_asleep: i32 = 0;
    let mut most: i32 = 0;

    for (minute, times_asleep) in sleeper_minutes {
        if times_asleep > &most {
            most = *times_asleep;
            most_asleep = *minute as i32;
        }
    }

    println!("Sleepiest guard: {}", sleeper.guard);
    println!("Most asleep: {}", most_asleep);
    println!("{}", (sleeper.guard as i32) * most_asleep);
}

fn parse_events(input: &str) -> Events {
    let mut events: Events = vec![];
    let mut guard_stack: Vec<i16> = vec![];
    let sorted_input: Vec<&str> = {
        let mut collected_lines: Vec<&str> = input.lines().collect();

        collected_lines.sort();

        collected_lines
    };

    for line in sorted_input {
        let last_guard = guard_stack.pop();
        let event = create_event(&line, last_guard);

        guard_stack.push(event.guard);
        events.push(event);
    }

    events.sort_by(|a, b| a.date_time.cmp(&b.date_time));

    events
}

fn create_event(entry: &str, last_guard: Option<i16>) -> Event {
    let parts: Vec<&str> = entry.split("]").collect();
    let date_time = parts[0].replace("[", "").trim().to_string();
    let minute = (*date_time)[(date_time.len() - 2)..].parse::<i8>().unwrap();
    let original_event = parts[1].trim().to_string();
    let mut event = (*original_event).to_string();
    let mut guard: Option<i16> = last_guard;

    lazy_static! {
        static ref GUARD_REGEX: Regex = Regex::new(r"^Guard #(?P<guard>\d+)").unwrap();
    }

    if let Some(guard_matches) = GUARD_REGEX.captures(&original_event) {
        let guard_match = guard_matches.name("guard").unwrap();
        let guard_str = &original_event[guard_match.start()..guard_match.end()];

        guard = Some(guard_str.parse::<i16>().expect("Couldn't parse guard"));
        event = original_event[guard_match.end()..].trim().to_string();
    }

    Event {
        date_time,
        guard: guard.unwrap(),
        event,
        minute,
        original_event,
    }
}
