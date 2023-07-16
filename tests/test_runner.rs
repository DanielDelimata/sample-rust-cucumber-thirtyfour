use cucumber::{writer, World};
use std::fs;
use thirtyfour::{DesiredCapabilities, WebDriver};

mod stepdefinitions;

#[derive(cucumber::World)]
#[world(init = Self::new)]
pub(crate) struct Context {
    driver: WebDriver,
    search_summary_at_very_beginning: String,
}

impl Context {
    async fn new() -> Self {
        let mut capabilities: thirtyfour::ChromeCapabilities = DesiredCapabilities::chrome();
        let _ = capabilities.add_chrome_arg("--headless");
        let _ = capabilities.add_chrome_arg("--start-maximized");
        let driver: WebDriver = WebDriver::new("http://localhost:4444", capabilities)
            .await
            .unwrap();
        Self {
            driver,
            search_summary_at_very_beginning: "".to_string(),
        }
    }
}

impl std::fmt::Debug for Context {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Context")
            .field("driver", &self.driver.session_id)
            .field(
                "search_summary_at_very_beginning",
                &self.search_summary_at_very_beginning,
            )
            .finish()
    }
}

#[tokio::main]
async fn main() {
    let file = fs::File::create(dbg!("target/junit.xml")).unwrap();
    Context::cucumber()
        .with_writer(writer::JUnit::new(file, 1))
        .fail_on_skipped()
        .run("tests/features")
        .await;
}
