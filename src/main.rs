use std::slice::Iter;

use color_eyre::Result;
use egg_mode::user;
use egg_mode::user::followers_of;
use egg_mode::user::UserID;
use egg_mode::user::TwitterUser;
use egg_mode::KeyPair;
use egg_mode::Token::Access;
use structopt::StructOpt;
use futures::{StreamExt, TryStreamExt};
use serde_json;
extern crate ratelimit;
use std::time::Duration;

#[derive(StructOpt)]
#[structopt(
    rename_all = "kebab-case",
    about = "Fetches the last tweets of a given account, then prints original quality URLs for all image tweets."
)]
struct CliOptions {
    /// The Twitter username of the account to fetch images from.
    #[structopt(env = "TARGET_USERNAME")]
    username: String,

    /// The maximum amount of tweets to check for images.
    #[structopt(long, default_value = "1024")]
    max_amount: i32,

    /// The consumer API key for the project.
    #[structopt(long, env, default_value = std::option_env!("CONSUMER_KEY").unwrap_or(""))]
    consumer_key: String,

    /// The consumer key secret for the project.
    #[structopt(long, env, default_value = std::option_env!("CONSUMER_KEY_SECRET").unwrap_or(""))]
    consumer_key_secret: String,

    /// The access token for your user, for the project.
    #[structopt(long, env, default_value = std::option_env!("ACCESS_TOKEN").unwrap_or(""))]
    access_token: String,

    /// The access token secret for your user.
    #[structopt(long, env, default_value = std::option_env!("ACCESS_TOKEN_SECRET").unwrap_or(""))]
    access_token_secret: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let options: CliOptions = CliOptions::from_args();

    let consumer = KeyPair::new(options.consumer_key, options.consumer_key_secret);
    let access = KeyPair::new(options.access_token, options.access_token_secret);
    let token = Access { consumer, access };

    let mut ratelimit = ratelimit::Builder::new()
        .capacity(10)
        .quantum(1)
        .interval(Duration::from_secs(60))
        .build();


    let user_id: UserID = options.username.into();
    println!("Fetching tweets...");
    for i in -10..options.max_amount {
        let tweets = user::followers_of(user_id.clone(), &token)
            .take(100)
            .map_ok(|r| r.response)
            .try_collect::<Vec<_>>().await?;

        tweets.iter().for_each(|tweet| {
            print_followers(&tweet);
        });
        ratelimit.wait();
    }
   
//    print!("{}", serde_json::to_string_pretty(&users)?);
    //print_followers(list.iter());

    Ok(())
}

fn print_followers(x: &TwitterUser) {
//        .filter_map(|status| status.entities.media.as_ref())
//        .flatten()
        // .map(|x| &x.media_url_https)
        // .filter(|x| !x.contains("thumb"))
        println!(r#"{{"id": "{:?}", "username": "{:?}", "name": "{:?}", "description", "{:?}, "created_at": "{:?}"}}"#,
            id = x.id,
            username = x.screen_name,
            name = x.name,
            description = x.description,
            created_at = x.created_at);
}
