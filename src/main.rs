#[allow(dead_code)]

mod error;
mod universe;
mod utils;

// use error::{Error, Result};
use universe::Universe;
use utils::set_panic_hook;

use leptos::*;
extern crate wasm_bindgen;

extern crate js_sys;
use std::f64;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

const CELL_SIZE: f64 = 5.0;
const GRID_COLOR: &str = "#CCCCCC";
const DEAD_COLOR: &str = "#FFFFFF";
const ALIVE_COLOR: &str = "#000000";

// extern crate web_sys;
// use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, console};

fn main() {
    set_panic_hook();
    leptos::mount_to_body(Game)
}

#[component]
fn Game() -> impl IntoView {
    init();
    view! {}
}

use wasm_bindgen::JsCast;


pub fn init() {
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
    let universe =  Universe::new(None);
    let width = universe.width();
    let height = universe.height();

    // Set the canvas size based on the Universe size
    canvas.set_height(((CELL_SIZE + 1.0) * height as f64 + 1.0) as u32);
    canvas.set_width(((CELL_SIZE + 1.0) * width as f64 + 1.0) as u32);

    draw_grid(&context, &universe);
    draw_cells(&context, &universe);
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