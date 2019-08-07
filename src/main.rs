// use {std::{env, fs::File}, nannou::prelude::*};

use nannou::prelude::*;

fn main() {
    // let args: Vec<String> = env::args().collect();
    // let filename = &args[1];

    nannou::app(model).update(update).simple_window(view).run();
}

struct Model {}

fn model(_app: &App) -> Model {
    Model {}
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(_app: &App, _model: &Model, frame: &Frame) {
    frame.clear(PURPLE);
}
