use crate::env::Vars;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {why:?}");
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

pub async fn run_bot() {
    // Attempt to load environment variables
    let vars = match Vars::load() {
        Ok(vars) => vars,
        Err(_) => {
            println!("Failed to load environment variables");
            return;
        }
    };

    // Get the Discord bot token
    let token = &vars.discord_bot_token;

    // Define the intents required for the bot
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Create a new instance of the client
    let mut client = match Client::builder(token, intents).event_handler(Handler).await {
        Ok(client) => client,
        Err(why) => {
            println!("Error creating client: {why:?}");
            return;
        }
    };

    // Start the client and handle any potential errors
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
