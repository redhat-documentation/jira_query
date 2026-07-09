/*
Copyright 2022 Marek Suchánek <msuchane@redhat.com>

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/

use thiserror::Error;

/// All errors that might occur in this crate.
#[derive(Error, Debug)]
pub enum JiraQueryError {
    #[error("Required issues are missing in the Jira response: {}.", .0.join(", "))]
    MissingIssues(Vec<String>),
    #[error("The Jira query returned no issues.")]
    NoIssues,
    #[error("Error in accessing the Jira REST API.")]
    Request(#[from] reqwest::Error),
    #[error("Authentication failed (HTTP {}): {}. Please check your JIRA_API_KEY and JIRA_USER_EMAIL environment variables and that the token specified in JIRA_API_KEY is owned by the account specified in JIRA_USER_EMAIL.", status, message)]
    AuthenticationFailed { status: u16, message: String },
}
