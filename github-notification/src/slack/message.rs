use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SlackMessageBlock {
    #[serde(rename = "type")]
    pub(crate) block_type: String,
    pub(crate) text: Option<SlackMessageBlockText>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SlackMessageBlockText {
    #[serde(rename = "type")]
    pub(crate) text_type: String,
    pub(crate) text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SlackMessageBlocks {
    #[serde(rename = "type")]
    pub blocks_type: String,
    pub blocks: Vec<SlackMessageBlock>,
}

impl Default for SlackMessageBlocks {
    fn default() -> Self {
        Self::new()
    }
}

impl SlackMessageBlocks {
    fn new() -> Self {
        Self {
            blocks_type: "home".to_string(),
            blocks: vec![],
        }
    }

    fn add_block(&mut self, block: SlackMessageBlock) {
        self.blocks.push(block);
    }

    pub fn add_header_block(&mut self, text: String) {
        let block = SlackMessageBlock {
            block_type: "header".to_string(),
            text: Some(SlackMessageBlockText {
                text_type: "plain_text".to_string(),
                text,
            }),
        };
        self.add_block(block);
    }

    pub fn add_text_block(&mut self, text: String) {
        let block = SlackMessageBlock {
            block_type: "section".to_string(),
            text: Some(SlackMessageBlockText {
                text_type: "mrkdwn".to_string(),
                text,
            }),
        };
        self.add_block(block);
    }
}

#[cfg(test)]
mod tests {
    use crate::slack::message::{SlackMessageBlock, SlackMessageBlockText};

    #[test]
    fn test_slack_message_block_1() {
        let slack_message_block = SlackMessageBlock {
            block_type: "test".to_string(),
            text: None,
        };
        assert_eq!(slack_message_block.block_type, "test");
        assert_eq!(slack_message_block.text, None);
    }

    #[test]
    fn test_slack_message_block_text_1() {
        let slack_message_block_text = SlackMessageBlockText {
            text_type: "test".to_string(),
            text: "test".to_string(),
        };
        assert_eq!(slack_message_block_text.text_type, "test");
        assert_eq!(slack_message_block_text.text, "test");
    }
}
