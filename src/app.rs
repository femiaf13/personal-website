use leptonic::prelude::*;
use leptos::*;
use leptos_meta::{provide_meta_context, Meta, Stylesheet, Title};
use leptos_router::*;
use leptos_use::use_media_query;

use crate::{
    error_template::{AppError, ErrorTemplate},
    pages::about::About,
    pages::resume::Resume,
    components::nav_contents::NavContents
};
pub const APP_BAR_HEIGHT: Height = Height::Em(3.5);
pub const LOGO_HEIGHT: Height = Height::Em(2.5);

#[derive(Debug, Clone, Copy)]
pub struct AppLayoutContext {
    pub is_large_screen: Signal<bool>,
    pub show_drawer: ReadSignal<bool>,
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    let is_large_screen:Signal<bool> = use_media_query("(min-width: 720px)");
    let (show_drawer, set_show_drawer) = create_signal(false);

    let ctx = AppLayoutContext {
        is_large_screen,
        show_drawer
    };
    provide_context(ctx);

    view! {
        <Meta name="charset" content="UTF-8"/>
        <Meta name="description" content="Portfolio Website in Leptos"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>
        <Meta name="theme-color" content="#e66956"/>

        // <Stylesheet id="leptos" href="/pkg/leptonic-template-ssr.css"/>
        <Stylesheet href="https://fonts.googleapis.com/css?family=Roboto&display=swap"/>

        <Title text="Frank's Portfolio Website"/>

        <Root default_theme=LeptonicTheme::default()>
            <Router fallback=|| {
                let mut outside_errors = Errors::default();
                outside_errors.insert_with_default_key(AppError::NotFound);
                view! { <ErrorTemplate outside_errors/> }
            }>
                <Box style="display: flex; flex-direction: column; align-items: center; min-width: 100%">
                    <AppBar
                        height=APP_BAR_HEIGHT
                        style="z-index: 1; background: var(--brand-color); color: white;"
                    >
                        <img
                            style="width: auto; margin: 0 0;"
                            style:height=move || format!("{}", LOGO_HEIGHT)
                            src="./Computer_Dude_Logo.png"
                        />
                        <Stack
                            orientation=StackOrientation::Horizontal
                            spacing=Size::Em(1.0)
                            style="margin-right: 1em"
                        >
                            <Show
                                when=move || { is_large_screen.get() }
                                // Small screens get the card view
                                fallback=move || view! {
                                    <button style="background: content-box; border: none;" on:click={move |_| set_show_drawer.update(|value| *value = !*value)} >
                                        <Icon class="header-icon" icon=icondata::ChMenuHamburger/>
                                    </button>
                                }
                            >
                                <NavContents vertical=false></NavContents>
                            </Show>
                        </Stack>
                    </AppBar>
                </Box>
                <Routes>
                    <Route path="*" view=|| view! { <Resume/> }/>
                    <Route path="/about" view=|| view! { <About/> }/>
                </Routes>
            </Router>
        </Root>
    }
}
