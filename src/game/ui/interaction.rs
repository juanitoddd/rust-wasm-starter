use std::cell::{Cell};
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Interaction {
    pressed: Rc<Cell<bool>>,
    chosen_circle: Rc<Cell<isize>>,
    initial_rectangle: Rc<Cell<isize>>,
    ending_rectangle: Rc<Cell<isize>>
}

impl Interaction {
    pub fn new() -> Interaction {
        let pressed = Rc::new(Cell::new(false));
        let chosen_circle = Rc::new(Cell::new(-1));
        let initial_rectangle = Rc::new(Cell::new(-1));
        let ending_rectangle = Rc::new(Cell::new(-1));
        
        Interaction {
            pressed,
            chosen_circle,
            initial_rectangle,
            ending_rectangle
        }
    }
}