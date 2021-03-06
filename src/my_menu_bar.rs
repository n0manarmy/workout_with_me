use crate::prelude::*;

pub struct MyMenuBar;

impl MyMenuBar {

    pub fn init() -> gtk::Box {
        let completed_frame = gtk::Box::new(Orientation::Vertical, 5);
        let help_menu = gtk::Menu::new();
        let menu_bar = gtk::MenuBar::new();
        let help = gtk::MenuItem::with_label("Help");
        let about = gtk::MenuItem::with_label("About");

        help_menu.append(&about);
        help.set_submenu(Some(&help_menu));
        menu_bar.append(&help);

        about.connect_activate(move |_| {
            let d = gtk::AboutDialog::new();
            d.set_title("Time Tracker");
            d.set_authors(&["Penn Wilbert"]);
            d.set_version(Some("1.0"));
            d.run();
            // d.destroy();
        });

        completed_frame.pack_start(&menu_bar, false, false, 20);
        completed_frame
    }
}