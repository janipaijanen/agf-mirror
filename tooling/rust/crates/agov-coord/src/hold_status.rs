//! Pure temporal evaluation for pinned publication-hold evidence.
//! This module never turns a declaration into authority or permission to delay.
use time::OffsetDateTime;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TemporalState {
    Future,
    Active,
    Ended,
    Expired,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Interval {
    pub start: OffsetDateTime,
    pub effective_end: OffsetDateTime,
    pub state: TemporalState,
}

/// Inputs must already be schema-valid. An end preceding its named start is a
/// semantic error, including when that end lies outside the current time window.
/// Multiple ends are idempotent; the earliest one controls the half-open interval.
pub fn evaluate_interval(
    start: OffsetDateTime,
    expiry: OffsetDateTime,
    ends: &[OffsetDateTime],
    now: OffsetDateTime,
) -> Result<Interval, &'static str> {
    if expiry <= start || expiry - start > time::Duration::hours(24) {
        return Err("invalid hold duration");
    }
    if ends.iter().any(|end| *end < start) {
        return Err("end precedes start");
    }
    let earliest = ends.iter().min().copied();
    let effective_end = earliest.map_or(expiry, |end| end.min(expiry));
    let state = if now < start {
        TemporalState::Future
    } else if now < effective_end {
        TemporalState::Active
    } else if earliest.is_some_and(|end| end <= expiry) {
        TemporalState::Ended
    } else {
        TemporalState::Expired
    };
    Ok(Interval {
        start,
        effective_end,
        state,
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Overlap {
    Current,
    Historical,
    Future,
}

/// Compare only controls for the same canonical target. SHA equality is not an
/// input: a stale SHA must never conceal an active same-target conflict.
pub fn interval_overlap(a: &Interval, b: &Interval, now: OffsetDateTime) -> Option<Overlap> {
    let begin = a.start.max(b.start);
    let end = a.effective_end.min(b.effective_end);
    if begin >= end {
        return None;
    }
    Some(if now < begin {
        Overlap::Future
    } else if now >= end {
        Overlap::Historical
    } else {
        Overlap::Current
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    fn t(seconds: i64) -> OffsetDateTime {
        OffsetDateTime::from_unix_timestamp(seconds).unwrap()
    }
    #[test]
    fn start_end_expiry_boundaries_are_half_open() {
        let state = |now, ends: &[i64]| {
            evaluate_interval(
                t(10),
                t(30),
                &ends.iter().map(|e| t(*e)).collect::<Vec<_>>(),
                t(now),
            )
            .unwrap()
            .state
        };
        assert_eq!(state(9, &[]), TemporalState::Future);
        assert_eq!(state(10, &[]), TemporalState::Active);
        assert_eq!(state(29, &[]), TemporalState::Active);
        assert_eq!(state(30, &[]), TemporalState::Expired);
        assert_eq!(state(20, &[20]), TemporalState::Ended);
        assert_eq!(state(19, &[20]), TemporalState::Active);
        assert_eq!(state(30, &[30]), TemporalState::Ended);
        assert_eq!(state(30, &[31]), TemporalState::Expired);
        assert_eq!(state(10, &[10]), TemporalState::Ended);
    }
    #[test]
    fn earliest_end_controls_and_invalid_chronology_never_disappears() {
        let a = evaluate_interval(t(10), t(30), &[t(25), t(15), t(15), t(40)], t(20)).unwrap();
        assert_eq!(a.effective_end, t(15));
        assert_eq!(a.state, TemporalState::Ended);
        assert!(evaluate_interval(t(10), t(30), &[t(9), t(15)], t(100)).is_err());
        assert!(evaluate_interval(t(10), t(10), &[], t(10)).is_err());
        assert!(evaluate_interval(t(10), t(86411), &[], t(10)).is_err());
        assert!(evaluate_interval(t(10), t(86410), &[], t(10)).is_ok());
    }
    #[test]
    fn overlap_is_current_only_during_the_intersection() {
        let a = evaluate_interval(t(10), t(30), &[t(25)], t(0)).unwrap();
        let b = evaluate_interval(t(20), t(40), &[], t(0)).unwrap();
        assert_eq!(interval_overlap(&a, &b, t(19)), Some(Overlap::Future));
        assert_eq!(interval_overlap(&a, &b, t(20)), Some(Overlap::Current));
        assert_eq!(interval_overlap(&a, &b, t(24)), Some(Overlap::Current));
        assert_eq!(interval_overlap(&a, &b, t(25)), Some(Overlap::Historical));
        let c = evaluate_interval(t(25), t(40), &[], t(0)).unwrap();
        assert_eq!(interval_overlap(&a, &c, t(25)), None);
        let empty = evaluate_interval(t(20), t(40), &[t(20)], t(0)).unwrap();
        assert_eq!(interval_overlap(&a, &empty, t(20)), None);
    }
}
