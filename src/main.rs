mod universe;
mod utils;

use universe::Universe;
use utils::set_panic_hook;

use leptos::*;
use leptos_use::{use_event_listener, use_interval_fn, utils::Pausable};
use std::cell::RefCell;
use std::{f64, rc::Rc};
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

const CELL_SIZE: f64 = 10.0;
const GRID_COLOR: &str = "#CCCCCC";
const DEAD_COLOR: &str = "#FFFFFF";
const DEFAULT_COLOR: &str = "#000000";

/// High level context object to pass to event listeners
struct Context {
    // Access to the rendering context
    rendering_context: CanvasRenderingContext2d,
    // State of the universe
    universe: Universe,

    // Rendering configuration:
    // Whether to use color
    color: bool
}

impl Context {
    fn new(rendering_context: CanvasRenderingContext2d, universe: Universe, color: bool) -> Self {
        Self {
            rendering_context,
            universe,
            color
        }
    }

    pub fn rendering_context(&self) -> &CanvasRenderingContext2d {
        &self.rendering_context
    }
    pub fn universe(&self) -> &Universe {
        &self.universe
    }
    pub fn universe_mut(&mut self) -> &mut Universe {
        &mut self.universe
    } 
    pub fn color(&self) -> bool {
        self.color
    }
    pub fn flip_color(&mut self) {
        self.color = !self.color;
    }
}

#[component]
fn App() -> impl IntoView {
    /* Create Refernces for our viewable components */
    let canvas_ref: NodeRef<html::Canvas> = create_node_ref::<leptos::html::Canvas>();
    let pause_button_ref: NodeRef<html::Button> = create_node_ref::<leptos::html::Button>();
    let clear_button_ref: NodeRef<html::Button> = create_node_ref::<leptos::html::Button>();
    let reset_button_ref: NodeRef<html::Button> = create_node_ref::<leptos::html::Button>();
    let spaceship_button_ref: NodeRef<html::Button> = create_node_ref::<leptos::html::Button>();
    let color_button_ref: NodeRef<html::Button> = create_node_ref::<leptos::html::Button>();

    /*  Everything relies on the canvas being loaded, so we mount our event listeners here */
    canvas_ref.on_load(move |_| {
        // Get the now loaded canvas element and extract the rendering context
        let canvas = canvas_ref.get().unwrap();
        let ctx = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();
        
        // Create a new universe and draw it
        let universe = Universe::new(None);
        let width = universe.width();
        let height = universe.height();
        let canvas_height = ((CELL_SIZE + 1.0) * height as f64 + 1.0) as u32;
        let canvas_width = ((CELL_SIZE + 1.0) * width as f64 + 1.0) as u32;
        canvas.set_height(canvas_height);
        canvas.set_width(canvas_width);
        
        /* Create a context object to pass to our event listeners */

        let mut ctx = Context::new(ctx, universe, false);
        draw_grid(&mut ctx);
        let ctx = Rc::new(RefCell::new(ctx));

        /* Mount event listeners on top level element */

        // Handle clicks on the canvas
        let ctx_clone = Rc::clone(&ctx); 
        let _ = use_event_listener(
            canvas_ref,
            leptos::ev::click,
            move |event: web_sys::MouseEvent| {
                leptos::logging::log!("click");
                let mut ctx = ctx_clone.as_ref().borrow_mut();
                handle_click_on_canvas(event, &mut ctx);
            },
        );

        /* Mount Event listeners on child elements */

        // Pause and unpause the simulation
        let ctx_clone = Rc::clone(&ctx);
        pause_button_ref.on_load(move |_| {
            // Create a pausable interval function
            let Pausable {
                pause,
                resume,
                is_active,
            } = use_interval_fn(
                move || {
                    let mut ctx = ctx_clone.as_ref().borrow_mut();
                    let universe = ctx.universe_mut();
                    universe.tick();
                    draw_cells(&mut ctx);
                },
                100_u64,
            );
            // Mount the event listener on the pause button
            let _ = use_event_listener(pause_button_ref, leptos::ev::click, move |_| {
                let pause_button = pause_button_ref.get().unwrap();
                if is_active() {
                    pause();
                    pause_button.set_inner_text("Resume");
                } else {
                    resume();
                    pause_button.set_inner_text("Pause");
                }
            });
        });

        // Clear the simulation
        let ctx_clone = Rc::clone(&ctx);
        clear_button_ref.on_load(move |_| {
            let _ = use_event_listener(clear_button_ref, leptos::ev::click, move |_| {
                let mut ctx = ctx_clone.as_ref().borrow_mut(); 
                let universe = ctx.universe_mut();
                universe.cells.clear();
                draw_cells(&ctx);
            });
        });

        // Reset the simulation
        let ctx_clone = Rc::clone(&ctx);
        reset_button_ref.on_load(move |_| {
            let _ = use_event_listener(reset_button_ref, leptos::ev::click, move |_| {
                let ctx = ctx_clone.as_ref();
                let mut ctx = ctx.borrow_mut();
                let universe = ctx.universe_mut();
                universe.reset();
                draw_cells(&ctx); 
            });
        });

        // Draw a spaceship in the upper left corner
        let ctx_clone = Rc::clone(&ctx);
        spaceship_button_ref.on_load(move |_| {
            let _ = use_event_listener(spaceship_button_ref, leptos::ev::click, move |_| {
                let mut ctx = ctx_clone.as_ref().borrow_mut();
                let universe = ctx.universe_mut();
                universe.set_cells(&[(1,2), (2,3), (3,1), (3,2), (3,3)]);
                draw_cells(&ctx);
            });
        });

        // Determine if we should use random colors
        let ctx_clone = Rc::clone(&ctx);
        color_button_ref.on_load(move |_| {
            let _ = use_event_listener(color_button_ref, leptos::ev::click, move |_| {
                let mut ctx = ctx_clone.as_ref().borrow_mut();
                let color_button = color_button_ref.get().unwrap();
                if ctx.color() {
                    color_button.set_inner_text("B/W");
                } else {
                    color_button.set_inner_text("Color");
                }
                ctx.flip_color();
                draw_cells(&ctx);
            });
        });
    });

    // Return the view
    view! {
        <div class="container">
            <h1 class="title">Wasm Game Of Life</h1>
            // We pass our node refs to the elements we want to mount event listeners on
            <div class="controls">
                <button
                    node_ref=pause_button_ref
                >Pause</button>
                <button
                    node_ref=clear_button_ref
                >Clear</button>
                <button
                    node_ref=reset_button_ref
                >Reset</button>
                <button
                    node_ref=spaceship_button_ref
                >Spaceship</button>
                <button
                    node_ref=color_button_ref
                >Color</button>
            </div>
            <canvas
                node_ref=canvas_ref
            ></canvas>
        </div>
    }
}

fn main() {
    set_panic_hook();
    mount_to_body(App);
}

fn handle_click_on_canvas(event: web_sys::MouseEvent, context: &mut Context) {
    let universe = context.universe_mut();
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

    draw_cells(context);
}

fn draw_grid(context: &Context) {
    let universe = context.universe();
    let context = context.rendering_context();
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

fn draw_cells(context: &Context) {
    let universe = context.universe();
    let color = context.color();
    let context = context.rendering_context();
    // Draw the cells
    let width = universe.width();
    let height = universe.height();
    for row in 0..height {
        for col in 0..width {
            let idx = universe.get_index(row, col);

            let random_color = random_color::RandomColor::new().to_hex();
            let fill_style = if !universe.cells[idx] {
                DEAD_COLOR
            } else {
                if color {
                    &random_color
                } else {
                    DEFAULT_COLOR
                }
            };

            context.set_fill_style(&fill_style.into());

            let x = col as f64 * (CELL_SIZE + 1.0) + 1.0;
            let y = row as f64 * (CELL_SIZE + 1.0) + 1.0;
            context.fill_rect(x, y, CELL_SIZE, CELL_SIZE);
        }
    }
}
