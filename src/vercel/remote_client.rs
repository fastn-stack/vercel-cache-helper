#[derive(Debug)]
pub struct RemoteClient {
    token: String,
    team_id: Option<String>,
    user_agent: String,
}

#[allow(dead_code)]
impl RemoteClient {
    pub fn new(token: String, team_id: Option<String>, product: String) -> RemoteClient {
        RemoteClient {
            token,
            team_id,
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
