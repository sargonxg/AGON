#![forbid(unsafe_code)]
//! aco-time — time expression extraction + Allen-13 algebra.
//!
//! PROMPT 03 implementation. v0.1: stub API + Allen-13 calculator over absolute intervals.
//! Full multilingual regex/DFA detection lands in PROMPT 03 proper.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tacitus_contracts::AllenRelation;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TimePoint {
    Absolute(DateTime<Utc>),
    Approximate { center: DateTime<Utc>, slack_days: u32 },
    Unresolved(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeInterval {
    pub start: TimePoint,
    pub end: TimePoint,
    pub raw: String,
}

/// Allen-13 relation between two intervals. Both intervals must have absolute
/// endpoints; returns None for unresolved/approximate cases.
pub fn allen_relation(a: &TimeInterval, b: &TimeInterval) -> Option<AllenRelation> {
    let (a_start, a_end) = absolute_endpoints(a)?;
    let (b_start, b_end) = absolute_endpoints(b)?;

    Some(match (a_start.cmp(&b_start), a_end.cmp(&b_end)) {
        _ if a_end < b_start => AllenRelation::Before,
        _ if a_end == b_start => AllenRelation::Meets,
        _ if a_start > b_end => AllenRelation::After,
        _ if a_start == b_end => AllenRelation::MetBy,
        (std::cmp::Ordering::Equal, std::cmp::Ordering::Equal) => AllenRelation::Equals,
        (std::cmp::Ordering::Equal, std::cmp::Ordering::Less) => AllenRelation::Starts,
        (std::cmp::Ordering::Equal, std::cmp::Ordering::Greater) => AllenRelation::StartedBy,
        (std::cmp::Ordering::Greater, std::cmp::Ordering::Less) => AllenRelation::During,
        (std::cmp::Ordering::Less, std::cmp::Ordering::Greater) => AllenRelation::Contains,
        (std::cmp::Ordering::Greater, std::cmp::Ordering::Equal) => AllenRelation::Finishes,
        (std::cmp::Ordering::Less, std::cmp::Ordering::Equal) => AllenRelation::FinishedBy,
        (std::cmp::Ordering::Less, std::cmp::Ordering::Less) => AllenRelation::Overlaps,
        (std::cmp::Ordering::Greater, std::cmp::Ordering::Greater) => AllenRelation::OverlappedBy,
    })
}

fn absolute_endpoints(i: &TimeInterval) -> Option<(DateTime<Utc>, DateTime<Utc>)> {
    let s = match &i.start { TimePoint::Absolute(t) => *t, _ => return None };
    let e = match &i.end { TimePoint::Absolute(t) => *t, _ => return None };
    Some((s, e))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDateTime;

    fn iv(s: &str, e: &str) -> TimeInterval {
        let parse = |x: &str| -> DateTime<Utc> {
            NaiveDateTime::parse_from_str(x, "%Y-%m-%d %H:%M:%S")
                .unwrap()
                .and_utc()
        };
        TimeInterval {
            start: TimePoint::Absolute(parse(s)),
            end: TimePoint::Absolute(parse(e)),
            raw: format!("{} to {}", s, e),
        }
    }

    #[test]
    fn before_relation() {
        let a = iv("2026-01-01 00:00:00", "2026-01-02 00:00:00");
        let b = iv("2026-02-01 00:00:00", "2026-02-02 00:00:00");
        assert_eq!(allen_relation(&a, &b), Some(AllenRelation::Before));
    }

    #[test]
    fn equals_relation() {
        let a = iv("2026-01-01 00:00:00", "2026-01-02 00:00:00");
        let b = iv("2026-01-01 00:00:00", "2026-01-02 00:00:00");
        assert_eq!(allen_relation(&a, &b), Some(AllenRelation::Equals));
    }
}
