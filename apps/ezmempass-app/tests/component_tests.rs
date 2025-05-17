// apps/ezmempass-app/tests/component_tests.rs

/*
 * Tests for the core Leptos UI components
 */

use ezmempass_app::components::password_generator::PasswordGenerator;
use leptos::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[allow(dead_code)]
#[wasm_bindgen_test]
fn password_generator_renders() {
    // 1.  Grab the document
    let document = document(); // <- from leptos::prelude

    // 2.  Set up a mount target
    let test_container = document.create_element("div").unwrap();
    test_container.set_id("test-container");
    document
        .body()
        .unwrap()
        .append_child(&test_container)
        .unwrap();

    // 3.  Mount the component (pass a real HtmlElement)
    let _ = mount_to(
        test_container
            .clone()
            .dyn_into::<web_sys::HtmlElement>()
            .unwrap(),
        || view! { <PasswordGenerator /> },
    );

    // 4.  Assertions
    let password_display = document.query_selector(".password-text").unwrap();
    assert!(
        password_display.is_some(),
        "Password text element should be rendered"
    );

    let generator_options = document.query_selector(".generator-options").unwrap();
    assert!(
        generator_options.is_some(),
        "Generator options should be rendered"
    );

    let buttons = document.query_selector_all("button").unwrap();
    assert!(buttons.length() >= 2, "Should render at least two buttons");
}
