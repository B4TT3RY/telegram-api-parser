pub struct Update {
    pub id: i32,
    pub kind: UpdateKind,
}

pub enum UpdateKind {
    Message,
    EditedMessage,
    ChannelPost,
    EditedChannelPost,
    InlineQuery,
    ChosenInlineResult,
    CallbackQuery,
    ShippingQuery,
    PreCheckoutQuery,
    Poll,
    PollAnswer,
    MyChatMember,
    ChatMember,
    Error,
    Unknown,
}