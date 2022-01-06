use crate::prelude::*;



const TIME_FORMAT_STAMP: &'static str = "%Y-%m-%d %H%M";
const TIME_FORMAT_SECONDS: &'static str = "%Y-%m-%d %H:%M:%S";

/// # Usage
/// provided a label and a path, we build a string to be sent to our log file.
pub fn get_time_stamp(label: &str) -> String {
    // String that will be output to the file
    [&get_time(TIME_FORMAT_STAMP), " ", &label].concat()
    // call our output program to write the string with our output and the path
    // FileUtils::write_to_log_file(&log_out, &path);
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
