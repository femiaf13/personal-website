use leptos::*;
use leptonic::prelude::*;

#[derive(Debug, Clone)]
pub struct JobCardInfo {
    pub id: u32,
    pub company: String,
    pub job_title: String,
    pub start_date: String,
    pub end_date: String,
    // TODO: Content should be like a vec<String> or something
    pub link: String,
}

#[component]
pub fn JobCard(
    job: JobCardInfo
) -> impl IntoView {
    view! {
        <Box style="background: var(--card-background-color)">
            <div><b>{job.company}</b></div>
            <div style="font-size: 14px; padding-left: 10px;">{job.start_date} - {job.end_date}</div>
            <div style="font-size: 14px; padding-left: 10px;">{job.job_title}</div>
            <div>
                // TODO: Make the links right
                <LinkExt href=job.link.clone() target=LinkExtTarget::Blank>
                    <Button on_click=move |_| {}>
                        {job.link}
                    </Button>
                </LinkExt>
            </div>
        </Box>
    }
}