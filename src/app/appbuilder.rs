use crate::{app::App, prelude::*};

/// Not a traditional builder, but is passed into `App::load` as `load(AppBuilder)`
pub struct AppBuilder<'a, T: App> {
    pub(crate) routes: routefinder::Router<&'a RoutePath<'a, T>>
}

impl<'a ,T: App> AppBuilder<'a, T> {
    pub fn at<R: Into<Route>>(&'a mut self, v: R) -> RoutePath<'a, T> {
        return RoutePath {
            p: v.into(),
            ..Default::default()
        };
    }
}

pub struct RoutePath<'a, T: App> {
    p: Route,
    get: Option<&'a dyn Fn(T) -> Result<()>>
}

impl<'a, T: App> RoutePath<'a, T> {
    pub fn get<F: Fn(T) -> Result<()> + 'a>(&mut self, cb: &'a F) -> &Self {
        self.get = Some(cb);
        return self;
    }
    pub fn add_to_builder(&'a self, app: &mut AppBuilder<'a, T>) -> Result<()> {
        if self.get.is_some() {
            app.routes.add(self.p.path, self).map_err(|s| anyhow!(s))?;
        };
        Ok(())
    }
}

impl<'a, T: App> Default for RoutePath<'a, T> {
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