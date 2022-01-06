extern crate serde;

mod file_utils;
pub mod time_utils;
mod objects;
mod ui;
// mod manual_gui_construct;
mod message;
mod time_object;
// mod my_menu_bar;

mod prelude {
    // pub use crate::manual_gui_construct::*;
    pub use crate::ui::*;
    pub use crate::time_object::TimeObject;
    pub use crate::file_utils::FileUtils;    
    pub use crate::file_utils::*;
    pub use crate::message::*;
    pub use crate::time_utils;
    pub use crate::time_object::*;
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
        let log_file: &'static str = "log_file.json";
        let times: Vec<TimeObject> = if FileUtils::log_file_exists(&log_file) {
            TimeObject::build_time_object_vec(FileUtils::read_log_file_to_vec(&log_file))
        } else {
            Vec::new()
        };
        // dbg!(times);
        // GuiConstruct::build(app, times, log_file);
        ui::build_ui::build(app, times);
    });

    // application.connect_activate(|app| {
    //     //init data store for times
    //     let log_file: &'static str = "log_file.json";
    //     let times: Vec<TimeObject> = if FileUtils::log_file_exists(&log_file) {
    //         TimeObject::build_time_object_vec(FileUtils::read_log_file_to_vec(&log_file))
    //     } else {
    //         Vec::new()
    //     };
    //     // dbg!(times);
    //     // GuiConstruct::build(app, times, log_file);
    // });

    // application.run(&args().collect::<Vec<_>>());
    app.run();
}