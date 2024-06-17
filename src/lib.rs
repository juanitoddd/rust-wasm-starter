extern crate js_sys;
extern crate wasm_bindgen;

mod game;
mod macros;
mod utils;

use std::cell::{Cell, RefCell};
use std::rc::Rc;
use web_sys::HtmlCanvasElement;

use crate::game::manager::Manager;
use crate::game::ui::graphics::Graphics;
use crate::game::utils::player_number_match;
use crate::wasm_bindgen::prelude::*;
use crate::wasm_bindgen::JsCast;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    utils::set_panic_hook();
    Ok(())
}

// TODO move all the complexity of interaction into graphics.
#[wasm_bindgen]
pub fn start_game(canvas: HtmlCanvasElement, name1: String, name2: String) {
    log!("Game Starting");    

    let graphics = Rc::new(RefCell::new(Graphics::new(canvas.clone())));
    // let manager = Rc::new(RefCell::new(Manager::new(name1, name2)));

    let original_circle_x_y = Rc::new(Cell::new((0.0, 0.0)));

    // process mousedown
    /*
    {
        let graphics = graphics.clone();
        let manager = manager.clone();
        let original_circle_x_y = original_circle_x_y.clone();

        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            let x = event.offset_x() as f64;
            let y = event.offset_y() as f64;
            let mut graphics = graphics.borrow_mut();
            let manager = manager.borrow();
            
            graphics.set_largest_clicked_circle(x, y);
            if graphics.interaction.get_chosen_circle() > -1 {
                let current_turn = manager.get_turn();
                let shape_owner = graphics.get_circle().get_player();

                if player_number_match(shape_owner, current_turn) {
                    graphics.interaction.set_pressed(true);

                    let rectangle_index = graphics.get_clicked_rectangle_index(x, y);
                    graphics.interaction.set_initial_rectangle(rectangle_index);

                    original_circle_x_y.set(graphics.get_circle().get_pos())
                } else {
                    graphics.interaction.reset_state();
                }
            }
        }) as Box<dyn FnMut(_)>);
        canvas
            .add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }
    */

    
    // process mouse move
    {        
        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            let x = event.offset_x() as f64;
            let y = event.offset_y() as f64;
            log!("{x} ~ {y}");       
        }) as Box<dyn FnMut(_)>);

        canvas
            .add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }    

    
    //process mouse up
    {
            
        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            let x = event.offset_x() as f64;
            let y = event.offset_y() as f64;
            log!("{x} - {y}");
        }) as Box<dyn FnMut(_)>);

        canvas
            .add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }    
}

#[wasm_bindgen]
pub fn keyboard_event(key: String) {
    log!("{key}");
}

#[wasm_bindgen]
pub fn mouse_event(x: i32, y: i32) {
    log!("{x}, {y}");
}
