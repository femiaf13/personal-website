use leptos::*;
use leptonic::prelude::*;

use crate::job::job_info::JobInfo;

#[component]
pub fn JobTab(
    job: JobInfo
) -> impl IntoView {
    let label: String = format!("{} ({}-{})", job.company, job.start_date, job.end_date);

    view! {
        <Tab name=job.id.to_string() label=label.into_view()>
            "Content of tab 1"
        </Tab>
    }
}