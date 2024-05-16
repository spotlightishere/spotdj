use std::{borrow::Cow, env};

use librespot::{
    core::{authentication::Credentials, config::SessionConfig, http::Method, session::Session},
    protocol::tts_resolve::{
        resolve_request::{AudioFormat, Prompt, TtsProvider, TtsVoice},
        ResolveRequest,
    },
};
use poise::serenity_prelude as serenity;

/// Pass across our signed in Spotify session.
// TODO(spotlightishere): Is it safe to retain this session like this?
struct Data {
    session: Session,
}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

/// Utilises the Spotify DJ TTS voice to narrate your message.
#[poise::command(slash_command)]
async fn spotifydj(
    ctx: Context<'_>,
    #[description = "Message to read"] message: String,
) -> Result<(), Error> {
    // TODO(spotlightishere): Figure out a way to bridge markdown to proper SSML formatting
    let formatted_prompt = format!("<speak xml:lang=\"en-US\">{message}</speak>");

    // Next, prepare our TTS request.
    // These values were observed while working with the official client.
    let narration_request = ResolveRequest {
        audio_format: AudioFormat::MP3.into(),
        // It's unknown if these other TTS providers function.
        // (As of writing, Sonatic is a company owned by Spotify.)
        tts_provider: TtsProvider::SONANTIC.into(),
        // Similarly - do any of these other voices exist?
        //
        // The official client appears to have "Voice 1" hardcoded, but there
        // are VOICE1 to VOICE40 as of Spotify 1.2.22.982.g794acc0a (macOS).
        tts_voice: TtsVoice::VOICE1.into(),
        sample_rate_hz: 44100,
        prompt: Prompt::Ssml(formatted_prompt.clone()).into(),

        // Language is oddly not specified by the official client.
        ..Default::default()
    };

    // Make our TTS request!
    // Note that this should be "application/x-www-form-urlencoded",
    // but request_with_protobuf overwrites it to "application/x-protobuf".
    //
    // Thankfully, this does not appear to be validated :)
    let raw_response = ctx
        .data()
        .session
        .spclient()
        .request_with_protobuf(
            &Method::POST,
            "/client-tts/v1/fulfill",
            None,
            &narration_request,
        )
        .await
        .expect("failed requesting TTS from Spotify");

    let mp3_bytes = Cow::Owned(raw_response.to_vec());
    let mp3_attachment = serenity::CreateAttachment::bytes(mp3_bytes, "spotify_dj.mp3".to_string());

    let message = poise::CreateReply::default()
        .content(format!(
            "Sending the following SSML: ```xml
{formatted_prompt}
```"
        ))
        .attachment(mp3_attachment)
        .reply(true);

    ctx.send(message).await?;

    Ok(())
}

#[tokio::main]
async fn main() {
    let session_config = SessionConfig::default();

    // TODO(spotlightishere): Make this slightly less hectic
    let spotify_username =
        env::var("SPOTIFY_USERNAME").expect("should have env var SPOTIFY_USERNAME set");
    let spotify_password =
        env::var("SPOTIFY_PASSWORD").expect("should have env var SPOTIFY_PASSWORD set");
    let discord_token = env::var("DISCORD_TOKEN").expect("should have env var DISCORD_TOKEN set");
    let credentials = Credentials::with_password(spotify_username, spotify_password);

    // Although we really only need credentials here, we need to connect to a Spotify session.
    println!("Connecting...");
    let session = Session::new(session_config, None);
    session
        .connect(credentials, true)
        .await
        .expect("Failed to connect to Spotify!");

    // Next, let's connect to Discord.
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![spotifydj()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data { session })
            })
        })
        .build();

    let intents =
        serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;
    let client = serenity::ClientBuilder::new(discord_token, intents)
        .framework(framework)
        .await;

    client.unwrap().start().await.unwrap();

    println!("Done");
}
