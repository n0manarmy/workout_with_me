// #![cfg_attr(not(feature = "gtk_3_24"), allow(unused_variables, unused_mut))]
extern crate serde;

mod file_utils;
mod time_utils;
mod manual_gui_construct;
mod message;
mod time_object;
mod my_menu_bar;

mod prelude {
    pub use crate::manual_gui_construct::*;
    pub use crate::time_object::TimeObject;
    pub use crate::file_utils::FileUtils;    
    pub use crate::file_utils::*;
    pub use crate::message::*;
    pub use crate::time_utils::*;
    pub use crate::time_object::*;
    pub use crate::my_menu_bar::*;

    pub use chrono::prelude::*;
    pub use serde::{Serialize, Deserialize};
    pub use serde_json::*;
    pub use gio::prelude::*;
    pub use gtk::prelude::CellRendererExt;
    pub use gtk::prelude::*;
    pub use std::env::args;
    pub use std::rc::Rc;

    pub use std::{thread, time, fmt};
    pub use glib::{clone, Continue};
    pub use gtk::{
        Orientation, Application,
        ApplicationWindow, TreeView
    };
}

use prelude::*;

fn main() {

    let application = gtk::Application::new(
        Some("com.workout.with.me"),
        Default::default(),
    );

    application.connect_activate(|app| {
        //init data store for times
        let log_file: &'static str = "log_file.json";
        let times: Vec<TimeObject> = if FileUtils::log_file_exists(&log_file) {
            TimeObject::build_time_object_vec(FileUtils::read_log_file_to_vec(&log_file))
        } else {
            Vec::new()
        };
        // dbg!(times);
        GuiConstruct::build(app, times, log_file);
    });

    // application.run(&args().collect::<Vec<_>>());
    application.run();
}