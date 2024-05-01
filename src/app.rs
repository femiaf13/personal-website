use leptonic::prelude::*;
use leptos::*;
use leptos_meta::{provide_meta_context, Meta, Stylesheet, Title};
use leptos_router::*;

use crate::{
    error_template::{AppError, ErrorTemplate},
    pages::resume::Resume,
};
pub const APP_BAR_HEIGHT: Height = Height::Em(3.5);
pub const LOGO_HEIGHT: Height = Height::Em(2.5);

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

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
                view! {
                    <ErrorTemplate outside_errors/>
                }
            }>
                <Box style="display: flex; flex-direction: column; align-items: center; min-width: 100%">
                    <AppBar height=APP_BAR_HEIGHT style="z-index: 1; background: var(--brand-color); color: white;">
                        <img style="width: auto; margin: 0 0;" style:height=move || format!("{}", LOGO_HEIGHT) src="./Computer_Dude_Logo.png"/>
                        <Stack orientation=StackOrientation::Horizontal spacing=Size::Em(1.0) style="margin-right: 1em">
                            
                            <LinkExt href="https://www.linkedin.com/in/frank-femia-iii" target=LinkExtTarget::Blank>
                                <Icon id="linkedin-icon" class="header-icon" icon=icondata::BsLinkedin/>
                            </LinkExt>
                            <LinkExt href="https://github.com/femiaf13" target=LinkExtTarget::Blank>
                                <Icon id="github-icon" class="header-icon" icon=icondata::BsGithub/>
                            </LinkExt>
                        </Stack>
                    </AppBar>
                </Box>
                <Routes>
                    <Route path="*" view=|| view! {
                        <Resume/>
                    }/>
                    // <Route path="/about" view=|| view! {
                    //     About page TBD
                    //     <LinkExt href="https://leptos.dev" target=LinkExtTarget::Blank>
                    //         <span style="padding: 5px;">Made with</span><Icon class="header-icon" icon=icondata::SiLeptos/>
                    //     </LinkExt>
                    // }/>
                </Routes>
            </Router>
        </Root>
    }
}
