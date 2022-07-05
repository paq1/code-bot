use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

use std::fs::File;

#[derive(PartialEq)]
pub enum BotState {
    Nothing,
    Survey
}

pub struct Handler {pub state: BotState}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        const IMAGE_URL: &str = "assets/femme.gif";
        let femme_cheval = get_image_femme_cheval_singe().await;
        let file = [(&femme_cheval.0, femme_cheval.1.as_str())];

        if msg.content == "!trouvemoiunemeuf" && self.state == BotState::Nothing {
            msg.channel_id
                .send_message(&ctx.http, |create_message| {
                    create_message.files(file);
                    create_message
                })
                .await
                .unwrap();
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

// todo clean ImageFile (le mettre dans une factory ou autre)

struct ImageFile(tokio::fs::File, String);

async fn get_image_femme_cheval_singe() -> ImageFile {
    const IMAGE_URL: &str = "assets/femme.gif";
    let file: tokio::fs::File = tokio::fs::File::open(IMAGE_URL).await.unwrap();
    ImageFile(file, IMAGE_URL.to_string())
}
