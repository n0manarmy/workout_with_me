use crate::prelude::*;

// #[derive(Serialize, Deserialize, Debug)]
// enum WeekDays {
//     M,
//     T,
//     W,
//     Th,
//     F,
// }

#[derive(Serialize, Deserialize, Debug)]
pub struct Workout {
    pub name: String,
    hour: String,
    pub num_completed: u64,
    pub following: Vec<String>,
    min: String,
    mon: bool,
    tues: bool,
    wed: bool,
    thurs: bool,
    fri: bool,
}

impl Workout {
        
    pub fn new(
        name: String,
        hour: String,
        min: String,
        mon: bool, 
        tues: bool, 
        wed: bool, 
        thurs: bool, 
        fri: bool) -> Self {
            Workout {
                name,
                hour,
                min,
                mon,
                tues,
                wed,
                thurs,
                fri,
                num_completed: 0,
                following: Vec::new(),
            }
    }

    pub fn from_tree_store(
        name: String, 
        workout_time: String, 
        workouts_done: u64, 
        _people_following: String) -> Self {

            let p = parse_time(workout_time);

            Workout {
                name,
                hour: p.0,
                min: p.1,
                mon: p.2,
                tues: p.3,
                wed: p.4,
                thurs: p.5,
                fri: p.6,
                num_completed: workouts_done,
                following: Vec::new(),
            }
    }

    pub fn format_time_values(&self) -> String {
        let days = {
            [
                if self.mon {
                "M "
                } else {
                    "".into()
                },
                if self.tues {
                    "T "
                } else {
                    "".into()
                },
                if self.wed {
                    "W "
                } else {
                    "".into()
                },
                if self.thurs {
                    "Th "
                } else {
                    "".into()
                },
                if self.fri {
                    "F "
                } else {
                    "".into()
                },
            ].concat()
        };
        
        [&self.hour, &self.min, " ", &days].concat()
    }

    pub fn to_struct(s: String) -> Self {
        match serde_json::from_str(&s) {
            Ok(k) => k,
            Err(_) => panic!("Error parsing time object from log file"),
        }
    }

    pub fn build_vec(times: Vec<String>) -> Vec<Self> {
        let mut objects: Vec<Self> = Vec::new();
        for t in times {
            objects.push(Self::to_struct(t));
        }

        objects
    }
}

pub fn parse_time(time: String) -> (String, String, bool, bool, bool, bool, bool) {
    let (hour, min) = time.split_at(2);
    let min = min.split_at(2).0;
    (hour.to_string(), min.to_string(), time.contains("M "), time.contains("T "),time.contains("W "),time.contains("Th "),time.contains("F "))
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    pub fn test_parse_time() {
        let a: String = String::from("1010 M ");
        let a_answer: (String, String, bool, bool, bool, bool, bool) = ("10".to_string(), "10".to_string(), true, false, false, false, false);
        assert_eq!(parse_time(a), a_answer);

        let b: String = String::from("2314 M T W F ");
        let b_answer: (String, String, bool, bool, bool, bool, bool) = ("23".to_string(), "14".to_string(), true, true, true, false, true);
        assert_eq!(parse_time(b), b_answer);

        let c: String = String::from("0000 M W Th F ");
        let c_answer: (String, String, bool, bool, bool, bool, bool) = ("00".to_string(), "00".to_string(), true, false, true, true, true);
        assert_eq!(parse_time(c), c_answer);

        let d: String = String::from("1245 M T W Th F ");
        let d_answer: (String, String, bool, bool, bool, bool, bool) = ("12".to_string(), "45".to_string(), true, true, true, true, true);
        assert_eq!(parse_time(d), d_answer);

        let e: String = String::from("0610 M F ");
        let e_answer: (String, String, bool, bool, bool, bool, bool) = ("06".to_string(), "10".to_string(), true, false, false, false, true);
        assert_eq!(parse_time(e), e_answer);

        let f: String = String::from("1101 M W F ");
        let f_answer: (String, String, bool, bool, bool, bool, bool) = ("11".to_string(), "01".to_string(), true, false, true, false, true);
        assert_eq!(parse_time(f), f_answer);
    }
}