pub mod parse;
pub mod eval;
pub mod variable;

pub mod prelude {
    pub use crate::{
        parse::prelude::*,
        eval::prelude::*,
        variable::prelude::*,
    };
}
