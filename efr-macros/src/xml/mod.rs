mod handler;
mod input;
mod markup;
mod ns_check;
mod print;

pub use handler::handler;
pub use input::{Input, NsBlock};
pub use markup::*;
pub use print::Instr;
