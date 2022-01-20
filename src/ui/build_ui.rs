use crate::prelude::*;

pub const TABLE_COLUMN_COUNT: usize = 4;

pub fn build(app: &Application, workouts: Vec<Workout>) {
    let window = ApplicationWindow::new(app);

    window.set_title(Some(static_labels::TITLE));
    window.set_vexpand(true);
    window.set_hexpand(true);
    window.set_default_height(600);
    window.set_default_width(800);

    //Storage frame for all frames before adding to the window
    let completed_frame = gtk::Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(5)
        .build();

    //Right frame for the table
    let right_frame = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(5)
        .hexpand(true)
        .vexpand(true)
        .build();

    let left_frame = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(5)
        .build();

    let tree_store = gtk::TreeStore::new(
        &[
            String::static_type(), //workout name
            String::static_type(), //workout time
            // String::static_type(), //start/stop
            // gtk::CellRendererToggle::static_type(), //start/stop
            // bool::static_type(),
            u64::static_type(), //# done
            String::static_type(),
        ], //# people following
    );

    let tree_view: TreeView = TreeView::builder()
        .model(&tree_store)
        .headers_clickable(true)
        .vexpand(true)
        .build();

    let tree_model: gtk::TreeModelSort = gtk::TreeModelSort::with_model(&tree_view.model().unwrap());

    build_table(workouts, &tree_view, &tree_store);

    // let time_diff_value: gtk::Label = if times.len() > 0 {
    //     let temp = &time_utils::get_time_diff(times.last().unwrap());
    //     dbg!(&temp);
    //     gtk::Label::new(Some(&temp))
    // } else {
    //     gtk::Label::new(Some(&String::from("No time IN")))
    // };

    let scrolled_window = gtk::ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never)
        // .child(&list_store);
        .child(&tree_view)
        .build();

    let current_time_label: gtk::Label = gtk::Label::new(Some(static_labels::CURRENT_TIME_LABEL));
    let current_time: gtk::Label = gtk::Label::new(Some(&time_utils::get_current_time()));

    let start_stop_button: gtk::Button = gtk::Button::builder().label(static_labels::START_BUTTON_LABEL).build();
    
    // start_stop_button.set_sensitive(false);

    let add_workout_button: gtk::Button = gtk::Button::builder().label(static_labels::ADD_WORKOUT_BUTTON_LABEL).build();

    let window_dialog_clone = window.clone();

    let name_label: gtk::Label = gtk::Label::new(Some(static_labels::WORKOUT_DESCRIPTION_LABEL));
    let name_entry: gtk::Entry = gtk::Entry::new();
    let days_check_button_label = gtk::Label::new(Some(static_labels::SELECT_DAYS_LABEL));

    let workout_time_label: gtk::Label = gtk::Label::new(Some(static_labels::WORKOUT_TIME_LABEL));
    let workout_time_hours: gtk::DropDown = gtk::DropDown::from_strings(&static_labels::HOURS_OPTIONS);
    let workout_time_mins: gtk::DropDown = gtk::DropDown::from_strings(&static_labels::MINS_OPTIONS);

    let mon_check_button_label = gtk::Label::new(Some(static_labels::MONDAY_LABEL));
    let tues_check_button_label = gtk::Label::new(Some(static_labels::TUES_LABEL));
    let wed_check_button_label = gtk::Label::new(Some(static_labels::WEDS_LABEL));
    let thurs_check_button_label = gtk::Label::new(Some(static_labels::THURS_LABEL));
    let fri_check_button_label = gtk::Label::new(Some(static_labels::FRI_LABEL));

    let mon_check_button = gtk::CheckButton::new();
    let tues_check_button = gtk::CheckButton::new();
    let wed_check_button = gtk::CheckButton::new();
    let thurs_check_button = gtk::CheckButton::new();
    let fri_check_button = gtk::CheckButton::new();

    let tree_model_clone = tree_model.clone();

    add_workout_button.connect_clicked(clone!(@weak tree_store => move |_b| {

        let dialog = gtk::Dialog::with_buttons(
            Some(static_labels::ADD_WORKOUT_BUTTON_LABEL),
            Some(&window_dialog_clone),
            DialogFlags::MODAL,
            &[
                (static_labels::ADD_LABEL, ResponseType::Accept),
                (static_labels::CANCEL_LABEL, ResponseType::Cancel),
            ],
        );

        let name_info_box = gtk::Box::builder()
            .orientation(Orientation::Horizontal)
            .spacing(5)
            .hexpand(true)
            .vexpand(true)
            .build();

        let workout_time_box = gtk::Box::builder()
            .orientation(Orientation::Horizontal)
            .spacing(5)
            .hexpand(true)
            .vexpand(true)
            .build();

        let workout_days_box = gtk::Box::builder()
            .orientation(Orientation::Horizontal)
            .spacing(5)
            .hexpand(true)
            .build();

        let content_area_box = gtk::Box::builder()
            .orientation(Orientation::Vertical)
            .spacing(5)
            .hexpand(true)
            .vexpand(true)
            .build();

        // let tree_store_clone = tree_store.clone();

        let name_entry_clone = name_entry.clone();
        let workout_time_hours_clone = workout_time_hours.clone();
        let workout_time_mins_clone = workout_time_mins.clone();
        let mon_check_button_clone = mon_check_button.clone();
        let tues_check_button_clone = tues_check_button.clone();
        let wed_check_button_clone = wed_check_button.clone();
        let thurs_check_button_clone = thurs_check_button.clone();
        let fri_check_button_clone = fri_check_button.clone();

        name_info_box.append(&name_label);
        name_info_box.append(&name_entry);

        workout_time_box.append(&workout_time_label);
        workout_time_box.append(&workout_time_hours);
        workout_time_box.append(&workout_time_mins);

        workout_days_box.append(&days_check_button_label);
        workout_days_box.append(&mon_check_button_label);
        workout_days_box.append(&mon_check_button);
        workout_days_box.append(&tues_check_button_label);
        workout_days_box.append(&tues_check_button);
        workout_days_box.append(&wed_check_button_label);
        workout_days_box.append(&wed_check_button);
        workout_days_box.append(&thurs_check_button_label);
        workout_days_box.append(&thurs_check_button);
        workout_days_box.append(&fri_check_button_label);
        workout_days_box.append(&fri_check_button);

        content_area_box.append(&name_info_box);
        content_area_box.append(&workout_time_box);
        content_area_box.append(&workout_days_box);

        dialog.content_area().append(&content_area_box);

        dialog.connect_response(clone!(@weak tree_model => move |d, r| {
            match r {
                ResponseType::Accept => {
                    // let content_area = &d.content_area();
                    info!("Accept");
                    let hours = workout_time_hours_clone.selected();
                    let mins = workout_time_mins_clone.selected();

                    if name_entry_clone.text().to_string() != "" {
                        let workout = Workout::new(
                            name_entry_clone.text().to_string(),
                            static_labels::HOURS_OPTIONS[hours as usize].to_string(),
                            static_labels::MINS_OPTIONS[mins as usize].to_string(),
                            mon_check_button_clone.is_active(),
                            tues_check_button_clone.is_active(),
                            wed_check_button_clone.is_active(),
                            thurs_check_button_clone.is_active(),
                            fri_check_button_clone.is_active(),
                        );

                        info!("{:?}", json!(workout));
                        tree_store_helper::add_row(&workout, &tree_store);
                        tree_store_helper::save(&tree_model);
                        // tree_store_helper::save(&tree_model, &tree_store);
                        // file_utils::write_to_log_file(&json!(workout).to_string(), LOG_FILE_NAME);
                        d.destroy();
                    }
                }
                ResponseType::Cancel => {
                    info!("Cancel");
                    d.destroy();
                }
                _ => (),
            }
        }));

        dialog.show();
    }));

    start_stop_button.connect_clicked(clone!(@weak tree_store => move |b| {
        
        // let button_label = b.label().expect("Error unwrapping button label");

        // if button_label == START_BUTTON_LABEL {
        //     b.set_label(STOP_BUTTON_LABEL);
        // } else {
        //     b.set_label(START_BUTTON_LABEL);

            let selection = tree_view.selection();

            if let Some((model, iter)) = selection.selected() {
                let workout_name: String = model
                    .get(&iter, 0)
                    .get::<String>()
                    .expect("Error parsing workout name");
                let workout_time: String = model
                    .get(&iter, 1)
                    .get::<String>()
                    .expect("Error parsing workout time");
                let mut workouts_done: u64 = model
                    .get(&iter, 2)
                    .get::<u64>()
                    .expect("Error parsing workouts done");
                let people_following: String = model
                    .get(&iter, 3)
                    .get::<String>()                
                    .expect("Error parsing people following");

                workouts_done += 1;

                tree_store_helper::set_row(&iter, &tree_store, workout_name, workout_time, workouts_done, people_following);
                tree_store_helper::save(&tree_model_clone);
            }
        // }
    }));

    right_frame.append(&scrolled_window);

    left_frame.append(&current_time_label);
    left_frame.append(&current_time);
    left_frame.append(&add_workout_button);
    left_frame.append(&start_stop_button);

    let tick = move || {
        current_time.set_text(&time_utils::get_current_time());
        glib::Continue(true)
    };

    glib::timeout_add_seconds_local(1, tick);
    //END LEFT FRAME

    completed_frame.append(&left_frame);
    completed_frame.append(&right_frame);

    window.set_child(Some(&completed_frame));

    window.present();
}

// pub fn build_table(list_store: &gtk::ListBox, times: Vec<TimeObject>, col_indicies: &[u32]) -> gtk::TreeView {
pub fn build_table(workouts: Vec<Workout>, tree_view: &gtk::TreeView, tree_store: &gtk::TreeStore) {
    tree_store_helper::add_columns(&tree_view);

    for w in workouts {
        info!("{:?}", &w);
        tree_store_helper::add_row(&w, &tree_store);
    }
}