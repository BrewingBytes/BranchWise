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

    pub fn get_type_(&self) -> &String {
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
    fn from_encoded_data(
        encoded_data: &[u8],
        needs_decoding: bool,
    ) -> Result<Self, GitObjectError> {
        // Decode the data and check if the header is valid
        let decoded_data = if needs_decoding {
            Self::decode_data(encoded_data)?
        } else {
            String::from_utf8_lossy(encoded_data).to_string()
        };

        let data = if needs_decoding {
            Self::check_header_valid_and_get_data(&decoded_data)?.0
        } else {
            &decoded_data
        };

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
        Header::Tag
    }

    fn get_data_string(&self) -> String {
        self.to_string()
    }
}

impl fmt::Display for GitTag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let content = format!(
            "object {}\ntype {}\ntag {}\n{}\n\n{}",
            self.object_hash, self.type_, self.tag, self.tagger, self.message
        );

        write!(f, "{content}")
    }
}

#[cfg(test)]
mod tests {
    use crate::git::git_user::GitUser;
    use std::io::Read;

    use super::*;

    fn mock_git_tagger() -> GitCommitAuthor {
        GitCommitAuthor::new(
            GitUser {
                name: "Test User".to_string(),
                email: "test@example.com".to_string(),
            },
            1234567890,
            "+0000".to_string(),
            GitCommitAuthorType::Tagger,
        )
    }

    fn mock_git_tag() -> GitTag {
        GitTag::new(
            "1234567890abcdef1234567890abcdef12345678",
            "commit",
            "v1.0.0",
            mock_git_tagger(),
            "This is a test tag",
        )
    }

    fn create_encoded_git_tag(
        object_hash: &str,
        type_: &str,
        name: &str,
        tagger: GitCommitAuthor,
        message: &str,
    ) -> Result<Vec<u8>, GitObjectError> {
        let file_content = format!(
            "object {}\ntype {}\ntag {}\n{}\n\n{}",
            object_hash,
            type_,
            name,
            tagger.to_string(),
            message
        );

        let file_to_encode = format!("tag {}\x00{}\n", file_content.len(), file_content);

        let mut zlib = flate2::bufread::ZlibEncoder::new(
            file_to_encode.as_bytes(),
            flate2::Compression::default(),
        );
        let mut encoded_file_content = Vec::new();
        zlib.read_to_end(&mut encoded_file_content)
            .map_err(|_| GitObjectError::DecompressionError)?;

        Ok(encoded_file_content)
    }

    #[test]
    fn test_from_string() {
        let tagger = mock_git_tagger();

        let encoded = create_encoded_git_tag(
            "25723a3e66cd8dcbaf085ed83b86a8007df7ff32",
            "commit",
            "test",
            tagger.clone(),
            "hi",
        )
        .unwrap();

        let git_tag = GitTag::from_encoded_data(&encoded, true).unwrap();
        assert!(git_tag.get_object_hash() == "25723a3e66cd8dcbaf085ed83b86a8007df7ff32");
        assert!(git_tag.get_type_() == "commit");
        assert!(git_tag.get_tag_name() == "test");
        assert!(git_tag.get_tagger() == &tagger);
        assert!(git_tag.get_message() == "hi");
        assert_eq!(git_tag.get_type(), Header::Tag);
    }

    #[test]
    fn test_from_string_invalid() {
        let encoded_file_content = "invalid content".as_bytes();

        let git_tag = GitTag::from_encoded_data(encoded_file_content, true);
        assert!(git_tag.is_err());
    }

    #[test]
    fn test_to_string() {
        let git_tag = mock_git_tag();
        let expected = "object 1234567890abcdef1234567890abcdef12345678\ntype commit\ntag v1.0.0\ntagger Test User <test@example.com> 1234567890 +0000\n\nThis is a test tag";

        assert!(git_tag.to_string() == expected);
    }

    #[test]
    fn test_already_decoded_data() {
        let tag = mock_git_tag();
        let decoded_data = tag.get_data_string() + "\n";

        let git_tag = GitTag::from_encoded_data(decoded_data.as_bytes(), false).unwrap();
        assert_eq!(git_tag.get_hash(), tag.get_hash());
    }

    #[test]
    fn test_serialize() {
        let git_tag = mock_git_tag();
        let serialized = serde_json::to_string(&git_tag).unwrap();
        let expected = "{\"object_hash\":\"1234567890abcdef1234567890abcdef12345678\",\"type_\":\"commit\",\"tag\":\"v1.0.0\",\"tagger\":{\"user\":{\"name\":\"Test User\",\"email\":\"test@example.com\"},\"date_seconds\":1234567890,\"timezone\":\"+0000\",\"type_\":\"Tagger\"},\"message\":\"This is a test tag\"}";

        assert!(serialized == expected);
    }

    #[test]
    fn test_deserialize() {
        let serialized = "{\"object_hash\":\"1234567890abcdef1234567890abcdef12345678\",\"type_\":\"commit\",\"tag\":\"v1.0.0\",\"tagger\":{\"user\":{\"name\":\"Test User\",\"email\":\"test@example.com\"},\"date_seconds\":1234567890,\"timezone\":\"+0000\",\"type_\":\"Tagger\"},\"message\":\"This is a test tag\"}";
        let git_tag: GitTag = serde_json::from_str(serialized).unwrap();

        assert!(git_tag.get_object_hash() == "1234567890abcdef1234567890abcdef12345678");
        assert!(git_tag.get_type_() == "commit");
        assert!(git_tag.get_tag_name() == "v1.0.0");
        assert!(git_tag.get_tagger() == &mock_git_tagger());
        assert!(git_tag.get_message() == "This is a test tag");
    }
}
