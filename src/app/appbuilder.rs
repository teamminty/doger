use crate::{app::App, prelude::*};

/// Not a traditional builder, but is passed into `App::load` as `load(AppBuilder)`
pub struct AppBuilder<T: App> {
    pub(crate) routes: routefinder::Router<RoutePath<T>>
}

impl<T: App> AppBuilder<T> {
    pub fn at<R: Into<Route>>(&mut self, v: R) -> RoutePath<T> {
        return RoutePath {
            p: v.into(),
            ..Default::default()
        };
    }
}

pub struct RoutePath<T: App> {
    p: Route,
    get: Option<Box<dyn Fn(T) -> Result<()>>>
}

impl<T: App> RoutePath<T> {
    pub fn get<F: Fn(T) -> Result<()>>(mut self, cb: &'static F) -> Self {
        self.get = Some(Box::new(cb));
        return self;
    }
    pub fn add_to_builder(self, app: &mut AppBuilder<T>) -> Result<()> {
        if self.get.is_some() {
            app.routes.add(self.p.path, self).map_err(|s| anyhow!(s))?;
        };
        Ok(())
    }
}

impl<T: App> Default for RoutePath<T> {
    fn default() -> Self {
        return Self { get: None, p: "*".into() }
    }
}

pub struct Route {
    pub(crate) path: &'static str
}

impl Into<Route> for &'static str {
    fn into(self) -> Route {
        return Route { path: self }
    }
}