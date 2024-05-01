use leptonic::prelude::*;
use leptos::*;

/// About page displays stuff about me!
#[component]
pub fn About() -> impl IntoView {
    // let is_large_screen = use_media_query("(min-width: 720px)");

    view! {
        <Box style="display: flex; flex-direction: column; align-items: center; flex-grow: 1; padding: 1em 1em 1em 1em; min-height: fit-content; min-width: 100%; background: linear-gradient(to bottom, #023788, 70%, #FF4365); color: var(--std-text-bright); z-index: 1;">
            <img style="margin: 0 0;" class="sun" src="./synth-sun.svg"/>
            <H1>About Me</H1>
            <LinkExt class="bottom" href="https://leptos.dev" target=LinkExtTarget::Blank>
                <span style="padding: 5px;">Made with</span><Icon class="header-icon" icon=icondata::SiLeptos/>
            </LinkExt>
        </Box>
    }
}