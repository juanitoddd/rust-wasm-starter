use crate::wasm_bindgen::{JsCast, JsValue};

use std::f64;


use web_sys::{HtmlCanvasElement, CanvasRenderingContext2d, Path2d};

use super::shapes::{Rectangle};
use super::interaction::Interaction;


#[derive(Debug, Clone)]
pub struct Graphics {
    pub interaction: Interaction,
    element: HtmlCanvasElement,
    context: CanvasRenderingContext2d,
    rectangles: Vec<Rectangle>,    
}

impl Graphics {

    pub fn new(element: HtmlCanvasElement) -> Graphics {
        let context = element
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

        // let rectangles = Graphics::create_board(&context, &element);
        let rectangles = Graphics::create_board(&context, &element);        
        let interaction = Interaction::new();
        Graphics { 
            interaction,
            element, 
            context, 
            rectangles,
        }
    }    

    fn create_board(context: &CanvasRenderingContext2d, element: &HtmlCanvasElement) -> Vec<Rectangle> {
        let bg: JsValue = JsValue::from_str("#333");
        context.set_fill_style(&bg);
        context.fill_rect(0.0, 0.0, element.width() as f64, element.height() as f64);                                
        let mut rectangles: Vec<Rectangle> = Vec::with_capacity(16);
        rectangles
    }
}