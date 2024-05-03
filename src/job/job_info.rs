#[derive(Default, Debug, Clone)]
pub struct JobInfo {
    pub id: u32,
    pub company: String,
    pub job_title: String,
    pub start_date: String,
    pub end_date: String,
    pub description: Vec<String>,
    pub tech_stack: String,
    // pub link: String,
}
