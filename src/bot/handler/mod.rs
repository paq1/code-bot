use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

#[derive(PartialEq)]
pub enum BotState {
    Nothing,
    ChoiceSurvey,
    Survey
}

pub struct Handler {pub state: BotState}

#[async_trait]
impl EventHandler for Handler {
    // todo rendre immutable
    async fn message(&mut self, ctx: Context, msg: Message) {
        trouve_moi_une_meuf(&ctx, &msg, &self.state).await;
        choose_survey(&ctx, &msg, &mut self.state).await;
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

async fn choose_survey(ctx: &Context, msg: &Message, state: &mut BotState) {
    if *state == BotState::Nothing && msg.content == "!survey" {
        *state = BotState::ChoiceSurvey;
        msg.channel_id.send_message(&ctx.http, |create_message| {
            create_message.content("choisir un questionnaire : ")
        }).await.unwrap();
    }
}

async fn survey_test(ctx: &Context, msg: &Message, state: &BotState) {

}

// todo clean ImageFile (le mettre dans une factory ou autre)
struct ImageFile(tokio::fs::File, String);

async fn trouve_moi_une_meuf(ctx: &Context, msg: &Message, state: &BotState) {
    let femme_cheval = get_image_femme_cheval_singe().await;
    let file = [(&femme_cheval.0, femme_cheval.1.as_str())];

    if msg.content == "!trouvemoiunemeuf" && *state == BotState::Nothing {
        msg.channel_id
            .send_message(&ctx.http, |create_message| {
                create_message.files(file);
                create_message
            })
            .await
            .unwrap();
    }
}

async fn get_image_femme_cheval_singe() -> ImageFile {
    const IMAGE_URL: &str = "assets/femme.gif";
    let file: tokio::fs::File = tokio::fs::File::open(IMAGE_URL).await.unwrap();
    ImageFile(file, IMAGE_URL.to_string())
}
