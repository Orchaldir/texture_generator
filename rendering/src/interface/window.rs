use crate::interface::app::App;
use std::cell::RefCell;
use std::rc::Rc;

pub trait Window {
    /// Runs the application using the window as the render target
    fn run(&mut self, app: Rc<RefCell<dyn App>>) -> !;
}
