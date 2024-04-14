use leptonic::prelude::*;
use leptos::*;

use crate::job_card::job_card::*;

#[component]
pub fn Welcome() -> impl IntoView {
    let (resume, set_resume) = create_signal::<Vec<JobCardInfo>>(Vec::new());

    set_resume.update(move |resume|{
        resume.push(JobCardInfo {
            id: 0,
            company: String::from("Precision Optical Technologies"),
            job_title: String::from("Senior Software Engineer"),
            start_date: String::from("10/22"),
            end_date: String::from("Present"),
            // TODO: Make the links right
            link: String::from("https://valeen.rocks"),

        })
    });

    set_resume.update(move |resume|{
        resume.push(JobCardInfo {
            id: 1,
            company: String::from("Datto"),
            job_title: String::from("Software Engineer 2"),
            start_date: String::from("1/22"),
            end_date: String::from("10/22"),
            // TODO: Make the links right
            link: String::from("https://valeen.rocks"),

        })
    });
    // TODO: Rest of my jobs and also loop more elegantly maybe?

    view! {
        <Box style="display: flex; flex-direction: column; flex-grow: 1; align-items: center; padding: 1em; min-height: fit-content; min-width: 100%; background: linear-gradient(to bottom, #023788, 70%, #FF4365);">
            <img style="width: auto; margin: 0 0;" style:height="200px" src="./synth-sun.svg"/>
            <For
                // a function that returns the items we're iterating over; a signal is fine
                each=move || resume.get().into_iter()
                // a unique key for each item
                key=|job| job.id.clone()
                // renders each item to a view
                children=move |job| view! {
                    <Card><JobCard job=job/></Card>
                }
            />
        </Box>
    }
}
