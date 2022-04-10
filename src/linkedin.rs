use fancy_regex::{Regex};
use rocket::serde::{Serialize};

pub struct LinkedIn;


#[derive(Debug, Clone)]
pub struct ApiError;

pub type Result<T> = std::result::Result<T, ApiError>;

const LINKEDIN_CAREERS_URL : &str = "https://github.com/about/careers";

async fn fetch_careers () -> Result<String> {
    reqwest::get(LINKEDIN_CAREERS_URL)
        .await
        .map_err(|_| ApiError)
        ?.text().await
        .map_err(|_| ApiError)
}

#[derive(Debug, Serialize)]
pub struct JobCareer {
    pub url: String,
    pub name: String
}

fn get_html_section_with_jobs (html: &str) -> &str {
    let re = Regex::new(r"Candidate Privacy Policy(?P<jobs>(.|\n)*)Internships").unwrap();
    let cap = re.captures(&html).unwrap().unwrap();
    cap.name("jobs").unwrap().as_str()
}

fn get_parts_with_urls (section: &str) -> Vec<&str> {
    let re_url = Regex::new(r"<a href=(.|\n)+?(?=a>)").unwrap();

    let filtered : Vec<&str> = section.split("Details js-details-container")
        .filter(|j| j.contains("https://boards.greenhouse.io/github"))
        .collect();
    let list_of_captures = filtered
        .into_iter()
        .flat_map(|el| re_url.captures_iter(el));
    let list_of_jobs = list_of_captures
        .flat_map(|el| el
                .unwrap()
                .iter()
                .map(|el2| el2
                    .unwrap()
                    .as_str()
                ).collect::<Vec<&str>>())
        .collect::<Vec<&str>>();
    
        list_of_jobs
}

fn get_jobs (html: &str) -> Vec<JobCareer> {
    let section_with_jobs = get_html_section_with_jobs(html);

    let jobs = get_parts_with_urls(section_with_jobs);

    let str_regex = "<a href=\"(?P<url>[^\\d]+\\d+)\">(.|\n)+<span>(?P<jobName>[a-zA-Z\\-\\s]+)<\\/span>";
    let re_url_job_name = Regex::new(str_regex).unwrap();

    let urls = jobs
        .into_iter()
        .filter_map(|url| {
            match re_url_job_name.captures(url).unwrap() {
                Some(captured) => Some(JobCareer {
                    url: captured.name("url").unwrap().as_str().to_owned(),
                    name: captured.name("jobName").unwrap().as_str().to_owned(),
                }),
                None => None
            }
        })
        .collect::<Vec<JobCareer>>();
    urls
}

impl LinkedIn {
    pub async fn careers () -> Result<Vec<JobCareer>> {
        let response = fetch_careers().await?;
        Ok(get_jobs(&response))
    }
}