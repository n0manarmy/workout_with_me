use crate::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
enum WeekDays {
    M,
    T,
    W,
    Th,
    F,

}

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