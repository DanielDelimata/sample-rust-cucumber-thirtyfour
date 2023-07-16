use cucumber::{given, then, when};
use thirtyfour::prelude::WebDriverResult;
use thirtyfour_cucumber_example::customerspage::CustomersPage;

use super::super::Context;

#[given("the user is on the page")]
pub(crate) async fn the_user_is_on_the_page(context: &mut Context) -> WebDriverResult<()> {
    context
        .driver
        .goto("https://danieldelimata.github.io/sample-page/")
        .await?;

    let customers_page = CustomersPage::from_driver_ref(&context.driver).await?;
    context.search_summary_at_very_beginning = customers_page.get_summary_text().await?;
    Ok(())
}

#[when(expr = "the user enters the value {string} in the text-input")]
pub(crate) async fn the_user_enters_the_value_in_the_text_input(
    context: &mut Context,
    value: String,
) -> WebDriverResult<()> {
    CustomersPage::from_driver_ref(&context.driver)
        .await?
        .set_search_input(&value)
        .await?;
    Ok(())
}

#[when(expr = "the user selects value {string} in the drop-down")]
pub(crate) async fn the_user_selects_value_in_the_drop_down(
    context: &mut Context,
    value: String,
) -> WebDriverResult<()> {
    CustomersPage::from_driver_ref(&context.driver)
        .await?
        .set_search_column_drop_down_list_field(&value)
        .await?;
    Ok(())
}

#[when(expr = "the user sets case sensitivity switch to {string}")]
pub(crate) async fn the_user_sets_case_sensitivity_switch_to(
    context: &mut Context,
    value: String,
) -> WebDriverResult<()> {
    CustomersPage::from_driver_ref(&context.driver)
        .await?
        .set_match_case_checkbox_field(&value)
        .await?;
    Ok(())
}

#[then(expr = "the user should see the following result summary {string}")]
pub(crate) async fn the_user_should_see_the_following_result_summary(
    context: &mut Context,
    value: String,
) -> WebDriverResult<()> {
    assert_eq!(
        CustomersPage::from_driver_ref(&context.driver)
            .await?
            .get_summary_text()
            .await?,
        value
    );
    Ok(())
}

#[when(expr = "the user clears filters")]
pub(crate) async fn the_user_clears_filters(context: &mut Context) -> WebDriverResult<()> {
    CustomersPage::from_driver_ref(&context.driver)
        .await?
        .clear_button_click()
        .await?;
    Ok(())
}

#[then(expr = "the user should see that search criteria are cleared")]
pub(crate) async fn the_user_should_see_that_search_criteria_are_cleared(
    context: &mut Context,
) -> WebDriverResult<()> {
    assert_eq!(
        CustomersPage::from_driver_ref(&context.driver)
            .await?
            .get_search_input_text()
            .await?,
        ""
    );
    Ok(())
}

#[then(expr = "the user should see that the search result summary is as in the very beginning")]
pub(crate) async fn the_user_should_see_that_the_search_result_summary_is_as_in_the_very_beginning(
    context: &mut Context,
) -> WebDriverResult<()> {
    assert_eq!(
        CustomersPage::from_driver_ref(&context.driver)
            .await?
            .get_summary_text()
            .await?,
        context.search_summary_at_very_beginning
    );
    Ok(())
}
