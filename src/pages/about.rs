use leptonic::prelude::*;
use leptos::*;

use crate::{components::nav_contents::NavContents, AppLayoutContext};

/// About page displays stuff about me!
#[component]
pub fn About() -> impl IntoView {
    let ctx = expect_context::<AppLayoutContext>();

    view! {
        // TODO: This should be its own component instead of being copied in 2 places
        <Drawer 
            side=DrawerSide::Right
            shown=Signal::derive( move || ctx.show_drawer.get() && !ctx.is_large_screen.get()) 
            style="padding: 0.25em; overflow: scroll; position: absolute; top: 3.5em; right: 0; background-color: var(--brand-color); border-left: 1px solid gray;"
        >
            <Stack spacing=Size::Em(0.6)>
                <NavContents vertical=true></NavContents>
            </Stack>
        </Drawer>
        
        <Box style="display: flex; flex-direction: column; align-items: center; flex-grow: 1; padding: 1em 1em 1em 1em; min-height: fit-content; min-width: 100%; background: linear-gradient(to bottom, #023788, 70%, #FF4365); color: var(--std-text-bright); z-index: 1;">
            <img style="margin: 0 0;" class="sun" src="./synth-sun.svg"/>
            <H1>About Frank</H1>
            <Grid spacing=Size::Em(0.6)>
                <Row>
                    <Col md=6 sm=6 xs=12 h_align=ColAlign::Center>
                        <p>"Hi! My name is Frank Femia, I'm a Software Engineer living in Rochester, NY."</p>
                    </Col>
                </Row>
                <Row>
                    <Col md=4 sm=4 xs=6 h_align=ColAlign::Center>
                        <Card>
                            <H4>"Things I do for fun"</H4>
                            <ul>
                                <li>"Dungeons and Dragons"</li>
                                <li>"Maintain a homelab"</li>
                                <li>"Read (currently working on Brandon Sanderson)"</li>
                                <li>"Trying to replicate my grandma's cooking"</li>
                                <li>"Basic DiY home improvement"</li>
                                <li>"Warhammer"</li>
                            </ul>
                        </Card>
                    </Col>
                    <Col md=4 sm=4 xs=6 h_align=ColAlign::Center>
                        <Card>
                            <H4>"My homelab (the real reason you're here)"</H4>
                            <ul>
                                <li>"TrueNAS Scale NAS"</li>
                                    <ul>
                                        <li>"6 4TB HDDs setup up as 3 mirrors striped together"</li>
                                    </ul>
                                <li>"An old NUC from 2016"</li>
                                    <ul>
                                        <li>"Runs 2 "<LinkExt href="https://foundryvtt.com/" target=LinkExtTarget::Blank>"Foundry VTT"</LinkExt>" servers"</li>
                                    </ul>
                                <li>"An old laptop repurposed as a server"</li>
                                    <ul>
                                        <li>
                                            "This runs my "
                                            <LinkExt href="https://tailscale.com" target=LinkExtTarget::Blank>tailscale exit node</LinkExt>
                                            and my 
                                            <LinkExt href="https://pi-hole.net" target=LinkExtTarget::Blank>pi-hole server</LinkExt>
                                        </li>
                                    </ul>
                                <li>"The equivalent of a low-end gaming desktop"</li>
                                    <ul>
                                        <li>"This runs my reverse proxy,"
                                            <LinkExt href="https://mealie.io" target=LinkExtTarget::Blank>"a private recipe book"</LinkExt>,
                                            <LinkExt href="https://matrix.org" target=LinkExtTarget::Blank>"a Matrix server"</LinkExt>,
                                            <LinkExt href="https://nextcloud.com/" target=LinkExtTarget::Blank>" and a Nextcloud instance"</LinkExt>
                                        </li>
                                    </ul>
                            </ul>
                        </Card>
                    </Col>
                    <Col md=4 sm=4 xs=6 h_align=ColAlign::Center>
                        <Card>
                            <H4>"Work-related things I know"</H4>
                            <ul>
                                <li>"Javascript/Typescript"</li>
                                <li>"Python"</li>
                                <li>"Docker/Podman"</li>
                                <li>"Bash scripting"</li>
                                <li>"C (but only if I have to)"</li>
                                <li>"Rust but only because of this website"</li>
                            </ul>
                        </Card>
                    </Col>
                </Row>
            </Grid>
            <LinkExt class="bottom" href="https://leptos.dev" target=LinkExtTarget::Blank>
                <span style="padding: 5px;">Made in rust with</span><Icon class="header-icon" icon=icondata::SiLeptos/>
            </LinkExt>
        </Box>
    }
}