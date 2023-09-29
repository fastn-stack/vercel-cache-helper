use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct RemoteClient {
    token: String,
    team_id: Option<String>,
    product: String,
    user_agent: String,
}

#[allow(dead_code)]
impl RemoteClient {
    pub fn new(token: String, team_id: Option<String>, product: String) -> RemoteClient {
        RemoteClient {
            token,
            team_id,
            product: product.clone(),
            user_agent: vercel_cache_helper::vercel::utils::get_user_agent(product.as_str()),
        }
    }

    fn get_remote_cache_endpoint_url(&self, hash: String) -> vercel_cache_helper::Result<String> {
        if hash.contains('/') {
            return Err(vercel_cache_helper::Error::InvalidInput(
                "Invalid hash: Cannot contain '/'".to_string(),
            ));
        }
        let params = if let Some(team_id) = &self.team_id {
            format!("?teamId={}", team_id)
        } else {
            "".to_string()
        };
        Ok(format!(
            "{}/{}{}",
            super::constants::REMOTE_CACHE_ENDPOINT,
            hash,
            params
        ))
    }

    fn get_project_env_endpoint_url(&self) -> vercel_cache_helper::Result<String> {
        let params = if let Some(team_id) = &self.team_id {
            format!("?teamId={}", team_id)
        } else {
            "".to_string()
        };
        Ok(format!(
            "{}/{}/env/{}",
            super::constants::REMOTE_PROJECT_ENDPOINT,
            &self.product,
            params
        ))
    }

    pub async fn set_env_var(
        &self,
        key: String,
        value: String,
    ) -> vercel_cache_helper::Result<reqwest::Response> {
        let client = reqwest::Client::new();

        let response = client
            .post(&self.get_project_env_endpoint_url()?)
            .bearer_auth(&self.token)
            .json(&EnvVarData {
                key,
                value,
                data_type: "plain".to_string(),
                target: vec!["production".to_string(), "preview".to_string()],
                git_branch: None,
                comment: "Vercel cache helper generated hash".to_string(),
            })
            .send()
            .await?;

        Ok(response)
    }

    pub fn get(
        &self,
        hash: String,
        options: Option<vercel_cache_helper::vercel::artifact::ArtifactOptions>,
    ) -> vercel_cache_helper::Result<vercel_cache_helper::vercel::artifact::ArtifactGetRequest>
    {
        Ok(vercel_cache_helper::vercel::artifact::ArtifactGetRequest(
            vercel_cache_helper::vercel::artifact::ArtifactBaseRequest::new(
                self.token.to_string(),
                self.get_remote_cache_endpoint_url(hash)?.clone(),
                self.user_agent.to_string(),
                options,
            ),
        ))
    }

    pub fn put(
        &self,
        hash: String,
        options: Option<vercel_cache_helper::vercel::artifact::ArtifactOptions>,
    ) -> vercel_cache_helper::Result<vercel_cache_helper::vercel::artifact::ArtifactPutRequest>
    {
        Ok(vercel_cache_helper::vercel::artifact::ArtifactPutRequest(
            vercel_cache_helper::vercel::artifact::ArtifactBaseRequest::new(
                self.token.to_string(),
                self.get_remote_cache_endpoint_url(hash)?.clone(),
                self.user_agent.to_string(),
                options,
            ),
        ))
    }

    pub fn exists(
        &self,
        hash: String,
        options: Option<vercel_cache_helper::vercel::artifact::ArtifactOptions>,
    ) -> vercel_cache_helper::Result<vercel_cache_helper::vercel::artifact::ArtifactExistsRequest>
    {
        Ok(
            vercel_cache_helper::vercel::artifact::ArtifactExistsRequest(
                vercel_cache_helper::vercel::artifact::ArtifactBaseRequest::new(
                    self.token.to_string(),
                    self.get_remote_cache_endpoint_url(hash)?.clone(),
                    self.user_agent.to_string(),
                    options,
                ),
            ),
        )
    }
}

#[derive(Serialize, Deserialize)]
struct EnvVarData {
    key: String,
    value: String,
    #[serde(rename = "type")]
    data_type: String,
    target: Vec<String>,
    #[serde(rename = "gitBranch")]
    git_branch: Option<String>,
    comment: String,
}
