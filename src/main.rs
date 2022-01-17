extern crate serde;

pub mod file_utils;
pub mod time_utils;
mod objects;
mod ui;
// mod manual_gui_construct;
// mod message;
// mod my_menu_bar;

mod prelude {
    // pub use crate::manual_gui_construct::*;
    pub use crate::ui::*;
    pub use crate::objects::time_object::TimeObject;
    pub use crate::file_utils;
    // pub use crate::message::*;
    pub use crate::time_utils;
    pub use crate::objects::time_object::*;
    pub use crate::objects::workout::*;
    // pub use crate::cust_timer_label::*;
    // pub use crate::my_menu_bar::*;

    pub use chrono::prelude::*;
    pub use serde::{Serialize, Deserialize};
    pub use serde_json::*;
    pub use gio::prelude::*;
    pub use std::env::args;
    pub use std::rc::Rc;
    pub use log::{info, warn, error, debug};

    pub use std::{thread, time, fmt};
    // pub use glib::{clone, Continue};
    pub use gtk::subclass::prelude::*;
    pub use gtk::glib;
    pub use glib::clone;
    pub use gtk::{
        prelude::*,
        Orientation, Application,
        ApplicationWindow, TreeView,
        ResponseType, DialogFlags,
    };
}

use prelude::*;

fn main() {
    pretty_env_logger::init();

    debug!("init main");

    let app = Application::builder()
        .application_id("com.workout.with.me")
        .build();
    
    app.connect_activate(|app| {
        let workouts: Vec<Workout> = if file_utils::log_file_exists(build_ui::LOG_FILE_NAME) {
            Workout::build_vec(file_utils::read_log_file_to_vec(build_ui::LOG_FILE_NAME))
        } else {
            Vec::new()
        };
        ui::build_ui::build(app, workouts);
    });

    app.run();
}