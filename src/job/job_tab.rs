use leptos::*;
use leptonic::prelude::*;

use crate::job::job_info::JobInfo;

/// Specific implementation of Tab for a job
#[component]
pub fn JobTab(
    /// The job to display in the tab
    job: JobInfo
) -> impl IntoView {
    let label: String = format!("{} ({}-{})", job.company, job.start_date, job.end_date);

    view! {
        <Tab name=job.id.to_string() label=label.into_view()>
            <b>{job.job_title.clone()}</b>
            <br/>
            <ul>
                {job.description.clone().iter()
                    .map(|n| view! { <li>{n}</li>})
                    .collect_view()}
            </ul>
        </Tab>
    }
}