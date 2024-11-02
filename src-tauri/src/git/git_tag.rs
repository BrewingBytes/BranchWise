use super::{
    git_commit_author::{GitCommitAuthor, GitCommitAuthorType},
    object::{GitObject, Header},
};
use crate::errors::git_object_error::{CommitError, GitObjectError};
use core::fmt;
use serde::{Deserialize, Serialize};

pub enum TagPrefix {
    Object,
    Type,
    Tag,
    Tagger,
    Message,
    Invalid,
}

impl From<&str> for TagPrefix {
    fn from(prefix: &str) -> Self {
        match prefix {
            "object" => TagPrefix::Object,
            "type" => TagPrefix::Type,
            "tag" => TagPrefix::Tag,
            "tagger" => TagPrefix::Tagger,
            "message" => TagPrefix::Message,
            _ => TagPrefix::Invalid,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GitTag {
    object_hash: String,
    type_: String,
    tag: String,
    tagger: GitCommitAuthor,
    message: String,
}

impl GitTag {
    pub fn new(
        object_hash: &str,
        type_: &str,
        tag: &str,
        tagger: GitCommitAuthor,
        message: &str,
    ) -> GitTag {
        GitTag {
            object_hash: object_hash.to_string(),
            type_: type_.to_string(),
            tag: tag.to_string(),
            tagger,
            message: message.to_string(),
        }
    }

    pub fn get_object_hash(&self) -> &String {
        &self.object_hash
    }

    pub fn get_type(&self) -> &String {
        &self.type_
    }

    pub fn get_tag_name(&self) -> &String {
        &self.tag
    }

    pub fn get_tagger(&self) -> &GitCommitAuthor {
        &self.tagger
    }

    pub fn get_message(&self) -> &String {
        &self.message
    }
}

impl GitObject for GitTag {
    /**
     * Create a new GitTag from the encoded data
     *
     * encoded_data: The encoded data to create the GitTag from
     *
     * Returns the GitTag if the encoded data is valid, otherwise an error
     */
    fn from_encoded_data(encoded_data: &[u8]) -> Result<Self, GitObjectError> {
        let decoded_data = Self::decode_data(encoded_data)?;
        let (data, _) = Self::check_header_valid_and_get_data(&decoded_data)?;

        // The data must contain an object hash, a type, a tag name, a tagger and a message
        let mut object = "";
        let mut type_ = "";
        let mut tag = "";
        let mut tagger = None;
        let mut message = "";

        // Remove the last newline character
        let mut data = &data[..data.len() - 1];
        while !data.is_empty() {
            // Get the next line
            let (line, remaining_data) =
                data.split_once('\n')
                    .ok_or(GitObjectError::InvalidCommitFile(
                        CommitError::InvalidContent,
                    ))?;

            // Get the prefix of the line, which is the first word
            // If there is none, use "message" as the prefix
            let prefix = line.split_whitespace().next().unwrap_or("message");

            let value = line
                .strip_prefix((String::from(prefix) + " ").as_str())
                .unwrap_or(remaining_data);

            // Match the prefix and assign the value to the correct field
            // If the prefix is invalid, return an error
            // If the prefix is Author or Committer, parse the value into a GitCommitAuthor
            match TagPrefix::from(prefix) {
                TagPrefix::Object => object = value,
                TagPrefix::Type => type_ = value,
                TagPrefix::Tag => tag = value,
                TagPrefix::Tagger => {
                    tagger = Some(GitCommitAuthor::from_string(
                        value,
                        GitCommitAuthorType::Tagger,
                    )?)
                }
                TagPrefix::Message => {
                    message = value;
                    break;
                }
                TagPrefix::Invalid => {
                    return Err(GitObjectError::InvalidCommitFile(
                        CommitError::InvalidContent,
                    ));
                }
            }

            data = remaining_data;
        }

        // Check that the author and committer are valid
        let tagger = tagger.ok_or(GitObjectError::InvalidCommitFile(
            CommitError::InvalidHeader,
        ))?;

        Ok(GitTag::new(object, type_, tag, tagger, message))
    }

    fn get_type(&self) -> Header {
        Header::Commit
    }

    fn get_data_string(&self) -> String {
        self.to_string()
    }
}

impl fmt::Display for GitTag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let content = format!(
            "object {}\ntype {}\ntag {}\ntagger {}\n\n{}",
            self.object_hash,
            self.type_,
            self.tag,
            self.tagger.to_string(),
            self.message
        );

        write!(f, "{}", content)
    }
}
