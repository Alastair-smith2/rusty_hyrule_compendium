//! The compendium error representation
use thiserror::Error;

/// Possible errors while retrieving requested data
#[derive(Error, Debug)]
pub enum CompendiumError {
    /// An invalid base url exists for the Compendium client
    #[error("Invalid base url of '{0}' provided")]
    InvalidBaseUrl(String),
    /// An error representing a failure in building the url to request the resource's data
    #[error("An error occurred while trying to create the resource path")]
    ErrorConstructingResourceUrl,
    /// An error representing a failure in requesting the data
    #[error("An error in occurred while requesting data")]
    RequestError(#[source] reqwest::Error),
    /// An error representing no data found for the requested resource
    #[error("There was no data found for '{0}'")]
    NoDataFound(String),
    /// An error representing a failure in the API's response
    #[error("There was an unexpected error from the server")]
    ServerError,
    /// An error representing a failure in parsing the API's response
    #[error("There was an error in parsing the response")]
    ResponseParsingError(#[source] reqwest::Error),
}
