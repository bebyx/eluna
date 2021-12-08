//! Not-so-precise moon calendar for 1900-2100.
//!
//! A Rust translation of [Minkukel][article]'s algorithm, which is claimed to provide **lunar data for a period of 1900-2100**.
//!
//! Input is a timestamp ([`i64`]), whether positive or negative. It's compatible with [`chrono::DateTime::timestamp()`][chrono] function.
//!
//! [article]: https://minkukel.com/en/various/calculating-moon-phase/
//! [chrono]: https://docs.rs/chrono/latest/chrono/struct.DateTime.html#method.timestamp

use float_extras::f64::fmod;

const LUNAR_DAYS: f64 = 29.53058770576;
const MILLENIUM_NEW_MOON: i64 = 947182440;
const LUNAR_SECS: f64 = LUNAR_DAYS * (24.0 * 60.0 * 60.0);

/// Return a moon second of the given time (like a day, but second).
pub fn raw(input_timestamp: i64) -> u64 {
    let total_secs = input_timestamp - MILLENIUM_NEW_MOON;
    let mut moon_second = fmod(total_secs as f64, LUNAR_SECS);
    if moon_second.is_sign_negative() {
        moon_second = moon_second + LUNAR_SECS;
    }
    moon_second as u64
}

/// Return fraction of the moon -- convert to percent if needed.
pub fn fraction(input_timestamp: i64) -> f64 {
    raw(input_timestamp) as f64 / LUNAR_SECS
}

/// Return a moon day.
pub fn moon_day(input_timestamp: i64) -> f64 {
    fraction(input_timestamp) * LUNAR_DAYS
}

/// Return a moon phase as number (0-8).
pub fn numeric_phase(input_timestamp: i64) -> u8 {
    let moon_day = moon_day(input_timestamp);
    let phases: [[f64; 2]; 9] = [
        [0.0, 1.0],
        [1.0, 6.38264692644],
        [6.38264692644, 8.38264692644],
        [8.38264692644, 13.76529385288],
        [13.76529385288, 15.76529385288],
        [15.76529385288, 21.14794077932],
        [21.14794077932, 23.14794077932],
        [23.14794077932, 28.53058770576],
        [28.53058770576, 29.53058770576],
    ];

    let mut numeric_phase: u8 = 5;
    for (i, period) in phases.iter().enumerate() {
        if moon_day >= period[0] && moon_day <= period[1] {
            numeric_phase = i as u8;
        }
    }

    numeric_phase
}

/// Return a moon phase as a human-readable word or phrase.
pub fn verbal_phase(input_timestamp: i64) -> String {
    let numeric_phase = numeric_phase(input_timestamp);
    let verbal_phase = match numeric_phase {
        0 => "new",
        1 => "waxing crescent",
        2 => "first quarter",
        3 => "waxing gibbous",
        4 => "full",
        5 => "waning gibbous",
        6 => "last quarter",
        7 => "waning crescent",
        8 => "new",
        _ => "error",
    };

    verbal_phase.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn millenium_phase_num() {
        assert_eq!(numeric_phase(MILLENIUM_NEW_MOON), 0);
    }

    #[test]
    fn millenium_phase_verb() {
        assert_eq!(verbal_phase(MILLENIUM_NEW_MOON), String::from("new"));
    }

    // Phase moon data for the tests below is taken from https://www.calendar-12.com/moon_phases/

    // Fri Jan 10 07:23:00 PM UTC 2020
    #[test]
    fn twenty_twenty_first_full_moon() {
        assert_eq!(verbal_phase(1578684180), String::from("full"))
    }

    // Thu Jan 22 12:55:00 PM UTC 1970
    #[test]
    fn epoch_first_full_moon() {
        assert_eq!(verbal_phase(1860900), String::from("full"))
    }

    // Thu Jan 1 12:00:00 AM UTC 1970
    #[test]
    fn just_unix_timestamp() {
        assert_eq!(verbal_phase(0), String::from("last quarter"))
    }

    // Tue Jan 14 10:21:00 PM UTC 1930
    #[test]
    fn ninety_thirty_first_full_moon() {
        assert_eq!(verbal_phase(-1261100340), String::from("full"))
    }
}
