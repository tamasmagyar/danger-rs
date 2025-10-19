use gitlab::Gitlab;
use gitlab::api::projects::merge_requests::notes::CreateMergeRequestNote;
use gitlab::api::Query;
use std::env;

pub struct GitLabClient {
    client: Gitlab,
    project_id: u64,
    merge_request_iid: u64,
}

impl GitLabClient {
    pub fn new(project_id: &str, merge_request_iid: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let api_token = env::var("DANGER_GITLAB_API_TOKEN")
            .expect("DANGER_GITLAB_API_TOKEN environment variable is required");
        
        let base_url = env::var("CI_SERVER_URL")
            .unwrap_or_else(|_| "https://gitlab.com".to_string());

        let client = Gitlab::new(&base_url, &api_token)
            .map_err(|e| format!("Failed to create GitLab client: {}", e))?;

        Ok(Self {
            client,
            project_id: project_id.parse()
                .map_err(|e| format!("Invalid project ID '{}': {}", project_id, e))?,
            merge_request_iid: merge_request_iid.parse()
                .map_err(|e| format!("Invalid MR IID '{}': {}", merge_request_iid, e))?,
        })
    }

    pub async fn create_note(&self, body: &str) -> Result<(), Box<dyn std::error::Error>> {
        let endpoint = CreateMergeRequestNote::builder()
            .project(self.project_id)
            .merge_request(self.merge_request_iid)
            .body(body)
            .build()
            .map_err(|e| format!("Failed to build note endpoint: {}", e))?;

        endpoint.query(&self.client)
            .map_err(|e| format!("Failed to create note: {}", e))?;
        
        println!("✅ Successfully created GitLab note: {}", body);
        Ok(())
    }
}