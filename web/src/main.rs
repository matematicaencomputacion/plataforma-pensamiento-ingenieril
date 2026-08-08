//! IngenierIA web shell — Leptos CSR targeting the Go API on :8080.

use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;

mod api;

use api::API_BASE_URL;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! { <App /> }
    });
}

/// Root application shell (CSR).
#[component]
fn App() -> impl IntoView {
    view! {
        <Router>
            <div class="shell">
                <header class="shell__header">
                    <p class="shell__brand">"IngenierIA"</p>
                    <p class="shell__meta">"Leptos CSR · Go API"</p>
                </header>
                <main class="shell__main">
                    <Routes fallback=|| {
                        view! { <p>"Ruta no encontrada"</p> }
                    }>
                        <Route path=path!("/") view=HomePage />
                    </Routes>
                </main>
            </div>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    view! {
        <section class="hero">
            <p class="hero__eyebrow">"Pensamiento ingenieril"</p>
            <h1 class="hero__title">"IngenierIA"</h1>
            <p class="hero__support">
                "Scaffold Leptos CSR listo. El backend Go permanece en "
                <code>{API_BASE_URL}</code>
                " — Qwik en frontend/ intacto hasta el cutover."
            </p>
            <p class="hero__api">{format!("API_BASE_URL = {API_BASE_URL}")}</p>
        </section>
    }
}
