use leptos::*;
use leptonic::prelude::*;

#[derive(Default)]
#[derive(Debug, Clone)]
pub struct JobCardInfo {
    pub id: u32,
    pub company: String,
    pub job_title: String,
    pub start_date: String,
    pub end_date: String,
    pub description: String,
    // TODO: Content should be like a vec<String> or something
    pub link: String,
}

#[component]
pub fn JobCard(
    job: JobCardInfo
) -> impl IntoView {
    view! {
        <Box style="background: var(--card-background-color); color: var(--std-text-bright);" class="job-card">
            <div><b>{job.company}</b></div>
            <div style="font-size: 14px; padding-left: 10px;"><i>{job.start_date} - {job.end_date}</i></div>
            <div style="font-size: 14px; padding-left: 10px;"><i>{job.job_title}</i></div>
            <br/>
            <p>{job.description}</p>
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