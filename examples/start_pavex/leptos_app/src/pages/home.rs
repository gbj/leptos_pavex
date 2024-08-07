use leptos::prelude::{ClassAttribute, ElementChild};
use leptos::{component, view, IntoView};
use leptos_meta::Title;

/// Renders the home page of your application.
#[component]
fn Home() -> impl IntoView {
    view! {
        <Title text="Leptos Pavex Starter"/>
        <div class="content-section">
            <div class="logo-row">
                <img src="/img/leptos_logo.svg" alt="Leptos Logo" width="300"/>
                <span class="plus">"+"</span>
                <img src="/img/pavex_logo.webp" alt="Pavex Logo" width="300"/>
            </div>
            <h1>"Starter"</h1>

        </div>
    }
}
