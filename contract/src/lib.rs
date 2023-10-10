use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, AccountId, PanicOnDefault};

// Updated Representation of a project
#[derive(BorshDeserialize, BorshSerialize, Clone)]
pub struct Project {
    name: String,
    description: String,
    link: String,
    verified: bool,
    category: String,
    tags: Vec<String>,
    metadata: String,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct ProjectRegistry {
    owner: AccountId,
    projects: Vec<Project>,
}

impl Default for ProjectRegistry {
    fn default() -> Self {
        Self {
            owner: "social.near".parse().unwrap(),
            projects: Vec::new(),
        }
    }
}

#[near_bindgen]
impl ProjectRegistry {
    // Adds a new project to the registry
    pub fn add_project(
        &mut self,
        name: String,
        description: String,
        link: String,
        verified: bool,
        category: String,
        tags: Vec<String>,
        metadata: String,
    ) {
        self.assert_owner();
        let project = Project {
            name,
            description,
            link,
            verified,
            category,
            tags,
            metadata,
        };
        self.projects.push(project);
    }

    // Lists all the projects
    pub fn list_projects(&self) -> Vec<Project> {
        self.projects.clone()
    }

    // Retrieve project by its index
    pub fn get_project(&self, index: usize) -> Option<Project> {
        self.projects.get(index).cloned()
    }

    // Internal method to check if the caller is the owner
    fn assert_owner(&self) {
        assert_eq!(
            env::predecessor_account_id(),
            self.owner,
            "Only the owner can call this function"
        );
    }
}