use crate::personal::types;

use serde_json::Result;

const PERSONAL_INFOS: &str = include_str!("../../contents/personal.json");

pub struct Generator {
    personal_info: types::PersonalInfo,
}

impl Generator {
    pub fn new() -> Self {
        let result = Self::get_information();
        if let Ok(personal_info) = result {
            return Self { personal_info };
        }
        panic!("Personal JSON file missing or corrupted!");
    }

    fn get_information() -> Result<types::PersonalInfo> {
        let personal_info: types::PersonalInfo = serde_json::from_str(PERSONAL_INFOS)?;
        Ok(personal_info)
    }

    pub fn website(&self) -> &types::Website {
        &self.personal_info.website
    }

    pub fn experiences(&self) -> &Vec<types::Experience> {
        &self.personal_info.experiences
    }

    pub fn projects(&self) -> &Vec<types::Project> {
        &self.personal_info.projects
    }

    pub fn education(&self) -> &Vec<types::Education> {
        &self.personal_info.education
    }

    pub fn certifications(&self) -> &Vec<types::Certification> {
        &self.personal_info.certifications
    }
}

impl Default for Generator {
    fn default() -> Self {
        Self::new()
    }
}
