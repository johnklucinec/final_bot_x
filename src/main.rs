use std::{env, error::Error, sync::Arc};
use twilight_cache_inmemory::{InMemoryCache, ResourceType};
use twilight_gateway::{Event, Intents, Shard, ShardId};
use twilight_http::Client as HttpClient;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    dotenv().ok();
    let token = env::var("DISCORD_TOKEN")?;

    // Specify intents requesting events about things like new and updated
    // messages in a guild and direct messages.
    let intents = Intents::GUILD_MESSAGES | Intents::DIRECT_MESSAGES | Intents::MESSAGE_CONTENT;

    // Create a single shard.
    let mut shard = Shard::new(ShardId::ONE, token.clone(), intents);

    // The http client is separate from the gateway, so startup a new
    // one, also use Arc such that it can be cloned to other threads.
    let http = Arc::new(HttpClient::new(token));

    // Since we only care about messages, make the cache only process messages.
    let cache = InMemoryCache::builder()
        .resource_types(ResourceType::MESSAGE)
        .build();

    // Startup the event loop to process each event in the event stream as they
    // come in.
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
        // Update the cache.
        cache.update(&event);

        // Spawn a new task to handle the event
        tokio::spawn(handle_event(event, Arc::clone(&http)));
    }

    Ok(())
}

async fn handle_event(
    event: Event,
    http: Arc<HttpClient>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    match event {
        Event::MessageCreate(msg) if msg.content == "!shige" => {
            http.create_message(msg.channel_id).content("chocomint's Made of Fire HDDT 98.54 full combo. Without a doubt, one of the most impressive plays ever set in osu! history, but one that takes some experience to appreciate fully. In the 12 years that this map has been ranked, chocomint's score remains the ONLY DT FC, and there's much more to unpack about this score. While some maps easily convey how difficult they are through the raw aim, or speed requirements, Made of Fire is much more nuanced than it may seem at first glance. To help illustrate just how difficult this play is, I would like to break down and analyze aspects of the map and the play itself. Right off the bat, we get a sense of the density of the notes, and the reading difficulty associated. With DT applied, the map becomes around 243 bpm, and approximately AR 9.67, a higher note density than most players are used to. Still, this should be manageable, but adding to the difficulty is the constant rhythm of, and spacing of the patterns which continues for the entire map. These would already pose a significant challenge to players' rhythm sense, reading, and finger control, but with hidden, the reading becomes MUCH harder. .")?.await?;
            http.create_message(msg.channel_id).content("This is all bread and butter to someone like chocomint, who plays maps with similar patterns all the time, but everything we've gone over is just the beginning. Ask any top player what the hardest part of Made of Fire is, and they will all say the aim control. Many of the patterns in this map are continuous with strange velocity and angle changes, which need very fine adjustments in a player's aim to hit. None are more apparent than the various zig-zag patterns, which appear in the highest spacing sections. These require a player to aim to each note in a 1/4th beat time window, while potentially changing to an almost opposite direction. This is where chocomint shines, as almost no players have the level of aim control which he does. Bringing it all together is where the magic of this play really lies. DT alone makes the aim control barely in reach for any other player, but adding hidden makes these highly control-intensive patterns nearly impossible. Following the rhythm on such a map can be hard already as well, and trying to keep high accuracy, given the layout of the patterns, becomes ridiculous too. ")?.await?;
            http.create_message(msg.channel_id).content("The real challenge is diverting focus between the aim aspect and the tapping aspects of the map, while keeping your reading in check as well. Keeping up with the map the entire way through to such a degree is exactly why chocomint's play is so astonishing. It's hard to put into words how much skill goes into a play like this, but to get an idea, I recommend trying the map for yourself. chocomint's HDDT score truly is in a league of its own when it comes to the aspects described earlier. Just over 2 years after it was set, some players are now approaching DT FCs, but not with nearly as high accuracy, and the majority lacking hidden. For now, it will remain a dream play for every top play, and a reality for chocomint. Hopefully this video has given you some more insight into, and appreciation of, one of osu!'s best plays of all time")?.await?;
        }

        Event::MessageCreate(msg) if msg.author.name == "bigsimpconnor"=> {
            http.create_message(msg.channel_id).content("Shut up @bigsimpconnor")?.await?;
        }

        Event::Ready(_) => {
            println!("Shard is ready");
        }
        _ => {}
    }

    Ok(())
}
 