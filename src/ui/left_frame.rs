use crate::prelude::*;

const START_BUTTON_LABEL: &str = "Start Workout";
const STOP_BUTTON_LABEL: &str = "Stop Workout";

pub fn build() -> gtk::Box {

    let curent_time_label_text = "Current Time";
    let current_time_label: gtk::Label = gtk::Label::new(Some(curent_time_label_text));
    let current_time: gtk::Label = gtk::Label::new(Some(&time_utils::get_current_time()));
    let frame = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(5)
        .build();

    let start_stop_button: gtk::Button = gtk::Button::builder()
        .label("Start Workout")
        .build();

    // start_stop_button.set_property("button_state", START_BUTTON_LABEL).expect("Error adding property");
    
    start_stop_button.connect_clicked(move |b|{        
        let button_label = b.label().expect("Error unwrapping button label");

        if button_label == START_BUTTON_LABEL {
            info!("setting {}", STOP_BUTTON_LABEL);
            b.set_label(STOP_BUTTON_LABEL);
        } else {
            info!("setting {}", START_BUTTON_LABEL);
            b.set_label(START_BUTTON_LABEL);
        }
    });

    // ----> Clock threads
    // Thread manager for current time
    // let (msg_sender, msg_receiver) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
    // let current_time_clone = current_time.clone();
    // // update enum time value
    // thread::spawn(move || {
    //     loop {
    //         thread::sleep(time::Duration::from_secs(1));
    //         let _ = msg_sender.send(Message::UpdateLabel(TimeUtils::get_current_time()));
    //         info!("{}", TimeUtils::get_current_time());
    //     }
    // });
    // // set gtk label with time value
    // msg_receiver.attach(None, move |msg| {
    //     match msg {
    //         Message::UpdateLabel(text) => current_time_clone.set_text(text.as_str()),
    //     }
    //     Continue(true)
    // });

    frame.append(&current_time_label);
    frame.append(&current_time);
    frame.append(&start_stop_button);

    let tick = move || {
        current_time.set_text(&time_utils::get_current_time());
        glib::Continue(true)
    };

    glib::timeout_add_seconds_local(1, tick);

    frame
}