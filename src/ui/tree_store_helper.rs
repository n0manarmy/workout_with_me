use crate::prelude::*;

pub fn set_row(
    iter: &gtk::TreeIter, 
    tree_store: &gtk::TreeStore, 
    workout_name: String, 
    workout_time: String, 
    workouts_done: u64, 
    people_following: String
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
        (3 as u32, &workout_values.following.len().to_string().to_value()),
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