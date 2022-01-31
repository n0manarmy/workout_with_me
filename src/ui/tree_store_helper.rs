use crate::prelude::*;

pub fn check_times(model: &gtk::TreeModelSort) {
    // let iter: gtk::TreeIter = model.iter_first().expect("Unable to create iter from tree_model");
    // let path: gtk::TreePath = model.path(&iter);

    let mut workouts: Vec<Workout> = Vec::new();

    // ::<(&gtk::TreeModel, &gtk::TreePath, &gtk::TreeIter)>
    let _ = &model.foreach(
        |model: &gtk::TreeModel, _path: &gtk::TreePath, iter: &gtk::TreeIter| {
            let workout_name: String = model
                .get(&iter, 0)
                .get::<String>()
                .expect("Error parsing workout name");
            let workout_time: String = model
                .get(&iter, 1)
                .get::<String>()
                .expect("Error parsing workout time");
            let workouts_done: u64 = model
                .get(&iter, 2)
                .get::<u64>()
                .expect("Error parsing workouts done");
            let people_following: String = model
                .get(&iter, 3)
                .get::<String>()
                .expect("Error parsing people following");

            // info!("{:?}, {:?}, {:?}", model, path, iter);
            // info!("{:?}, {:?}, {:?}, {:?}", workout_name, workout_time, workouts_done, people_following);

            let workout = Workout::from_tree_store(
                workout_name,
                workout_time,
                workouts_done,
                people_following,
            );

            info!("{:?}", json!(workout));
            workouts.push(workout);
            // file_utils::write_to_log_file(&json!(workout).to_string(), build_ui::LOG_FILE_NAME);

            false
        },
    );

    for w in workouts {
        if time_utils::within_time_warning(15, w.get_hours_mins_u32()) {
            let dialog = gtk::Dialog::new();
            // let dialog = gtk::Dialog::with_buttons(
            //     Some(static_labels::CONFIRM_RESET_TABLE),
            //     Some(&window),
            //     DialogFlags::MODAL,
            //     &[
            //         (static_labels::YES_LABEL, ResponseType::Yes),
            //         (static_labels::NO_LABEL, ResponseType::No),
            //     ],
            // );

            // dialog.connect_response(move |d, r| {
            //     match r {
            //         ResponseType::Yes => {
            //             // let content_area = &d.content_area();
            //             // tree_store.clear();
            //             d.destroy();
            //         }
            //         ResponseType::No => {
            //             d.destroy();
            //         }
            //         _ => (),
            //     }
            // });

            dialog.show();

            break;
        }
    }
}

pub fn save(model: &gtk::TreeModelSort) {
    // let iter: gtk::TreeIter = model.iter_first().expect("Unable to create iter from tree_model");
    // let path: gtk::TreePath = model.path(&iter);

    let mut workouts: Vec<Workout> = Vec::new();

    // ::<(&gtk::TreeModel, &gtk::TreePath, &gtk::TreeIter)>
    info!(
        "{:?}",
        &model.foreach(
            |model: &gtk::TreeModel, _path: &gtk::TreePath, iter: &gtk::TreeIter| {
                let workout_name: String = model
                    .get(&iter, 0)
                    .get::<String>()
                    .expect("Error parsing workout name");
                let workout_time: String = model
                    .get(&iter, 1)
                    .get::<String>()
                    .expect("Error parsing workout time");
                let workouts_done: u64 = model
                    .get(&iter, 2)
                    .get::<u64>()
                    .expect("Error parsing workouts done");
                let people_following: String = model
                    .get(&iter, 3)
                    .get::<String>()
                    .expect("Error parsing people following");

                // info!("{:?}, {:?}, {:?}", model, path, iter);
                // info!("{:?}, {:?}, {:?}, {:?}", workout_name, workout_time, workouts_done, people_following);

                let workout = Workout::from_tree_store(
                    workout_name,
                    workout_time,
                    workouts_done,
                    people_following,
                );

                info!("{:?}", json!(workout));
                workouts.push(workout);
                // file_utils::write_to_log_file(&json!(workout).to_string(), build_ui::LOG_FILE_NAME);

                false
            },
        )
    );

    let out: String = workouts
        .into_iter()
        .map(|s| [json!(s).to_string(), "\n".to_string()].concat())
        .collect::<String>();

    file_utils::write_to_log_file(&out, static_labels::LOG_FILE_NAME);
}

pub fn set_row(
    iter: &gtk::TreeIter,
    tree_store: &gtk::TreeStore,
    workout_name: String,
    workout_time: String,
    workouts_done: u64,
    people_following: String,
) {
    let v: [(u32, &dyn ToValue); build_ui::TABLE_COLUMN_COUNT] = [
        (0 as u32, &workout_name.to_value()),
        (1 as u32, &workout_time.to_value()),
        (2 as u32, &workouts_done.to_value()),
        (3 as u32, &people_following.to_value()),
    ];
    tree_store.set(&iter, &v);
}

pub fn add_row(workout_values: &Workout, tree_store: &gtk::TreeStore) {
    let v: [(u32, &dyn ToValue); build_ui::TABLE_COLUMN_COUNT] = [
        (0 as u32, &workout_values.name.to_value()),
        (1 as u32, &workout_values.format_time_values().to_value()),
        (2 as u32, &workout_values.num_completed.to_value()),
        (
            3 as u32,
            &workout_values.following.len().to_string().to_value(),
        ),
    ];
    tree_store.insert_with_values(None, None, &v);
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
