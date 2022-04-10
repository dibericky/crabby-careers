use fancy_regex::Regex;
use rocket::serde::{Serialize};

pub struct LinkedIn;


#[derive(Debug, Clone)]
pub struct ApiError;

pub type Result<T> = std::result::Result<T, ApiError>;

// Define our error types. These may be customized for our error handling cases.
// Now we will be able to write our own errors, defer to an underlying error
// implementation, or do something in between.

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

fn get_jobs (html: &str) -> Vec<JobCareer> {
    let re = Regex::new(r"Candidate Privacy Policy(?P<jobs>(.|\n)*)Internships").unwrap();
    let cap = re.captures(&html).unwrap().unwrap();
    let jobs_matched = cap.name("jobs").unwrap().as_str();

    let re_url = Regex::new(r"<a href=(.|\n)+?(?=a>)").unwrap();

    let splitted = jobs_matched
        .split("Details js-details-container")
        .filter(|j| j.contains("https://boards.greenhouse.io/github"))
        .map(|j| re_url.captures_iter(j));

    let mut jobs : Vec<String> = Vec::new();
    for matches in splitted {
        for matched in matches {
            let elem = matched.unwrap().iter().map(|el| el.unwrap().as_str()).collect();
            jobs.push(elem);
        }
    }
    let str_regex = "<a href=\"(?P<url>[^\\d]+\\d+)\">(.|\n)+<span>(?P<jobName>[a-zA-Z\\-\\s]+)<\\/span>";
    let re_url_job_name = Regex::new(str_regex).unwrap();
    // let urls = 
    let mut urls : Vec<JobCareer> = Vec::new();
    for job in jobs {
        if let Some(captured) = re_url_job_name.captures(&job).unwrap() {
            let job_career = JobCareer {
                url: captured.name("url").unwrap().as_str().to_owned(),
                name: captured.name("jobName").unwrap().as_str().to_owned(),
            };
            urls.push(job_career)
        }
    }
    urls
}

impl LinkedIn {
    pub async fn careers () -> Result<Vec<JobCareer>> {
        let response = fetch_careers().await?;
        Ok(get_jobs(&response))
    }
}