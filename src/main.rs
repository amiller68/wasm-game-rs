mod universe;
mod utils;

use universe::Universe;
use utils::set_panic_hook;

use leptos_use::use_interval_fn;
use std::cell::RefCell;
use std::{f64, rc::Rc};
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

const CELL_SIZE: f64 = 5.0;
const GRID_COLOR: &str = "#CCCCCC";
const DEAD_COLOR: &str = "#FFFFFF";
const ALIVE_COLOR: &str = "#000000";

use leptos::*;

#[component]
fn Demo() -> impl IntoView {
    let (ctx, universe) = init();
    let universe_ref = Rc::new(RefCell::new(universe));

    use_interval_fn(
        move || {
            let mut universe = universe_ref.as_ref().borrow_mut();
            universe.tick();
            draw_cells(&ctx, &universe);
        },
        100_u64,
    );

    view! {}
}

fn main() {
    set_panic_hook();
    mount_to_body(Demo);
}

pub fn init() -> (CanvasRenderingContext2d, Universe) {
    // Get the document and create a canvas element
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document
        .get_element_by_id("game-canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();

    // Get the canvas context
    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

    // Create the Universe and get its width and height
    let universe = Universe::new(None);
    let width = universe.width();
    let height = universe.height();

    // Set the canvas size based on the Universe size
    let canvas_height = ((CELL_SIZE + 1.0) * height as f64 + 1.0) as u32;
    let canvas_width = ((CELL_SIZE + 1.0) * width as f64 + 1.0) as u32;
    canvas.set_height(canvas_height);
    canvas.set_width(canvas_width);

    draw_grid(&context, &universe);

    (context, universe)
}

fn draw_grid(context: &CanvasRenderingContext2d, universe: &Universe) {
    let width = universe.width();
    let height = universe.height();
    // Draw the grid lines
    context.begin_path();
    context.set_stroke_style(&GRID_COLOR.into());

    for i in 0..=width {
        let x = i as f64 * (CELL_SIZE + 1.0) + 1.0;
        context.move_to(x, 0.0);
        context.line_to(x, (CELL_SIZE + 1.0) * height as f64 + 1.0);
    }

    for j in 0..=height {
        let y = j as f64 * (CELL_SIZE + 1.0) + 1.0;
        context.move_to(0.0, y);
        context.line_to((CELL_SIZE + 1.0) * width as f64 + 1.0, y);
    }

    context.stroke();
}

fn draw_cells(context: &CanvasRenderingContext2d, universe: &Universe) {
    // Draw the cells
    let width = universe.width();
    let height = universe.height();
    for row in 0..height {
        for col in 0..width {
            let idx = universe.get_index(row, col);

            let fill_style = if !universe.cells[idx] {
                DEAD_COLOR
            } else {
                ALIVE_COLOR
            };

            context.set_fill_style(&fill_style.into());

            let x = col as f64 * (CELL_SIZE + 1.0) + 1.0;
            let y = row as f64 * (CELL_SIZE + 1.0) + 1.0;
            context.fill_rect(x, y, CELL_SIZE, CELL_SIZE);
        }
    }
}
