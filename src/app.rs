mod appbuilder;

use crate::Result;

pub use appbuilder::*;

pub trait App where Self: Sized {
    fn load(builder: &mut AppBuilder<Self>) -> Result<Self>;
}