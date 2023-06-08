use crate::{app::App, Result};

/// Not a traditional builder, but is passed into `App::load` as `load(AppBuilder)`
pub struct AppBuilder<'a, T: App> {
    routes: routefinder::Router<&'a dyn Fn(T) -> Result<()>>
}

impl<'a ,T: App> AppBuilder<'a, T> {
    pub fn at<R: Into<Route>, F: Fn(T) -> Result<()> + 'a>(&'a mut self, v: R, cb: &'a F) {
        self.routes.add(v.into().path, cb).map_err(|e| panic!("Invalid path: {}", e)).unwrap(); // TODO: Testing needed
    }
}

pub struct RouterPath/*<'a, T: App>*/ {
    
}

pub struct Route {
    pub(crate) path: &'static str
}

impl Into<Route> for &'static str {
    fn into(self) -> Route {
        return Route { path: self }
    }
}