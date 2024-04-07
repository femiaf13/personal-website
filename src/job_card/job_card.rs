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
        <div class="card w-50">
            <div class="card-header">{job.start_date} - {job.end_date}</div>
            <div class="card-body">
                <h5 class="card-title">{job.company}</h5>
                <h6 class="card-subtitle mb-2 text-muted">{job.job_title}</h6>
                // TODO: Make the links right
                <LinkExt href=job.link.clone() target=LinkExtTarget::Blank>
                    <Button on_click=move |_| {}>
                        {job.link}
                    </Button>
                </LinkExt>
            </div>
        </div>
    }
}