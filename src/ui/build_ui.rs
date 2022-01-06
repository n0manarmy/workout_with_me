use crate::prelude::*;

const START_BUTTON_LABEL: &str = "Start Workout";
const STOP_BUTTON_LABEL: &str = "Stop Workout";
const HOURS_OPTIONS: [&str; 24] = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14", "15", "16",
"17", "18", "19", "20", "21", "22", "23"];
const MINS_OPTIONS: [&str; 12] = ["00", "05", "10", "15", "20", "25", "30", "35", "40", "45", "50", "55"];

pub fn build(app: &Application, times: Vec<TimeObject>) {
    let window = ApplicationWindow::new(app);

    window.set_title(Some("Workout with me"));
    window.set_vexpand(true);
    window.set_hexpand(true);
    window.set_default_height(600);
    window.set_default_width(800);
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

    let workout_time_label: gtk::Label = gtk::Label::new(Some("Workout Time"));
    let colon: gtk::Label = gtk::Label::new(Some(":"));

    let workout_time_hours: gtk::DropDown = gtk::DropDown::from_strings(&HOURS_OPTIONS);
    let workout_time_mins: gtk::DropDown = gtk::DropDown::from_strings(&MINS_OPTIONS);

    let content_area_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(5)
        .hexpand(true)
        .vexpand(true)
        .build();

    let name_label: gtk::Label = gtk::Label::new(Some("Workout Description"));
    let name_entry: gtk::Entry = gtk::Entry::new();
    let days_check_button_label = gtk::Label::new(Some("Days"));

    let mon_check_button_label = gtk::Label::new(Some("M"));
    let tues_check_button_label = gtk::Label::new(Some("T"));
    let wed_check_button_label = gtk::Label::new(Some("W"));
    let thurs_check_button_label = gtk::Label::new(Some("Th"));
    let fri_check_button_label = gtk::Label::new(Some("F"));

    let mon_check_button = gtk::CheckButton::new();
    let tues_check_button = gtk::CheckButton::new();
    let wed_check_button = gtk::CheckButton::new();
    let thurs_check_button = gtk::CheckButton::new();
    let fri_check_button = gtk::CheckButton::new();

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
            String::static_type(), //# done
            u64::static_type(),
        ], //# people following
    );

    let tree_view: TreeView = TreeView::builder().model(&tree_store).vexpand(true).build();

    build_table(times, &tree_view, &tree_store);

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

    let curent_time_label_text = "Current Time";
    let current_time_label: gtk::Label = gtk::Label::new(Some(curent_time_label_text));
    let current_time: gtk::Label = gtk::Label::new(Some(&time_utils::get_current_time()));

    let start_stop_button: gtk::Button = gtk::Button::builder().label("Start Workout").build();

    let add_workout_button: gtk::Button = gtk::Button::builder().label("Add workout").build();

    let window_dialog_clone = window.clone();

    let dialog = gtk::Dialog::with_buttons(
        Some("Add workout"),
        Some(&window_dialog_clone),
        DialogFlags::MODAL,
        &[
            ("Add", ResponseType::Accept),
            ("Cancel", ResponseType::Cancel),
        ],
    );

    let name_entry_clone = name_entry.clone();
    let workout_time_hours_clone = workout_time_hours.clone();
    let workout_time_mins_clone = workout_time_mins.clone();
    let mon_check_button_clone = mon_check_button.clone();
    let tues_check_button_clone = tues_check_button.clone();
    let wed_check_button_clone = wed_check_button.clone();
    let thurs_check_button_clone = thurs_check_button.clone();
    let fri_check_button_clone = fri_check_button.clone();


    dialog.connect_response(move |_, r| {
        match r {
            ResponseType::Accept => {
                // let content_area = &d.content_area();
                info!("Accept");
                let workout_name = name_entry_clone.text();
                let hours = workout_time_hours_clone.selected();
                let mins = workout_time_mins_clone.selected();
                let mon_checked = mon_check_button_clone.is_active();
                let tues_checked = tues_check_button_clone.is_active();
                let wed_checked = wed_check_button_clone.is_active();
                let thurs_checked = thurs_check_button_clone.is_active();
                let fri_checked = fri_check_button_clone.is_active();
                
                info!("{:?}", workout_name);
                info!("{:?}", &HOURS_OPTIONS[hours as usize]);
                info!("{:?}", &MINS_OPTIONS[mins as usize]);
                info!("{:?}", mon_checked);
                info!("{:?}", tues_checked);
                info!("{:?}", wed_checked);
                info!("{:?}", thurs_checked);
                info!("{:?}", fri_checked);

                let workout = Workout::new(
                    workout_name.to_string(), 
                    HOURS_OPTIONS[hours as usize].to_string(), 
                    MINS_OPTIONS[mins as usize].to_string(), 
                    mon_checked,
                    tues_checked,
                    wed_checked,
                    thurs_checked,
                    fri_checked);
                
                info!("{:?}", workout);
            },
            ResponseType::Cancel => info!("Cancel"),
            _ => (),
        }
    });

    add_workout_button.connect_clicked(move |_b| {
        name_info_box.append(&name_label);
        name_info_box.append(&name_entry);

        workout_time_box.append(&workout_time_label);
        workout_time_box.append(&workout_time_hours);
        workout_time_box.append(&colon);
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

        dialog.show();
    });

    start_stop_button.connect_clicked(move |b| {
        let button_label = b.label().expect("Error unwrapping button label");
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
            let workouts_done: String = model
                .get(&iter, 2)
                .get::<String>()
                .expect("Error parsing workouts done");
            let people_following: u64 = model
                .get(&iter, 3)
                .get::<u64>()
                .expect("Error parsing people following");

            info!(
                "{}, {}, {}, {}",
                workout_name, workout_time, workouts_done, people_following
            );
        }

        if button_label == START_BUTTON_LABEL {
            b.set_label(STOP_BUTTON_LABEL);
        } else {
            b.set_label(START_BUTTON_LABEL);
        }
    });

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
pub fn build_table(times: Vec<TimeObject>, tree_view: &gtk::TreeView, tree_store: &gtk::TreeStore) {
    add_columns(&tree_view);

    for x in 0..10 {
        let renderer = gtk::CellRendererToggle::builder()
            .activatable(true)
            .active(true)
            .radio(true)
            .build();
        renderer.connect_toggled(move |b, _| {
            info!("Clicked");
            info!("b.is_active: {}", b.is_active().to_string());
            b.set_active(!b.is_active());
            info!("b.is_active: {}", b.is_active().to_string());
        });
        const SIZE: usize = 4;
        let v: [(u32, &dyn ToValue); SIZE] = [
            (0, &"Test".to_value()),
            (1, &time_utils::get_current_time()),
            // (2, &renderer),
            // (2, &x.to_string()),
            (2, &"10".to_value()),
            (3, &x.to_value()),
        ];
        tree_store.insert_with_values(None, None, &v);
    }
}

pub fn add_columns(tree_view: &gtk::TreeView) {
    let mut pos = 0;
    {
        let renderer = gtk::CellRendererText::new();
        let column = gtk::TreeViewColumn::new();
        // renderer.set_alignment(0.5, 1.0);
        gtk::prelude::CellRendererExt::set_alignment(&renderer, 0.5, 1.0);
        column.pack_start(&renderer, true);
        column.set_title("Workout Name");
        column.set_alignment(0.5);
        column.add_attribute(&renderer, "text", pos);
        tree_view.append_column(&column);
    }

    pos += 1;
    {
        let renderer = gtk::CellRendererText::new();
        let column = gtk::TreeViewColumn::new();
        gtk::prelude::CellRendererExt::set_alignment(&renderer, 0.5, 1.0);
        column.pack_start(&renderer, true);
        column.set_title("Workout Time");
        column.set_alignment(0.5);
        column.add_attribute(&renderer, "text", pos);
        tree_view.append_column(&column);
    }

    // pos += 1;
    // {
    //     // let renderer = gtk::CellRendererText::new();
    //     let renderer = gtk::CellRendererToggle::new();
    //     // let renderer = gtk::CellRendererToggle::builder()
    //     //     .activatable(true)
    //     //     .build();

    //     // renderer.connect_toggled(move |b, _r| {
    //     //     info!("Clicked");
    //     //     info!("{:?}", b);
    //     //     info!("b.is_active: {}", b.is_active().to_string());
    //     //     b.set_active(!b.is_active());
    //     //     info!("!b.is_active: {}", b.is_active().to_string());
    //     // });

    //     let column = gtk::TreeViewColumn::new();
    //     gtk::prelude::CellRendererExt::set_alignment(&renderer, 0.5, 1.0);
    //     column.pack_start(&renderer, true);
    //     column.set_title("Start/Stop");
    //     column.set_alignment(0.5);
    //     // column.add_attribute(&renderer, "text", pos);
    //     column.add_attribute(&renderer, "toggled", pos);
    //     tree_view.append_column(&column);

    // }

    pos += 1;
    {
        let renderer = gtk::CellRendererText::new();
        let column = gtk::TreeViewColumn::new();
        gtk::prelude::CellRendererExt::set_alignment(&renderer, 0.5, 1.0);
        column.pack_start(&renderer, true);
        column.set_title("# Completed");
        column.set_alignment(0.5);
        column.add_attribute(&renderer, "text", pos);
        tree_view.append_column(&column);
    }

    pos += 1;
    {
        let renderer = gtk::CellRendererText::new();
        let column = gtk::TreeViewColumn::new();
        gtk::prelude::CellRendererExt::set_alignment(&renderer, 0.5, 1.0);
        column.pack_start(&renderer, true);
        column.set_title("# People");
        column.set_alignment(0.5);
        column.add_attribute(&renderer, "text", pos);
        tree_view.append_column(&column);
    }
}
