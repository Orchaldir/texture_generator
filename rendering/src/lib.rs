#[macro_use]
extern crate glium;
#[macro_use]
extern crate log;

pub mod implementation;
pub mod interface;
pub mod logging;

use crate::implementation::vertex::ColoredVertex;
use crate::implementation::vertex::TexturedVertex;

implement_vertex!(ColoredVertex, position, color);
implement_vertex!(TexturedVertex, position, color, tc);
