pub static mut STATE: BotState = BotState::Nothing;

#[derive(PartialEq)]
pub enum BotState {
    Nothing,
    ChoiceSurvey,
    Survey
}