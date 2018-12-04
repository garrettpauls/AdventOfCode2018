extern crate aoc_common;
extern crate regex;

use aoc_common::advent;

fn main() {
    advent(&parse_input, &part_one, &part_two);
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Event {
    BeginShift(u64),
    Sleep,
    Wake,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Input {
    instant: u64,
    minute: u64,
    event: Event,
}

fn parse_input(input: &str) -> Result<Vec<Input>, String> {
    use regex::{Regex, Captures};

    let re = Regex::new(r"(?x)
#[1518-11-01 00:00] Guard #10 begins shift
#[1518-11-01 00:05] falls asleep
#[1518-11-01 00:25] wakes up
^
\[(?P<year>\d{4})-(?P<month>\d{2})-(?P<day>\d{2})\s+(?P<hour>\d{2}):(?P<minute>\d{2})\]\s+
(
    (Guard\s\#(?P<id>\d+)\sbegins\sshift)
  | (?P<sleep>falls\sasleep)
  | (?P<wake>wakes\sup)
)
$
").expect("Built in regex was incorrect");

    let results: Result<Vec<_>, _> = input.lines()
        .map(|line| re.captures(&line)
            .ok_or(format!("Regex failed to match line: {}", &line))
            .and_then(&parse))
        .collect();

    return results.map(|mut v| {
        v.sort();
        v
    });

    fn parse(caps: Captures) -> Result<Input, String> {
        let year = u64_from_str(&caps["year"])?;
        let month = u64_from_str(&caps["month"])?;
        let day = u64_from_str(&caps["day"])?;
        let hour = u64_from_str(&caps["hour"])?;
        let minute = u64_from_str(&caps["minute"])?;
        let instant = minute + (hour * 100) + (day * 10000) + (month * 1000000) + (year * 100000000);

        let event = if let Some(cap) = caps.name("id") {
            let id = u64_from_str(cap.as_str())?;
            Event::BeginShift(id)
        } else if let Some(_) = caps.name("sleep") {
            Event::Sleep
        } else if let Some(_) = caps.name("wake") {
            Event::Wake
        } else {
            return Err("No action".to_owned());
        };

        Ok(Input {
            instant,
            minute,
            event,
        })
    }

    fn u64_from_str(value: &str) -> Result<u64, String> {
        use std::str::FromStr;
        u64::from_str(&value).map_err(|e| e.to_string())
    }
}

fn part_one(input: &Vec<Input>) -> Result<String, String> {
    use std::collections::{HashMap, HashSet};

    let mut sleeps = HashMap::new();
    let mut minutes = HashMap::new();
    let mut id = 0;
    let mut start_minute = 0;

    for inp in input {
        match inp.event {
            Event::BeginShift(gid) => id = gid,
            Event::Sleep => start_minute = inp.minute,
            Event::Wake => {
                let duration = inp.minute - start_minute;
                sleeps.entry(id).and_modify(|v| *v += duration).or_insert(duration);

                for m in start_minute..inp.minute {
                    minutes.entry(m)
                        .and_modify(|s: &mut HashSet<_>| { s.insert(id); })
                        .or_insert_with(|| {
                            let mut s = HashSet::new();
                            s.insert(id);
                            s
                        });
                }
            }
        }
    }

    let target_id = *sleeps.iter().max_by_key(|(_, v)| *v)
        .ok_or("No max found".to_owned())
        .map(|(id, _)| id)?;

    let mut minute_counts = HashMap::new();
    let mut i = 0;
    while i < input.len() {
        let inp = &input[i];
        match inp.event {
            Event::BeginShift(id) => {
                if id != target_id {
                    while let Some(next) = input.get(i + 1) {
                        match next.event {
                            Event::BeginShift(_) => break,
                            _ => { i += 1; }
                        }
                    }
                }
            }
            Event::Sleep => {
                if let Some(wake) = input.get(i + 1) {
                    i += 1;
                    for min in inp.minute..wake.minute {
                        minute_counts.entry(min).and_modify(|v| *v += 1).or_insert(1);
                    }
                }
            }
            Event::Wake => {}
        }
        i += 1;
    }

    let minute = minute_counts.iter()
        .max_by_key(|(_, v)| *v).map(|(k, _)| *k)
        .ok_or(format!("No max minute for guard {}", target_id))?;

    Ok(format!("id {} * min {} = {}", target_id, minute, target_id * minute))
}

fn part_two(input: &Vec<Input>) -> Result<String, String> {
    use std::collections::HashMap;

    // minute -> (id -> amount)
    let mut minute_to_id_amount = HashMap::new();

    let mut id = 0;
    let mut start_minute = 0;

    for inp in input {
        match inp.event {
            Event::BeginShift(gid) => id = gid,
            Event::Sleep => start_minute = inp.minute,
            Event::Wake => {
                for m in start_minute..inp.minute {
                    minute_to_id_amount.entry(m)
                        .and_modify(|id_to_amount: &mut HashMap<_, _>| {
                            id_to_amount.entry(id)
                                .and_modify(|v| *v += 1)
                                .or_insert(1);
                        })
                        .or_insert_with(|| {
                            let mut id_to_amount = HashMap::new();
                            id_to_amount.insert(id, 1);
                            id_to_amount
                        });
                }
            }
        }
    }

    let result =
        minute_to_id_amount.iter()
            // (min, (id, count))
            .map(|(min, id_to_amount)| id_to_amount.iter()
                .max_by_key(|(_, ct)| *ct)
                .map_or((0, 0, 0), |(id, ct)| (*min, *id, *ct)) // (u64, i32)
            )
            .max_by_key(|(_, _, count)| *count);

    if let Some((min, id, _)) = result {
        Ok(format!("id {} * min {} = {}", id, min, id * min))
    } else {
        Err("No max found".to_owned())
    }
}