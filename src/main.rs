use std::{env, error::Error, sync::Arc};
use twilight_cache_inmemory::{InMemoryCache, ResourceType};
use twilight_gateway::{Event, Intents, Shard, ShardId};
use twilight_http::{Client as HttpClient};
use dotenv::dotenv;
use twilight_gateway::{stream::{self, ShardEventStream}, Config};
use twilight_model::http::interaction::{InteractionResponse, InteractionResponseData, InteractionResponseType};
use twilight_model::id::marker::{ApplicationMarker, GuildMarker};
use vesper::prelude::*;
use vesper::framework::Framework;
use twilight_model::id::Id;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    dotenv().ok();
    let token = env::var("DISCORD_TOKEN")?;

    let intents = Intents::GUILD_MESSAGES | Intents::DIRECT_MESSAGES | Intents::MESSAGE_CONTENT;

    let mut shard = Shard::new(ShardId::ONE, token.clone(), intents);

    let http = Arc::new(HttpClient::new(token));

    let cache = InMemoryCache::builder()
        .resource_types(ResourceType::MESSAGE)
        .build();
    
    let app_id = Id::<ApplicationMarker>::new(env::var("APP_ID")?.parse()?);

    let framework = Arc::new(
        Framework::builder(Arc::clone(&http), app_id, ())
            .command(hello)
            .build()
    );

    framework.register_guild_commands(Id::<GuildMarker>::new(env::var("GUILD_ID")?.parse::<u64>()?)).await.unwrap();


    loop {
        let event = match shard.next_event().await {
            Ok(event) => event,
            Err(source) => {
                tracing::warn!(?source, "error receiving event");
                println!("{:?}", source);

                if source.is_fatal() {
                    println!("It broke bozo");
                    break;
                }

                continue;
            }
        };

        cache.update(&event);

        tokio::spawn(handle_event(event, Arc::clone(&http), Arc::clone(&framework)));
    }

    Ok(())
}

async fn handle_event(
    event: Event,
    http: Arc<HttpClient>,
    framework: Arc<Framework<()>>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    match event {
        Event::MessageCreate(msg) if msg.content == "!ping" => {
            http.create_message(msg.channel_id).content("pong")?.await?;
        }

        Event::MessageCreate(msg) if msg.author.name == "bigsimpconnor" => {
            http.create_message(msg.channel_id).content("Shut up @bigsimpconnor")?.await?;
        }

        Event::InteractionCreate(i) => {
            let clone = Arc::clone(&framework);
            tokio::spawn(async move {
                let inner = i.0;
                clone.process(inner).await;
            });
        }

        Event::Ready(_) => {
            println!("Shard is ready");
        }
        _ => {}
    }

    Ok(())
}

#[command]
#[description = "Says hello"]
async fn hello(ctx: &SlashContext<()>) -> DefaultCommandResult {
    ctx.interaction_client.create_response(
        ctx.interaction.id,
        &ctx.interaction.token,
        &InteractionResponse {
            kind: InteractionResponseType::ChannelMessageWithSource,
            data: Some(InteractionResponseData {
                content: Some(String::from("Hello world")),
                ..Default::default()
            })
        }
    ).await?;

    Ok(())
}
