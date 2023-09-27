pub struct ArtifactOptions {
    duration: Option<u64>,
    tag: Option<String>,
}

trait RequestHeaders {
    fn get_headers(&self, method: &str, content_len: Option<usize>) -> reqwest::header::HeaderMap;
}

// Define the base struct with common fields and behavior.
pub struct ArtifactBaseRequest {
    token: String,
    url: String,
    user_agent: String,
    options: Option<ArtifactOptions>,
}

impl ArtifactBaseRequest {
    pub fn new(
        token: String,
        url: String,
        user_agent: String,
        options: Option<ArtifactOptions>,
    ) -> Self {
        ArtifactBaseRequest {
            token,
            url,
            user_agent,
            options,
        }
    }
}

// Implement the trait for the base struct.
impl RequestHeaders for ArtifactBaseRequest {
    fn get_headers(&self, method: &str, content_len: Option<usize>) -> reqwest::header::HeaderMap {
        let mut headers = reqwest::header::HeaderMap::new();

        headers.insert(
            reqwest::header::AUTHORIZATION,
            reqwest::header::HeaderValue::from_str(&format!("Bearer {}", self.token)).unwrap(),
        );

        headers.insert(
            reqwest::header::USER_AGENT,
            reqwest::header::HeaderValue::from_str(&self.user_agent).unwrap(),
        );

        if method == "PUT" {
            headers.insert(
                reqwest::header::CONTENT_TYPE,
                reqwest::header::HeaderValue::from_static("application/octet-stream"),
            );

            if let Some(content_len) = content_len {
                headers.insert(
                    reqwest::header::CONTENT_LENGTH,
                    reqwest::header::HeaderValue::from(content_len),
                );
            }

            if let Some(options) = &self.options {
                if let Some(duration) = options.duration {
                    headers.insert(
                        reqwest::header::HeaderName::from_static("x-artifact-duration"),
                        reqwest::header::HeaderValue::from(duration),
                    );
                }

                if let Some(tag) = &options.tag {
                    headers.insert(
                        reqwest::header::HeaderName::from_static("x-artifact-tag"),
                        reqwest::header::HeaderValue::from_str(tag).unwrap(),
                    );
                }
            }
        }

        if let Ok(ci_name) = std::env::var("CI") {
            headers.insert(
                reqwest::header::HeaderName::from_static("x-artifact-client-ci"),
                reqwest::header::HeaderValue::from_str(&ci_name).unwrap(),
            );
        }

        let is_tty = atty::is(atty::Stream::Stdout);
        headers.insert(
            reqwest::header::HeaderName::from_static("x-artifact-client-interactive"),
            reqwest::header::HeaderValue::from_str(if is_tty { "1" } else { "0" }).unwrap(),
        );

        headers
    }
}

pub struct ArtifactPutRequest(pub ArtifactBaseRequest);
pub struct ArtifactGetRequest(pub ArtifactBaseRequest);
pub struct ArtifactExistsRequest(pub ArtifactBaseRequest);

impl ArtifactPutRequest {
    pub async fn stream(
        &mut self,
        artifact: &mut (dyn tokio::io::AsyncRead + Unpin),
    ) -> vercel_cache_helper::Result<reqwest::Response> {
        use tokio::io::AsyncReadExt;
        let client = reqwest::Client::new();
        let mut body: Vec<u8> = vec![];

        artifact.read_to_end(&mut body).await?;

        let headers = self.0.get_headers("PUT", None);

        println!("Headers: {:?}", headers);

        let response = client
            .put(&self.0.url)
            .headers(headers)
            .body(body)
            .send()
            .await?;

        Ok(response)
    }

    pub async fn buffer(
        &mut self,
        artifact: &mut [u8],
        content_len: usize
    ) -> vercel_cache_helper::Result<reqwest::Response> {
        let client = reqwest::Client::new();

        println!("{:?}", &artifact);

        let headers = self.0.get_headers("PUT", Some(content_len));

        println!("Headers: {:?}", headers);

        let response = client
            .put(&self.0.url)
            .headers(headers)
            .body(artifact.to_owned())
            .send()
            .await?;

        Ok(response)
    }
}

impl ArtifactGetRequest {
    pub async fn get(&mut self) -> vercel_cache_helper::Result<reqwest::Response> {
        let client = reqwest::Client::new();

        let response = client
            .get(&self.0.url)
            .headers(self.0.get_headers("GET", None))
            .send()
            .await?;

        Ok(response)
    }
}

impl ArtifactExistsRequest {
    pub async fn send(&mut self) -> vercel_cache_helper::Result<bool> {
        let client = reqwest::Client::new();

        let response = client
            .head(&self.0.url)
            .headers(self.0.get_headers("HEAD", None))
            .send()
            .await?;

        let status = response.status();

        if status.is_success() {
            return Ok(true);
        }

        Ok(false)
    }
}
