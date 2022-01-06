use crate::prelude::*;

pub fn build (times: Vec<TimeObject>) -> gtk::Box {
    let frame = gtk::Box::builder()
    .orientation(Orientation::Vertical)
    .spacing(5)
    .hexpand(true)
    .vexpand(true)
    .build();

    let time_diff_value: gtk::Label = if times.len() > 0 {
        let temp = &time_utils::get_time_diff(times.last().unwrap());
        dbg!(&temp);
        gtk::Label::new(Some(&temp))
    } else {
        gtk::Label::new(Some(&String::from("No time IN")))
    };

    let scrolled_window = gtk::ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never)
        // .child(&list_store);
        .child(&build_table(times))
        .build();

    frame.append(&scrolled_window);

    frame
}

// pub fn build_table(list_store: &gtk::ListBox, times: Vec<TimeObject>, col_indicies: &[u32]) -> gtk::TreeView {
pub fn build_table(times: Vec<TimeObject>) -> gtk::TreeView {
    let tree_view: TreeView = TreeView::new();
    let tree_store = gtk::TreeStore::new(&[
        String::static_type(), //workout name
        String::static_type(), //workout time
        // String::static_type(), //start/stop
        // gtk::CellRendererToggle::static_type(), //start/stop
        // bool::static_type(),
        String::static_type(), //# done
        String::static_type()] //# people following
    );

    tree_view.set_model(Some(&tree_store));
    tree_view.set_vexpand(true);

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
    
    tree_view
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