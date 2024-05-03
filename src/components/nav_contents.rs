use leptonic::prelude::*;
use leptos::*;

/// The links/icons that go in website navigation
#[component]
pub fn NavContents() -> impl IntoView {
    view! {
        <Link href="/about">
            <h3>About</h3>
        </Link>
        <Link href="/">
            <h3>Resume</h3>
        </Link>
        <LinkExt
            href="https://www.linkedin.com/in/frank-femia-iii"
            target=LinkExtTarget::Blank
        >
            <Icon
                id="linkedin-icon"
                class="header-icon"
                icon=icondata::BsLinkedin
            />
        </LinkExt>
        <LinkExt href="https://github.com/femiaf13" target=LinkExtTarget::Blank>
            <Icon id="github-icon" class="header-icon" icon=icondata::BsGithub/>
        </LinkExt>
    }
}