use nannou::{prelude::*, ui::prelude::*};

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Model {
    filenames: Vec<String>,
    selected: Option<usize>,
    ui: Ui,
    ids: Ids,
    resolution: usize,
    scale: f32,
}

struct Ids {
    dd_list: widget::Id,
    resolution: widget::Id,
    scale: widget::Id,
}

fn model(app: &App) -> Model {
    app.set_loop_mode(LoopMode::wait(3));

    let mut ui = app.new_ui().build().unwrap();

    let filenames = vec![
        "Option 1".to_string(),
        "Option 2".to_string(),
        "Option 3".to_string(),
    ];
    let selected = None;
    let ids = Ids {
        dd_list: ui.generate_widget_id(),
        resolution: ui.generate_widget_id(),
        scale: ui.generate_widget_id(),
    };

    Model {
        filenames,
        selected,
        ui,
        ids,
        resolution: 5,
        scale: 5.0,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let ui = &mut model.ui.set_widgets();

    fn slider(val: f32, min: f32, max: f32) -> widget::Slider<'static, f32> {
        widget::Slider::new(val, min, max)
            .w_h(200.0, 30.0)
            .label_font_size(15)
            .rgb(0.3, 0.3, 0.3)
            .label_rgb(1.0, 1.0, 1.0)
            .border(0.0)
    }

    for value in slider(model.resolution as f32, 3.0, 15.0)
        .top_left_with_margin(20.0)
        .label("Resolution")
        .set(model.ids.resolution, ui)
    {
        model.resolution = value as usize;
    }

    for value in slider(model.scale, 10.0, 500.0)
        .down(10.0)
        .label("Scale")
        .set(model.ids.scale, ui)
    {
        model.scale = value;
    }

    for selected in widget::DropDownList::new(&model.filenames, model.selected)
        .down(10.0)
        .w_h(800.0, 30.0)
        .label("Colors")
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
