use leptonic::prelude::*;
use leptos::*;
use leptos_use::use_media_query;

use crate::job::job_card::*;
use crate::job::job_info::JobInfo;
use crate::job::job_tab::*;

/// Resume page displays my resume in a large screen and mobile screen format
#[component]
pub fn Resume() -> impl IntoView {
    let (resume, set_resume) = create_signal::<Vec<JobInfo>>(Vec::new());
    let is_large_screen = use_media_query("(min-width: 720px)");

    set_resume.update(move |resume| {
        let mut description: Vec<String> = vec![];
        description.push(String::from(
            "Team lead and architect of a greenfield product breaking into an entirely new space for the company"
        ));
        description.push(String::from(
            "Worked with management to define requirements"
        ));
        description.push(String::from(
            "Created a container-based infrastructure for easy development and deployment"
        ));
        description.push(String::from(
            "Maintained a robust end-to-end test suite with the Robot Framework"
        ));
        resume.push(JobInfo {
            id: 0,
            company: String::from("Precision Optical Technologies"),
            job_title: String::from("Senior Software Engineer"),
            start_date: String::from("10/22"),
            end_date: String::from("Present"),
            link: String::from("https://www.precisionot.com/product-category/openpath-pon/"),
            description: description,
            tech_stack: String::from("Tech stack of Angular, Node.js and Python.")
        })
    });

    set_resume.update(move |resume| {
        let mut description: Vec<String> = vec![];
        description.push(String::from(
            "Full-stack engineer working on the launch of a new product(Datto Secure Edge)",
        ));
        description.push(String::from(
            "Delivered alpha and beta releases on-time and received positive feedback",
        ));
        description.push(String::from(
            "Drove incremental change to legacy frontend codebaseS",
        ));
        resume.push(JobInfo {
            id: 1,
            company: String::from("Datto"),
            job_title: String::from("Software Engineer 2"),
            start_date: String::from("1/22"),
            end_date: String::from("10/22"),
            // link: String::from("https://valeen.rocks"),
            description: description,
            tech_stack: String::from("Tech stack of PHP and Javascript."),
            ..Default::default()
        })
    });

    set_resume.update(move |resume| {
        let mut description: Vec<String> = vec![];
        description.push(String::from(
            "One of 3 engineers early in a startup"
        ));
        description.push(String::from(
            "Responsible for firmware, desktop applications, mobile applications, and RESTful backend server powering all of it"
        ));
        description.push(String::from(
            "Regularly communicated with stakeholders to ensure software deliveries met expectations"
        ));
        description.push(String::from(
            "Mentored interns, helping them to become self-sufficient contributors to the team"
        ));
        resume.push(JobInfo {
            id: 2,
            company: String::from("Precision Optical Technologies"),
            job_title: String::from("Senior Software Engineer"),
            start_date: String::from("2/18"),
            end_date: String::from("12/21"),
            // link: String::from("https://valeen.rocks"),
            description: description,
            tech_stack: String::from("Tech stack of Node.js and Python."),
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
