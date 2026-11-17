use std::env;
use std::process::ExitCode;

use inference_gateway_sdk::{
    CreateMusicRequest, CreateMusicRequestResponseFormat, CreateSFXRequest, CreateSFXRequestOutput,
    CreateSFXRequestResponseFormat, CreateSpeechRequest, GatewayError, InferenceGatewayAPI,
    InferenceGatewayClient, Provider,
};

const DEFAULT_PROVIDER: &str = "elevenlabs";
const DEFAULT_MODEL: &str = "elevenlabs/eleven_multilingual_v2";
const VOICE: &str = "Sarah";
const SFX_MODEL: &str = "elevenlabs/eleven_text_to_sound_v2";
const MUSIC_MODEL: &str = "elevenlabs/music_v2_5";

fn save(name: &str, bytes: &[u8]) -> Result<(), GatewayError> {
    std::fs::write(name, bytes).map_err(|e| {
        GatewayError::Other(Box::new(std::io::Error::other(format!(
            "failed to write '{name}': {e}"
        ))))
    })?;
    println!("Saved {name} ({} bytes)", bytes.len());
    Ok(())
}

#[tokio::main]
async fn main() -> ExitCode {
    match run().await {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("Error in audio examples: {e}");
            ExitCode::FAILURE
        }
    }
}

async fn run() -> Result<(), GatewayError> {
    let base_url = env::var("INFERENCE_GATEWAY_URL")
        .unwrap_or_else(|_| "http://localhost:8080/v1".to_string());

    let provider_str = env::var("PROVIDER").unwrap_or_else(|_| DEFAULT_PROVIDER.to_string());
    let model = env::var("LLM").unwrap_or_else(|_| DEFAULT_MODEL.to_string());

    let provider: Provider = provider_str.parse().map_err(|e| {
        GatewayError::Other(Box::new(std::io::Error::other(format!(
            "invalid PROVIDER '{provider_str}': {e}"
        ))))
    })?;

    println!("Using provider: {provider_str}");
    println!("Using model: {model}");
    println!("---");

    let client = InferenceGatewayClient::new(&base_url);

    // Example 1: Text to speech
    println!("🔊 Example 1: Text to Speech");
    let audio = client
        .create_speech(
            Some(provider),
            CreateSpeechRequest {
                model,
                input: "Hello from the Inference Gateway"
                    .parse()
                    .expect("the demo text is far below the 4096-character limit"),
                voice: VOICE.to_string(),
                ..Default::default()
            },
        )
        .await?;
    save("speech.mp3", &audio)?;
    println!("---\n");

    // Example 2: Sound effect from a text prompt
    println!("💥 Example 2: Sound Effect");
    let audio = client
        .create_sfx(
            Some(provider),
            CreateSFXRequest {
                duration_seconds: None,
                loop_: None,
                model: SFX_MODEL.to_string(),
                output: CreateSFXRequestOutput::Mix,
                preserve_speech: None,
                prompt: Some("distant thunder rolling over a valley".to_string()),
                prompt_influence: None,
                response_format: CreateSFXRequestResponseFormat::Mp3,
                start_offset_seconds: 0.0,
                video: None,
            },
        )
        .await?;
    save("sfx.mp3", &audio)?;
    println!("---\n");

    // Example 3 needs a video URL; set VIDEO_URL to run it.
    if let Ok(video_url) = env::var("VIDEO_URL") {
        println!("🎬 Example 3: Score a Video");
        let audio = client
            .create_sfx(
                Some(provider),
                CreateSFXRequest {
                    duration_seconds: None,
                    loop_: None,
                    model: MUSIC_MODEL.to_string(),
                    output: CreateSFXRequestOutput::Mix,
                    preserve_speech: Some(true),
                    prompt: None,
                    prompt_influence: None,
                    response_format: CreateSFXRequestResponseFormat::Mp3,
                    start_offset_seconds: 2.5,
                    video: Some(video_url),
                },
            )
            .await?;
        save("video-sfx.mp3", &audio)?;
        println!("---\n");
    } else {
        println!("VIDEO_URL not set - skipping the video-scoring example.");
        println!("Set VIDEO_URL to an https video URL to run it.");
        println!("---\n");
    }

    // Example 4: Music
    println!("🎵 Example 4: Music");
    let audio = client
        .create_music(
            Some(provider),
            CreateMusicRequest {
                duration_seconds: Some(10.0),
                instrumental: true,
                model: MUSIC_MODEL.to_string(),
                prompt: "lo-fi acoustic loop for a rainy afternoon".to_string(),
                response_format: CreateMusicRequestResponseFormat::Mp3,
            },
        )
        .await?;
    save("music.mp3", &audio)?;

    Ok(())
}
