use super::message::SlackMessageBlocks;

pub async fn notify_by_slack(webhook_url: String, message_blocks: SlackMessageBlocks) {
    let client = reqwest::Client::new();
    let res = client.post(webhook_url).json(&message_blocks).send().await;
    match res {
        Ok(res) => {
            // if status code is 200, it means success
            if res.status() == 200 {
                println!("Notify by Slack OK");
            } else {
                println!("Notify by Slack Error: {}", res.status());
            }
        }
        Err(err) => {
            println!("Notify by Slack Error: {err}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_notify_by_slack() {
        use httpmock::prelude::*;

        let server = MockServer::start();
        let mock = server.mock(|when, then| {
            when.method("POST")
                .path("/")
                .header("content-type", "application/json");
            then.status(200);
        });

        let mock_webhook_url = format!("http://{}", server.address());
        let mock_message_blocks = SlackMessageBlocks::default();

        notify_by_slack(mock_webhook_url, mock_message_blocks).await;
        mock.assert();
    }
}
