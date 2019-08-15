use nannou::{prelude::*, ui::prelude::*};

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Model {
    filenames: Vec<String>,
    selected: Option<usize>,
    ui: Ui,
    ids: Ids,
}

struct Ids {
    dd_list: widget::Id,
}

fn model(app: &App) -> Model {
    let mut ui = app.new_ui().build().unwrap();

    let filenames = vec![
        "Option 1".to_owned(),
        "Option 2".to_owned(),
        "Option 3".to_owned(),
    ];
    let selected = Some(0);
    let ids = Ids {
        dd_list: ui.generate_widget_id(),
    };

    Model {
        filenames,
        selected,
        ui,
        ids,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let ui = &mut model.ui.set_widgets();

    for selected in widget::DropDownList::new(&model.filenames, model.selected)
        .top_left_with_margin(20.0)
        .w_h(800.0, 30.0)
        .label("Colors")
        .label_color(ui::Color::Rgba(0.3, 0.4, 0.5, 1.0))
        .border(1.0)
        .set(model.ids.dd_list, ui)
        {
            println!("selected {:?}", selected);
        model.selected = Some(selected);
    }
}

fn view(app: &App, model: &Model, frame: &Frame) {
    frame.clear(PURPLE);
    model.ui.draw_to_frame(app, &frame).unwrap();
}
