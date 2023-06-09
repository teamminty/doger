mod appbuilder;

use crate::prelude::*;

pub use appbuilder::*;

pub trait App where Self: Sized {
    fn load(builder: &mut AppBuilder<Self>) -> Result<Self>;
}

pub fn build<T: App + 'static>() -> Result<()> {
    let mut builder: AppBuilder<T> = AppBuilder::<T> {
        routes: routefinder::Router::new()
    };
    let a = <T as App>::load(&mut builder)?;
    Ok(())
}