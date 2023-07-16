use core::time;
use std::thread::sleep;
use thirtyfour::{
    components::{Component, ElementResolver, SelectElement},
    prelude::*,
    resolve,
};

#[derive(Component, Clone)]
pub struct CustomersPage {
    pub(crate) base: WebElement,

    #[by(id = "clear-button")]
    pub(crate) clear_button: ElementResolver<WebElement>,

    #[by(id = "search-input")]
    pub(crate) search_input: ElementResolver<WebElement>,

    #[by(id = "search-column")]
    pub(crate) drop_down: ElementResolver<WebElement>,

    #[by(id = "match-case")]
    pub(crate) match_case: ElementResolver<WebElement>,

    #[by(id = "table-resume")]
    pub(crate) summary: ElementResolver<WebElement>,
}

impl CustomersPage {
    pub async fn from_driver_ref(driver_ref: &WebDriver) -> WebDriverResult<CustomersPage> {
        let base_element = driver_ref.query(By::XPath("//html")).single().await?;
        let customers_page: CustomersPage = base_element.into();
        Ok(customers_page)
    }

    pub async fn open(self, driver_ref: &WebDriver) -> WebDriverResult<CustomersPage> {
        driver_ref
            .goto("https://danieldelimata.github.io/sample-page/")
            .await?;
        Ok(self)
    }

    pub async fn set_search_input(self, input: &str) -> WebDriverResult<CustomersPage> {
        let search_input_element = resolve!(self.search_input);
        search_input_element.send_keys(input).await?;
        Ok(self)
    }

    pub async fn set_search_column_drop_down_list_field(
        self,
        value: &str,
    ) -> WebDriverResult<CustomersPage> {
        let dropdown_element = resolve!(self.drop_down);
        SelectElement::new(&dropdown_element)
            .await?
            .select_by_visible_text(value)
            .await?;
        Ok(self)
    }

    pub async fn set_match_case_checkbox_field(
        self,
        value: &str,
    ) -> WebDriverResult<CustomersPage> {
        let case_checkbox_element = resolve!(self.match_case);
        if case_checkbox_element.is_selected().await?.to_string() != value {
            case_checkbox_element.click().await?;
        }
        Ok(self)
    }

    pub async fn clear_button_click(self) -> WebDriverResult<CustomersPage> {
        let clear_button_element = resolve!(self.clear_button);
        clear_button_element.click().await?;
        Ok(self)
    }

    pub async fn get_summary_text(self) -> WebDriverResult<String> {
        let search_summary_element = resolve!(self.summary);
        let result = search_summary_element.text().await?;
        Ok(result)
    }

    pub async fn get_search_input_text(self) -> WebDriverResult<String> {
        sleep(time::Duration::from_secs(1));
        let input_text_element = resolve!(self.search_input);
        let result = input_text_element.text().await?;
        Ok(result)
    }
}
