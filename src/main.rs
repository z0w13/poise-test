use std::env::var;

use poise::{
    serenity_prelude::{self as serenity},
    FrameworkContext,
};

// Types used by all command functions
type Error = Box<dyn std::error::Error + Send + Sync>;
#[allow(unused)]
type Context<'a> = poise::Context<'a, Data, Error>;

// Custom user data passed to all command functions
pub struct Data {}

#[tokio::main]
async fn main() {
    env_logger::init();

    let token = var("DISCORD_TOKEN")
        .expect("Missing `DISCORD_TOKEN` env var, see README for more information.");
    let intents =
        serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

    let framework = poise::Framework::builder()
        .setup(move |_ctx, _ready, _framework| Box::pin(async move { Ok(Data {}) }))
        .options(poise::FrameworkOptions {
            event_handler: |ctx, event| Box::pin(event_handler(ctx, event)),
            ..Default::default()
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .event_handler(EvtHandler)
        .framework(framework)
        .await;

    client.unwrap().start().await.unwrap();
}

async fn event_handler(
    _ctx: FrameworkContext<'_, Data, Error>,
    event: &serenity::FullEvent,
) -> Result<(), Error> {
    println!("{}", event.snake_case_name());
    Ok(())
}

pub struct EvtHandler;

#[serenity::async_trait]
impl serenity::EventHandler for EvtHandler {
    async fn shard_stage_update(
        &self,
        _ctx: serenity::Context,
        _evt: serenity::ShardStageUpdateEvent,
    ) {
        println!("shard_stage_update")
    }
}
