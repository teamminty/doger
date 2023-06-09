#![cfg(test)]

use doger::prelude::*;

#[test]
fn anyhow_macro() {
    assert_eq!(doger::anyhow!("oof").to_string(), "oof");
}

#[test]
fn a() {
    struct App {
        
    }
    impl doger::app::App for App {
        fn load(builder: &mut doger::app::AppBuilder<Self>) -> Result<Self> {
            builder.at("/bananen/:id").get(&Self::bananen).add_to_builder(builder)?;
            Ok(Self {  })
        }
    }
    impl App {
        pub fn bananen(app: &mut Self, req: &mut Request) -> Result<()> {
            
            Ok(())
        }
    }
    run_app::<App>();
}