use {
    nannou::{prelude::*, ui::prelude::*},
    std::{env, path::PathBuf, process::exit},
};

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Model {
    path: PathBuf,
    filenames: Vec<String>,
    selected: Option<usize>,
    ui: Ui,
    ids: Ids,
}

struct Ids {
    dd_list: widget::Id,
}

fn model(app: &App) -> Model {
    let path = {
        let args: Vec<String> = env::args().collect();
        match args.get(1) {
            None => {
                eprintln!("Pass a path to scan for files");
                exit(1);
            }
            Some(path) => PathBuf::from(path),
        }
    };

    app.set_loop_mode(LoopMode::wait(3));

    let mut ui = app.new_ui().build().unwrap();

    let assets_path = app.assets_path().expect("There's no assets path!");
    let font_path = assets_path.join("fonts/DroidSansMono.ttf");
    ui.fonts_mut().insert_from_file(font_path).ok();

    let filenames = vec![
        "Option 1".to_string(),
        "Option 2".to_string(),
        "Option 3".to_string(),
    ];
    let selected = None;
    let ids = Ids {
        dd_list: ui.generate_widget_id(),
    };

    Model {
        path,
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
        .label(model.path.to_str().unwrap_or("Path wasn't passed"))
        .label_color(ui::color::BLACK)
        .color(ui::color::BLUE)
        .label_font_size(16)
        .border(1.0)
        .set(model.ids.dd_list, ui)
    {
        println!("selected {:?}", selected);
        model.selected = Some(selected);
    }
}

fn view(app: &App, model: &Model, frame: &Frame) {
    frame.clear(PURPLE);
    let draw = app.draw();

    draw.to_frame(app, &frame).unwrap();
    model.ui.draw_to_frame(app, &frame).unwrap();
}
