use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .view(view)
        .run();
}

struct Model {}

fn model(app: &App) -> Model {
    app.new_window()
        .build()
        .unwrap();

    Model {}
}

fn view(app: &App, _model: &Model, frame: Frame) {
    if frame.nth() == 0 {
        frame.clear(BLACK);
    }

    let draw = app.draw();

    if frame.nth() == 0 {
        // This one is positioned differently from the others
        draw.rect()
            .x_y(100., 100.)
            .w_h(100., 100.)
            .color(RED);
    }
    else if frame.nth() == 1 {
        draw.rect()
            .x_y(100., 100.)
            .w_h(100., 100.)
            .color(GREEN);
    }
    else if frame.nth() == 2 {
        draw.rect()
            .x_y(100., 100.)
            .w_h(100., 100.)
            .color(BLUE);
    }

    draw.to_frame(app, &frame).unwrap();
}