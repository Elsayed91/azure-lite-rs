//! Mock HTTP client for testing without hitting real Azure APIs.

use crate::{AzureError, Result};
use serde_json::Value;
use std::sync::{Arc, Mutex};

/// Mock HTTP client for testing
pub struct MockClient {
    expectations: Arc<Mutex<Vec<Expectation>>>,
    call_history: Arc<Mutex<Vec<Call>>>,
}

struct Expectation {
    method: String,
    path_matcher: PathMatcher,
    responses: Vec<Response>,
    response_index: usize,
    times: ExpectedTimes,
    called: usize,
}

enum PathMatcher {
    #[allow(dead_code)]
    Exact(String),
    StartsWith(String),
}

#[derive(Clone)]
enum Response {
    Json(Value),
    Bytes(Vec<u8>),
    Error(AzureError),
}

enum ExpectedTimes {
    Once,
    Times(usize),
    AtLeast(usize),
}

#[derive(Debug)]
#[allow(dead_code)]
struct Call {
    method: String,
    path: String,
    body: Option<Value>,
}

impl Default for MockClient {
    fn default() -> Self {
        Self::new()
    }
}

impl MockClient {
    /// Create a new mock client
    pub fn new() -> Self {
        Self {
            expectations: Arc::new(Mutex::new(Vec::new())),
            call_history: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Expect a GET request
    pub fn expect_get(&mut self, path: &str) -> ExpectationBuilder<'_> {
        ExpectationBuilder::new(self, "GET", path)
    }

    /// Expect a POST request
    pub fn expect_post(&mut self, path: &str) -> ExpectationBuilder<'_> {
        ExpectationBuilder::new(self, "POST", path)
    }

    /// Expect a DELETE request
    pub fn expect_delete(&mut self, path: &str) -> ExpectationBuilder<'_> {
        ExpectationBuilder::new(self, "DELETE", path)
    }

    /// Expect a PUT request
    pub fn expect_put(&mut self, path: &str) -> ExpectationBuilder<'_> {
        ExpectationBuilder::new(self, "PUT", path)
    }

    /// Expect a PATCH request
    pub fn expect_patch(&mut self, path: &str) -> ExpectationBuilder<'_> {
        ExpectationBuilder::new(self, "PATCH", path)
    }

    pub(crate) async fn execute(
        &self,
        method: &str,
        url: &str,
        _body: Option<&Value>,
    ) -> Result<Vec<u8>> {
        let path = if url.starts_with("http") {
            if let Some(pos) = url.find("://") {
                if let Some(slash_pos) = url[pos + 3..].find('/') {
                    &url[pos + 3 + slash_pos..]
                } else {
                    "/"
                }
            } else {
                url
            }
        } else {
            url
        };

        // Record call
        self.call_history.lock().unwrap().push(Call {
            method: method.to_string(),
            path: path.to_string(),
            body: None,
        });

        // Find matching expectation
        let mut expectations = self.expectations.lock().unwrap();

        for expectation in expectations.iter_mut() {
            if expectation.method == method && expectation.matches_path(path) {
                expectation.called += 1;

                let response = if expectation.responses.len() == 1 {
                    &expectation.responses[0]
                } else {
                    let idx = expectation
                        .response_index
                        .min(expectation.responses.len() - 1);
                    expectation.response_index += 1;
                    &expectation.responses[idx]
                };

                return match response {
                    Response::Json(value) => Ok(serde_json::to_vec(value).unwrap()),
                    Response::Bytes(data) => Ok(data.clone()),
                    Response::Error(err) => Err(err.clone()),
                };
            }
        }

        panic!(
            "Unexpected call: {} {}\nCall history:\n{:#?}",
            method,
            path,
            self.call_history.lock().unwrap()
        );
    }
}

impl Drop for MockClient {
    fn drop(&mut self) {
        let expectations = self.expectations.lock().unwrap();
        for exp in expectations.iter() {
            match exp.times {
                ExpectedTimes::Once if exp.called != 1 => {
                    panic!(
                        "Expected {} {} to be called once, called {} times",
                        exp.method,
                        exp.path_display(),
                        exp.called
                    );
                }
                ExpectedTimes::Times(n) if exp.called != n => {
                    panic!(
                        "Expected {} {} to be called {} times, called {} times",
                        exp.method,
                        exp.path_display(),
                        n,
                        exp.called
                    );
                }
                ExpectedTimes::AtLeast(n) if exp.called < n => {
                    panic!(
                        "Expected {} {} to be called at least {} times, called {} times",
                        exp.method,
                        exp.path_display(),
                        n,
                        exp.called
                    );
                }
                _ => {}
            }
        }
    }
}

/// Builder for setting up expectations
pub struct ExpectationBuilder<'a> {
    client: &'a mut MockClient,
    method: String,
    path: String,
    responses: Vec<Response>,
    times: ExpectedTimes,
}

impl<'a> ExpectationBuilder<'a> {
    fn new(client: &'a mut MockClient, method: &str, path: &str) -> Self {
        Self {
            client,
            method: method.to_string(),
            path: path.to_string(),
            responses: Vec::new(),
            times: ExpectedTimes::Once,
        }
    }

    /// Set the response to return JSON
    pub fn returning_json(mut self, value: Value) -> Self {
        self.responses.push(Response::Json(value));
        self
    }

    /// Set the response to return raw bytes.
    pub fn returning_bytes(mut self, data: Vec<u8>) -> Self {
        self.responses.push(Response::Bytes(data));
        self
    }

    /// Set multiple sequential responses
    pub fn returning_json_sequence(mut self, values: Vec<Value>) -> Self {
        for value in values {
            self.responses.push(Response::Json(value));
        }
        self
    }

    /// Set the response to return an error
    pub fn returning_error(mut self, error: AzureError) -> Self {
        self.responses.push(Response::Error(error));
        self
    }

    /// Set the number of times this expectation should be called
    pub fn times(mut self, n: usize) -> Self {
        self.times = ExpectedTimes::Times(n);
        self
    }

    /// Set the minimum number of times this expectation should be called
    pub fn at_least(mut self, n: usize) -> Self {
        self.times = ExpectedTimes::AtLeast(n);
        self
    }
}

impl Drop for ExpectationBuilder<'_> {
    fn drop(&mut self) {
        if self.responses.is_empty() {
            panic!(
                "No response set for expectation: {} {}",
                self.method, self.path
            );
        }

        let expectation = Expectation {
            method: self.method.clone(),
            path_matcher: PathMatcher::StartsWith(self.path.clone()),
            responses: std::mem::take(&mut self.responses),
            response_index: 0,
            times: std::mem::replace(&mut self.times, ExpectedTimes::Once),
            called: 0,
        };

        self.client.expectations.lock().unwrap().push(expectation);
    }
}

impl Expectation {
    fn matches_path(&self, path: &str) -> bool {
        match &self.path_matcher {
            PathMatcher::Exact(p) => path == p,
            PathMatcher::StartsWith(p) => path.starts_with(p.as_str()),
        }
    }

    fn path_display(&self) -> String {
        match &self.path_matcher {
            PathMatcher::Exact(p) => p.clone(),
            PathMatcher::StartsWith(p) => format!("{}*", p),
        }
    }
}
