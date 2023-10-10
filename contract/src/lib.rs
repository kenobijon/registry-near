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
    admin: AccountId,
    projects: Vec<Project>,
}

impl Default for ProjectRegistry {
    fn default() -> Self {
        Self {
            owner: "social.near".parse().unwrap(),
            admin: "social.near".parse().unwrap(),
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

    pub fn update_project(
        &mut self,
        index: usize,
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
        self.projects[index] = project;
    }

    // pub fn bulk_add_projects(&mut self, projects: Vec<Project>) {
    //     self.assert_owner();
    //     self.projects.extend(projects);
    // }

    pub fn verify_project(&mut self, index: usize) {
        self.assert_admin();
        self.projects[index].verified = true;
    }

    pub fn verify_projects(&mut self, indexes: Vec<usize>) {
        self.assert_admin();
        for index in indexes {
            self.projects[index].verified = true;
        }
    }

    // Lists all the projects
    pub fn list_projects(&self) -> Vec<Project> {
        self.projects.clone()
    }

    // Retrieve project by its index
    pub fn get_project(&self, index: usize) -> Option<Project> {
        self.projects.get(index).cloned()
    }

    pub fn remove_project(&mut self, index: usize) {
        self.assert_admin();
        self.projects.remove(index);
    }

    // Internal method to check if the caller is the owner
    fn assert_owner(&self) {
        assert_eq!(
            env::predecessor_account_id(),
            self.owner,
            "Only the owner can call this function"
        );
    }

    // Internal method to check if the caller is the owner
    fn assert_admin(&self) {
        assert_eq!(
            env::predecessor_account_id(),
            self.admin,
            "Only the admin can call this function"
        );
    }
}
