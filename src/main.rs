mod error;
mod universe;
mod utils;

use error::{Error, Result};
use universe::Universe;
use utils::set_panic_hook;

use leptos::*;
extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

extern crate js_sys;
use js_sys::Math;
use std::borrow::Borrow;

// extern crate web_sys;
// use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, console};

fn main() {
    set_panic_hook();
    leptos::mount_to_body(Game)
}

#[component]
fn Game() -> impl IntoView {
    console::log_1(&"Hello from Rust!".into());
    init();
    view! {}
}

use std::borrow::BorrowMut;
use std::cell::Cell;
use std::cmp::min;
use std::f64;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{console, CanvasRenderingContext2d, Document, HtmlCanvasElement, MouseEvent, Window};
use std::cell::RefCell;
const CELL_SIZE: f64 = 5.0;
const GRID_COLOR: &str = "#CCCCCC";
const DEAD_COLOR: &str = "#FFFFFF";
const ALIVE_COLOR: &str = "#000000";

#[wasm_bindgen]
extern "C" {
    fn requestAnimationFrame(closure: &Closure<dyn FnMut()>) -> i32;
}

pub fn init() {
    // Get the document and create a canvas element
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document
        .get_element_by_id("game-of-life-canvas")
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
    let mut universe = Rc::new(RefCell::new(universe));

    // Set the canvas size based on the Universe size
    canvas.set_height(((CELL_SIZE + 1.0) * height as f64 + 1.0) as u32);
    canvas.set_width(((CELL_SIZE + 1.0) * width as f64 + 1.0) as u32);

    // // Add event listener for canvas click
    // let mut universe_clone = universe.clone();
    // let closure = Closure::once(|event: MouseEvent| {
    //     handle_canvas_click(universe_clone, &canvas, &event);
    // });
    // canvas.set_onclick(Some(closure.as_ref().unchecked_ref()));
    // closure.forget();

        // Animation loop
        let mut animation_id = Rc::new(RefCell::new(None));
        // let mut animation_closure: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = Rc::new(RefCell::new(None));
        let mut animation_closure =  Rc::new(RefCell::new(None));

        let animation_callback = AnimationCallback {
            canvas: canvas.clone(),
            context: context.clone(),
            universe: universe.clone(),
            animation_id: animation_id.clone()
        };
    
        // animation_callback.render();
        // let window = web_sys::window().unwrap();

        // let mut animation_closure = Rc::new(Some(Closure::new(move || {
        //     animation_callback.render();
        //     let closure = animation_closure.borrow_mut().take().unwrap();
        //     let handle = requestAnimationFrame(&closure);
        //     animation_id.borrow_mut().replace(Some(handle));
        // })));
        // let binding = Rc::clone(&animation_closure);
        // let mut binding = binding.as_ref();
        // let mut closure = binding.borrow_mut().unwrap();
        // let handle = requestAnimationFrame(&closure);
        // closure.forget();
        animation_callback.render();
let window = web_sys::window().unwrap();

let animation_closure = Rc::new(RefCell::new(Some(Closure::new(move || {
    animation_callback.render();
    let mut closure = animation_closure.borrow_mut().take().unwrap();
    let handle = window
        .request_animation_frame(closure.as_ref().unchecked_ref().unwrap())
        .unwrap();
    animation_id.borrow_mut().replace(Some(handle));
}))));
let handle = window
    .request_animation_frame(animation_closure.borrow().as_ref().unwrap().as_ref().unchecked_ref().unwrap())
    .unwrap();
animation_closure.borrow_mut().as_mut().unwrap().replace(Some(handle));
}

// fn handle_canvas_click(universe: Rc<RefCell<Universe>>, canvas: &HtmlCanvasElement, event: &MouseEvent) {
//     let bounding_rect = canvas.get_bounding_client_rect();
//     let scale_x = canvas.width() as f64 / bounding_rect.width();
//     let scale_y = canvas.height() as f64 / bounding_rect.height();
//     let canvas_left = (event.client_x() as f64 - bounding_rect.left()) * scale_x;
//     let canvas_top = (event.client_y() as f64 - bounding_rect.top()) * scale_y;
//     let row = (canvas_top / (CELL_SIZE + 1.0)).floor() as u32;
//     let col = (canvas_left / (CELL_SIZE + 1.0)).floor() as u32;
//     let mut universe = universe.borrow_mut();
//     universe.toggle_cell(row, col);
// }

struct AnimationCallback {
    canvas: HtmlCanvasElement,
    context: CanvasRenderingContext2d,
    universe: Rc<RefCell<Universe>>,
    animation_id: Rc<RefCell<Option<i32>>>
}

impl AnimationCallback {
    fn render(&self) {
        // Implement your rendering logic here
        // Example: drawGrid(&self.context);
        //           drawCells(&self.context, &self.universe);
        //           self.universe.borrow_mut().tick();
    }
}

// const CELL_SIZE: f64 = 5.0; // px
// const GRID_COLOR: &str = "#CCCCCC";
// const DEAD_COLOR: &str = "#FFFFFF";
// const ALIVE_COLOR: &str = "#000000";

// pub fn init() {
//     let universe = Universe::new(None);
//     let width = universe.width();
//     let height = universe.height();

//     // Create a canvas element and get its context
//     let document = web_sys::window().unwrap().document().unwrap();
//     let canvas = document
//         .get_element_by_id("game-canvas")
//         .unwrap()
//         .dyn_into::<HtmlCanvasElement>()
//         .unwrap();
//     canvas.set_height(((CELL_SIZE + 1.0) * height as f64 + 1.0) as u32);
//     canvas.set_width(((CELL_SIZE + 1.0) * width as f64 + 1.0) as u32);

//     let context = canvas
//         .get_context("2d")
//         .unwrap()
//         .unwrap()
//         .dyn_into::<CanvasRenderingContext2d>()
//         .unwrap();

//     // Animation loop
//     let mut animation_id = None;
//     let render_loop = || {
//         draw_grid(&context, &universe);
//         draw_cells(&context, &universe);
//         universe.tick();
//         animation_id = Some(request_animation_frame(render_loop));
//     };

//     render_loop();

//     // Add event listener for canvas click
//     let canvas_clone = canvas.clone();
//     let universe_clone = universe.clone();
//     let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
//         handle_canvas_click(&canvas_clone, &universe_clone, &event);
//     }) as Box<dyn FnMut(_)>);

//     canvas.set_onclick(Some(closure.as_ref().unchecked_ref()));
//     closure.forget();
// }

// fn handle_canvas_click(
//     canvas: &HtmlCanvasElement,
//     universe: &Universe,
//     event: &web_sys::MouseEvent,
// ) {
//     let bounding_rect = canvas.get_bounding_client_rect();
//     let scale_x = canvas.width() as f64 / bounding_rect.width();
//     let scale_y = canvas.height() as f64 / bounding_rect.height();
//     let canvas_left = (event.client_x() as f64 - bounding_rect.left()) * scale_x;
//     let canvas_top = (event.client_y() as f64 - bounding_rect.top()) * scale_y;
//     let row = (canvas_top / (CELL_SIZE + 1.0)).floor() as u32;
//     let col = (canvas_left / (CELL_SIZE + 1.0)).floor() as u32;
//     universe.toggle_cell(row, col);
//     draw_grid(&canvas, &universe);
//     draw_cells(&canvas, &universe);
// }

// fn draw_grid(context: &CanvasRenderingContext2d, universe: &Universe) {
//     let width = universe.width();
//     let height = universe.height();
//     // Draw the grid lines
//     context.begin_path();
//     context.set_stroke_style(&GRID_COLOR.into());

//     for i in 0..=width {
//         let x = i as f64 * (CELL_SIZE + 1.0) + 1.0;
//         context.move_to(x, 0.0);
//         context.line_to(x, (CELL_SIZE + 1.0) * height as f64 + 1.0);
//     }

//     for j in 0..=height {
//         let y = j as f64 * (CELL_SIZE + 1.0) + 1.0;
//         context.move_to(0.0, y);
//         context.line_to((CELL_SIZE + 1.0) * width as f64 + 1.0, y);
//     }

//     context.stroke();
// }

// fn draw_cells(context: &CanvasRenderingContext2d, universe: &Universe) {
//     // Draw the cells
//     let width = universe.width();
//     let height = universe.height();
//     for row in 0..height {
//         for col in 0..width {
//             let idx = universe.get_index(row, col);

//             let fill_style = if !universe.cells[idx] {
//                 DEAD_COLOR
//             } else {
//                 ALIVE_COLOR
//             };

//             context.set_fill_style(&fill_style.into());

//             let x = col as f64 * (CELL_SIZE + 1.0) + 1.0;
//             let y = row as f64 * (CELL_SIZE + 1.0) + 1.0;
//             context.fill_rect(x, y, CELL_SIZE, CELL_SIZE);
//         }
//     }
// }

// fn request_animation_frame(f: impl 'static + FnMut()) -> i32 {
//     web_sys::window()
//         .unwrap()
//         .request_animation_frame(f.into())
//         .unwrap()
// }
