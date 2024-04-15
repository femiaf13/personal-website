use leptonic::prelude::*;
use leptos::*;
use leptos_use::use_media_query;

use crate::job::job_info::JobInfo;
use crate::job::job_card::*;
use crate::job::job_tab::*;

/// Welcome page displays my resume in a large screen and mobile screen format
#[component]
pub fn Welcome() -> impl IntoView {
    let (resume, set_resume) = create_signal::<Vec<JobInfo>>(Vec::new());
    let is_large_screen = use_media_query("(min-width: 720px)");

    set_resume.update(move |resume| {
        resume.push(JobInfo {
            id: 0,
            company: String::from("Precision Optical Technologies"),
            job_title: String::from("Senior Software Engineer"),
            start_date: String::from("10/22"),
            end_date: String::from("Present"),
            // TODO: Make the links right
            link: String::from("https://valeen.rocks"),
            description: String::from("I've done a ton of work making sure that a new prooduct called OpenPath happens.")
        })
    });

    set_resume.update(move |resume| {
        resume.push(JobInfo {
            id: 1,
            company: String::from("Datto"),
            job_title: String::from("Software Engineer 2"),
            start_date: String::from("1/22"),
            end_date: String::from("10/22"),
            // TODO: Make the links right
            link: String::from("https://valeen.rocks"),
            description: String::from("Contributed to both the backend and frontend of a new product(Datto Secure Edge)")
        })
    });

    set_resume.update(move |resume| {
        resume.push(JobInfo {
            id: 2,
            company: String::from("Precision Optical Technologies"),
            job_title: String::from("Software Engineer 2"),
            start_date: String::from("2/18"),
            end_date: String::from("12/21"),
            // TODO: Make the links right
            link: String::from("https://valeen.rocks"),
            ..Default::default()
        })
    });
    // TODO: Rest of my jobs and also loop more elegantly maybe?
    set_resume.update(move |resume| {
        resume.push(JobInfo {
            id: 3,
            company: String::from("Precision Optical Technologies"),
            job_title: String::from("Software Engineer 2"),
            start_date: String::from("2/18"),
            end_date: String::from("12/21"),
            // TODO: Make the links right
            link: String::from("https://valeen.rocks"),
            ..Default::default()
        })
    });
    set_resume.update(move |resume| {
        resume.push(JobInfo {
            id: 4,
            company: String::from("Precision Optical Technologies"),
            job_title: String::from("Software Engineer 2"),
            start_date: String::from("2/18"),
            end_date: String::from("12/21"),
            // TODO: Make the links right
            link: String::from("https://valeen.rocks"),
            ..Default::default()
        })
    });

    view! {
        <Box style="flex-grow: 1; padding: 1em 1em 1em 1em; min-height: fit-content; min-width: 100%; background: linear-gradient(to bottom, #023788, 70%, #FF4365); z-index: 1;">
            <img style="margin: 0 0;" class="sun" src="./synth-sun.svg"/>
            <h2 style="color: var(--std-text-bright); text-align: center; z-index: 1; position: relative;">Digital Resume</h2>
            <Grid spacing=Size::Em(0.6)>
                <Show
                    when=move || { is_large_screen.get() }
                    // Small screens get the card view
                    fallback=move || view! { 
                        <For
                            // a function that returns the items we're iterating over; a signal is fine
                            each=move || resume.get().into_iter()
                            // a unique key for each item
                            key=|job| job.id.clone()
                            // renders each item to a view
                            children=move |job| view! {
                                <Row>
                                    <Col h_align=ColAlign::Center style="padding-left: 1em;">
                                        <Card><JobCard job=job/></Card>
                                    </Col>
                                </Row>
                            }
                        /> 
                    }
                >
                    <Tabs mount=Mount::Once>
                        <For
                            // a function that returns the items we're iterating over; a signal is fine
                            each=move || resume.get().into_iter()
                            // a unique key for each item
                            key=|job| job.id.clone()
                            // renders each item to a view
                            children=move |job| view! {
                                <JobTab job=job/>
                            }
                        />
                    </Tabs>
                </Show>
            </Grid>
        </Box>
    }
}
