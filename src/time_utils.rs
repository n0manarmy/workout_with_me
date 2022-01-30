use crate::prelude::*;

const TIME_FORMAT_STAMP: &'static str = "%Y-%m-%d %H%M";
const TIME_FORMAT_SECONDS: &'static str = "%Y-%m-%d %H:%M:%S";
const TIME_FORMAT_HOURS: &'static str = "%H";
const TIME_FORMAT_MINS: &'static str = "%M";

/// # Usage
/// provided a label and a path, we build a string to be sent to our log file.
pub fn get_time_stamp(label: &str) -> String {
    // String that will be output to the file
    [&get_time(TIME_FORMAT_STAMP), " ", &label].concat()
    // call our output program to write the string with our output and the path
    // FileUtils::write_to_log_file(&log_out, &path);
}

pub fn get_current_hour_u32() -> u32 {
    (Local::now().format(TIME_FORMAT_HOURS).to_string()).parse::<u32>().expect("Error parsing string time value")
}

pub fn get_current_mins_u32() -> u32 {
    (Local::now().format(TIME_FORMAT_MINS).to_string()).parse::<u32>().expect("Error parsing string time value")
}

pub fn get_current_hours_mins_u32() -> u32 {
    (Local::now().format(&[TIME_FORMAT_HOURS, TIME_FORMAT_MINS].concat()).to_string()).parse::<u32>().expect("Error parsing string time value")
}

pub fn within_time_warning(within: u32, workout_time: u32) -> bool {
    let diff = (get_current_hours_mins_u32() as i32 - workout_time as i32).abs();
    dbg!(diff);
    if diff <= within as i32 && diff >= 0 {
        return true;
    }

    false
}

pub fn get_current_time() -> String {
    debug!("get_current_time_called");
    get_time(TIME_FORMAT_SECONDS)
}

pub fn get_time_diff(time: &TimeObject) -> String {
    match time.time_state {
        TimeState::IN => {
            let t_hours = time.time_stamp.signed_duration_since(Local::now()).num_hours().abs();
            let t_mins = time.time_stamp.signed_duration_since(Local::now()).num_minutes().abs();
            let t_mins = t_mins - (t_hours * 60);
            let t_mins = TimeObject::get_rounded_mins(t_mins);
            format!("h:{} m:{}", t_hours, t_mins)
        },
        _ => String::from("No time IN"),
    }
}

/// # Usage
/// Builds a string value based on our const TIME_FORMAT
pub fn get_time(time_format: &str) -> String {
    let time_stamp: String = Local::now().format(time_format).to_string();

    time_stamp
}


#[cfg(test)]

mod tests {

    use super::*;

    #[test]
    pub fn test_get_hours() {
        dbg!(get_current_hour_u32());
    }

    #[test]
    pub fn test_get_mins() {
        dbg!(get_current_mins_u32());
    }

    #[test]
    pub fn test_get_current_hours_mins() {
        dbg!(get_current_hours_mins_u32());
    }

    #[test]
    pub fn test_scratch() {
        dbg!(get_current_hour_u32());
        dbg!(get_current_mins_u32());
        let x = (100 * get_current_hour_u32()) + get_current_mins_u32();
        dbg!( x );
    }

    #[test]
    pub fn test_within_time_warning() {
        assert_eq!(within_time_warning(10, 1415), true);
    }
}