use leptonic::prelude::*;
use leptos::*;

use crate::job::job_info::JobInfo;

#[component]
pub fn JobCard(
    /// The job to display in the tab
    job: JobInfo,
) -> impl IntoView {
    view! {
        <Box style="background: var(--card-background-color); color: var(--std-text-bright);" class="job-card">
            <div><b>{job.company}</b></div>
            <div style="font-size: 14px; padding-left: 10px;"><i>{job.start_date} - {job.end_date}</i></div>
            <div style="font-size: 14px; padding-left: 10px;"><i>{job.job_title}</i></div>
            <br/>
            // <p>{job.description}</p>
            <ul>
                {job.description.clone().iter()
                    .map(|n| view! { <li>{n}</li>})
                    .collect_view()}
            </ul>
            <p>{job.tech_stack}</p>
            // <div>
            //     // TODO: Make the links right
            //     <LinkExt href=job.link.clone() target=LinkExtTarget::Blank>
            //         <Button on_click=move |_| {}>
            //             {job.link}
            //         </Button>
            //     </LinkExt>
            // </div>
        </Box>
    }
}
