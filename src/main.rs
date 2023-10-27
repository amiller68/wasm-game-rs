mod universe;
mod utils;

use universe::Universe;
use utils::set_panic_hook;

use leptos::*;
use leptos_use::{use_interval_fn, use_event_listener, utils::Pausable};
use std::cell::RefCell;
use std::{f64, rc::Rc};
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

const CELL_SIZE: f64 = 5.0;
const GRID_COLOR: &str = "#CCCCCC";
const DEAD_COLOR: &str = "#FFFFFF";
const ALIVE_COLOR: &str = "#000000";

#[component]
fn App() -> impl IntoView {
    // Create a reference to our app elements
    let canvas_ref: NodeRef<html::Canvas> = create_node_ref::<leptos::html::Canvas>();
    let pause_button_ref: NodeRef<html::Button> = create_node_ref::<leptos::html::Button>();
    let clear_button_ref: NodeRef<html::Button> = create_node_ref::<leptos::html::Button>();
    let spaceship_button_ref: NodeRef<html::Button> = create_node_ref::<leptos::html::Button>();

    // Create a callback to initialize our canvas
    canvas_ref.on_load(move |_| {
        // We create a new game state, in this case a universe
        let universe = Universe::new(None);

        // Access top level elements, perform necessary setup, and wrap in Rc<RefCell<T>>
        let canvas = canvas_ref.get().unwrap();
        let ctx = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();
        let width = universe.width();
        let height = universe.height();
        let canvas_height = ((CELL_SIZE + 1.0) * height as f64 + 1.0) as u32;
        let canvas_width = ((CELL_SIZE + 1.0) * width as f64 + 1.0) as u32;
        canvas.set_height(canvas_height);
        canvas.set_width(canvas_width);        
        draw_grid(&ctx, &universe);
        let universe = Rc::new(RefCell::new(universe));
        let ctx = Rc::new(ctx);
        
        // Mount event listeners on top level element
        let universe_clone = Rc::clone(&universe);
        let _ = use_event_listener(
            canvas_ref,
            leptos::ev::click,
            move |event: web_sys::MouseEvent| {
                leptos::logging::log!("click");
                handle_click(event, &universe_clone);
            },
        );

        // Mount event listeners on child elements

        // Pause and unpause the simulation
        let universe_clone = Rc::clone(&universe);
        let ctx_clone = Rc::clone(&ctx);
        pause_button_ref.on_load(move |_| {
            let Pausable {
                pause,
                resume,
                is_active
            }  = use_interval_fn(
                move || {
                    let ctx = ctx_clone.as_ref();
                    let mut universe = universe_clone.as_ref().borrow_mut();
                    universe.tick();
                    draw_cells(&ctx, &mut universe);
                },
                100_u64,
            );
            let pause_button = pause_button_ref.get().unwrap();
            pause_button.set_inner_text("Pause");
            let pause_button_clone = pause_button_ref.clone();
            let _ = use_event_listener(
                pause_button_ref,
                leptos::ev::click,
                move |_| {
                    let pause_button = pause_button_clone.get().unwrap();
                    if is_active() {
                        pause();
                        pause_button.set_inner_text("Resume");
                    } else {
                        resume();
                        pause_button.set_inner_text("Pause");
                    }
                },
            );
        });

        // Clear the simulation
        let universe_clone = Rc::clone(&universe);
        let ctx_clone = Rc::clone(&ctx);
        clear_button_ref.on_load(move |_| {
            let _ = use_event_listener(
                clear_button_ref,
                leptos::ev::click,
                move |_| {
                    let ctx = ctx_clone.as_ref();
                    let mut universe = universe_clone.as_ref().borrow_mut();
                    universe.cells.clear();
                    draw_cells(&ctx, &mut universe);
                },
            );
        });

        // Draw a spaceship in the upper left corner
        let universe_clone = Rc::clone(&universe);
        let ctx_clone = Rc::clone(&ctx);
        spaceship_button_ref.on_load(move |_| {
            let _ = use_event_listener(
                spaceship_button_ref,
                leptos::ev::click,
                move |_| {
                    let ctx = ctx_clone.as_ref();
                    let mut universe = universe_clone.as_ref().borrow_mut();
                    universe.set_cells(&[(1,2), (2,3), (3,1), (3,2), (3,3)]);
                    draw_cells(&ctx, &mut universe);
                },
            );
        });
    });

    // Return the view
    view! {
        <div>
            <h1>Wasm Game Of Life</h1>
            // We pass our node refs to the elements we want to mount event listeners on
            <canvas
                node_ref=canvas_ref
            ></canvas>
            <button
                node_ref=pause_button_ref
            ></button>
            <button
                node_ref=clear_button_ref
            >Clear</button>
            <button
                node_ref=spaceship_button_ref
            >Spaceship</button>
        </div>
    }
}

fn main() {
    set_panic_hook();
    mount_to_body(App);
}

fn handle_click(event: web_sys::MouseEvent, universe: &Rc<RefCell<Universe>>) {
    let mut universe = universe.as_ref().borrow_mut();
    let canvas = event
        .target()
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();

    let bounding_rect = canvas.get_bounding_client_rect();

    let canvas_left = bounding_rect.left() as i32;
    let canvas_top = bounding_rect.top() as i32;

    let client_x = event.client_x() as i32;
    let client_y = event.client_y() as i32;

    let left = client_x - canvas_left;
    let top = client_y - canvas_top;

    let col = (left as f64 / (CELL_SIZE + 1.0)).floor() as u32;
    let row = (top as f64 / (CELL_SIZE + 1.0)).floor() as u32;

    universe.toggle_cell(row, col);

    let ctx = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

    draw_cells(&ctx, &universe);
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
