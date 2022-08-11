use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Website {
    pub pre_intro: String,
    pub post_intro: String,
    pub linkedin: String,
    pub github: String,
    pub email: String,
}

#[derive(Serialize, Deserialize)]
pub struct Experience {
    pub title: String,
    pub company: String,
    pub start: String,
    pub end: String,
    pub description: String,
    pub location: Option<String>,
    pub logo: String,
}

#[derive(Serialize, Deserialize)]
pub struct Project {
    pub title: String,
    pub description: String,
    pub link: String,
    pub tags: Vec<String>,
    pub language: String,
}

#[derive(Serialize, Deserialize)]
pub struct Education {
    pub title: String,
    pub subject: String,
    pub start: String,
    pub end: String,
    pub institute: String,
}

#[derive(Serialize, Deserialize)]
pub struct Certification {
    pub title: String,
    pub issuer: String,
    pub link: String,
    pub issued_at: String,
}

#[derive(Serialize, Deserialize)]
pub struct PersonalInfo {
    pub website: Website,
    pub experiences: Vec<Experience>,
    pub projects: Vec<Project>,
    pub education: Vec<Education>,
    pub certifications: Vec<Certification>,
}
