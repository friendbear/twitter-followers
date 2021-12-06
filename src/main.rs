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

    let user_id: UserID = options.username.into();

    let list = egg_mode::user::followers_of(user_id, &token)
        .take(200)
            .map_ok(|r| r.response)
            .try_collect::<Vec<_>>().await?;

    print_followers(list.iter());

    Ok(())
}

fn print_followers(iterator: Iter<TwitterUser>) {
    iterator
//        .filter_map(|status| status.entities.media.as_ref())
//        .flatten()
        // .map(|x| &x.media_url_https)
        // .filter(|x| !x.contains("thumb"))
        .for_each(|x| println!(r#"{{id: {id}, "username": "{username}", "name": "{name}", "created_at": "{created_at}"}}"#,
            id = x.id,
            username = x.screen_name,
            name = x.name,
            created_at = x.created_at)
        );
}
