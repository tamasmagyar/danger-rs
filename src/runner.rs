use std::env;

#[derive(Debug)]
pub struct DangerContext {
    pub ci_platform: CiPlatform,
    pub pull_request: PullRequest,
    pub repository: Repository,
}

#[derive(Debug)]
pub enum CiPlatform {
    GitHubActions,
    GitLabCI,
    Unknown,
}

#[derive(Debug)]
pub struct PullRequest {
    pub id: String,
    pub base_sha: String,
    pub head_sha: String,
    pub base_ref: String,
    pub head_ref: String,
}

#[derive(Debug)]
pub struct Repository {
    pub owner: String,
    pub name: String,
}

impl DangerContext {
    pub fn detect() -> Result<Self, Box<dyn std::error::Error>> {
        let ci_platform = Self::detect_ci_platform();
        let (pull_request, repository) = Self::extract_ci_info(&ci_platform)?;

        Ok(Self {
            ci_platform,
            pull_request,
            repository,
        })
    }

    fn detect_ci_platform() -> CiPlatform {
        if env::var("GITHUB_ACTIONS").is_ok() {
            CiPlatform::GitHubActions
        } else if env::var("GITLAB_CI").is_ok() {
            CiPlatform::GitLabCI
        } else {
            CiPlatform::Unknown
        }
    }

    fn extract_ci_info(
        platform: &CiPlatform,
    ) -> Result<(PullRequest, Repository), Box<dyn std::error::Error>> {
        match platform {
            CiPlatform::GitHubActions => {
                let owner = env::var("GITHUB_REPOSITORY_OWNER")?;
                let repo = env::var("GITHUB_REPOSITORY")?
                    .split('/')
                    .last()
                    .unwrap()
                    .to_string();
                let pr_id = env::var("GITHUB_EVENT_PATH")
                    .and_then(|path| {
                        let event_content = std::fs::read_to_string(path).unwrap();
                        let event: serde_json::Value =
                            serde_json::from_str(&event_content).unwrap();
                        Ok(event["pull_request"]["number"].to_string())
                    })
                    .unwrap_or_else(|_| "unknown".to_string());

                Ok((
                    PullRequest {
                        id: pr_id,
                        base_sha: env::var("GITHUB_BASE_SHA").unwrap_or_default(),
                        head_sha: env::var("GITHUB_HEAD_SHA").unwrap_or_default(),
                        base_ref: env::var("GITHUB_BASE_REF").unwrap_or_default(),
                        head_ref: env::var("GITHUB_HEAD_REF").unwrap_or_default(),
                    },
                    Repository { owner, name: repo },
                ))
            }
            _ => {
                // Fallback for other CI platforms or local development
                Ok((
                    PullRequest {
                        id: "local".to_string(),
                        base_sha: "".to_string(),
                        head_sha: "".to_string(),
                        base_ref: "".to_string(),
                        head_ref: "".to_string(),
                    },
                    Repository {
                        owner: "local".to_string(),
                        name: "repo".to_string(),
                    },
                ))
            }
        }
    }
}
