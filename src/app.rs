mod appbuilder;

use crate::prelude::*;

pub use appbuilder::*;

pub trait App where Self: Sized {
    fn load(builder: &mut AppBuilder<Self>) -> Result<Self>;
}

pub fn build<T: App>() -> Result<()> {
    let a = <T as App>::load(&mut AppBuilder::<T> {
        routes: routefinder::Router::new()
    })?;
    Ok(())
}