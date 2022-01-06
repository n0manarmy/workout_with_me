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
    name: String,
    hour: String,
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
            }

    }
}