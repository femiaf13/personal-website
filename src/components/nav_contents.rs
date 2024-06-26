use leptonic::prelude::*;
use leptos::*;

/// The links/icons that go in website navigation
#[component]
pub fn NavContents(
    vertical: bool
) -> impl IntoView {

    view! {
        <Show
            when=move || !vertical
            fallback=move || view! {
                <Link href="/">
                    <h3>About</h3>
                </Link>
                <Link href="/resume">
                    <h3>Resume</h3>
                </Link>
                <LinkExt
                    href="https://www.linkedin.com/in/frank-femia-iii"
                    target=LinkExtTarget::Blank
                    style="margin-top: 15px;"
                >
                    <Icon
                        id="linkedin-icon"
                        class="header-icon"
                        icon=icondata::BsLinkedin
                    />
                </LinkExt>
                <LinkExt 
                    href="https://github.com/femiaf13" 
                    target=LinkExtTarget::Blank
                    style="margin-top: 15px;"
                >
                    <Icon id="github-icon" class="header-icon" icon=icondata::BsGithub/>
                </LinkExt>
            }
        >     
            <Link href="/">
                <h3>About</h3>
            </Link>
            <Link href="/resume">
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
            <LinkExt 
                href="https://github.com/femiaf13" 
                target=LinkExtTarget::Blank
            >
                <Icon id="github-icon" class="header-icon" icon=icondata::BsGithub/>
            </LinkExt>
        </Show>
    }
}