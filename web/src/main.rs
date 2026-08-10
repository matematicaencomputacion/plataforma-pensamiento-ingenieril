//! IngenierIA web shell — Leptos CSR targeting the Go API on :8080.

mod api;
mod auth;
mod components;
mod curriculum;
mod data_types;
mod interop;
mod pages;
mod session;

use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;

use components::{BrandLink, SessionBar};
use pages::{
    ForgotPasswordPage, LandingPage, LearnPage, LoginPage, OnboardingPage, RegisterPage,
    ResetPasswordPage, WorkspacePage,
};
use session::{SessionBootstrap, SessionCtx};

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! { <App /> }
    });
}

/// Root application shell (CSR).
#[component]
fn App() -> impl IntoView {
    let _session = SessionCtx::provide();

    view! {
        <Router>
            <SessionBootstrap />
            <div class="shell">
                <header class="shell__header">
                    <BrandLink class="shell__brand" />
                    <SessionBar />
                </header>
                <main class="shell__main">
                    <Routes fallback=|| {
                        view! { <p class="not-found">"Ruta no encontrada"</p> }
                    }>
                        <Route path=path!("/") view=LandingPage />
                        <Route path=path!("/login") view=LoginPage />
                        <Route path=path!("/register") view=RegisterPage />
                        <Route path=path!("/forgot-password") view=ForgotPasswordPage />
                        <Route path=path!("/reset-password") view=ResetPasswordPage />
                        <Route path=path!("/workspace") view=WorkspacePage />
                        <Route path=path!("/onboarding") view=OnboardingPage />
                        <Route path=path!("/learn") view=LearnPage />
                        <Route path=path!("/learn/:step") view=LearnPage />
                    </Routes>
                </main>
            </div>
        </Router>
    }
}
