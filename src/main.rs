use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};
use std::io;
// use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use inquire::{InquireError, Select};

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct GameQualitySetting {
    KeyQualityLevel: u8,
    KeyCustomFrameRate: u32,
    KeyNewShadowQuality: u8,
    KeyNiagaraQuality: u8,
    KeyImageDetail: u8,
    KeyAntiAliasing: u8,
    KeySceneAo: u8,
    KeyVolumeFog: u8,
    KeyVolumeLight: u8,
    KeyMotionBlur: u8,
    KeyStreamLevel: u8,
    KeyPcVsync: u8,
    KeyMobileResolution: u8,
    KeySuperResolution: u8,
    KeyPcResolutionWidth: u32,
    KeyPcResolutionHeight: u32,
    KeyBrightness: u8,
    KeyPcWindowMode: u8,
    KeyNvidiaSuperSamplingEnable: u8,
    KeyNvidiaSuperSamplingFrameGenerate: u8,
    KeyNvidiaSuperSamplingMode: u8,
    KeyNvidiaSuperSamplingSharpness: u8,
    KeyNvidiaReflex: u8,
    KeyFsrEnable: u8,
    HorizontalViewSensitivity: u8,
    VerticalViewSensitivity: u8,
    AimHorizontalViewSensitivity: u8,
    AimVerticalViewSensitivity: u8,
    CameraShakeStrength: u8,
    MobileHorizontalViewSensitivity: u8,
    MobileVerticalViewSensitivity: u8,
    MobileAimHorizontalViewSensitivity: u8,
    MobileAimVerticalViewSensitivity: u8,
    MobileCameraShakeStrength: u8,
    CommonSpringArmLength: u8,
    FightSpringArmLength: u8,
    IsResetFocusEnable: u8,
    IsSidestepCameraEnable: u8,
    IsSoftLockCameraEnable: u8,
    JoystickShakeStrength: u8,
    JoystickShakeType: u8,
    WalkOrRunRate: f32,
    JoystickMode: u8,
    IsAutoSwitchSkillButtonMode: u8,
    AimAssistEnable: u8,
}

fn main() {
    if let Err(e) = run() {
        eprintln!("error: {}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    println!(
        "\nEnter Wuthering Waves Location (e.g. C:/Games/Wuthering Waves/Wuthering Waves Game):"
    );

    let mut game_location = String::new();
    io::stdin()
        .read_line(&mut game_location)
        .expect("Failed to read line");

    game_location = game_location.trim().to_string();

    let db_path = format!(
        "{}/Client/Saved/LocalStorage/LocalStorage.db",
        game_location
    );

    let conn = Connection::open(db_path)?;

    let mut stmt =
        conn.prepare("SELECT value FROM LocalStorage WHERE key = 'GameQualitySetting'")?;
    let mut rows = stmt.query([])?;

    if let Some(row) = rows.next()? {
        let value: String = row.get(0)?;

        let mut game_quality: GameQualitySetting =
            serde_json::from_str(&value).expect("JSON was not well-formatted");

        println!("\nCurrent Frame Rate: {}", game_quality.KeyCustomFrameRate);

        let fps_options: Vec<&str> = vec!["30", "60", "120", "240", "Custom"];
        let fps_selection: Result<&str, InquireError> =
            Select::new("What FPS to set?", fps_options).prompt();

        let new_frame_rate = match fps_selection {
            Ok("30") => 30,
            Ok("60") => 60,
            Ok("120") => 120,
            Ok("240") => 240,
            Ok("Custom") => {
                let mut custom_frame_rate = String::new();
                println!("Enter custom frame rate:");
                io::stdin()
                    .read_line(&mut custom_frame_rate)
                    .expect("Failed to read line");
                custom_frame_rate
                    .trim()
                    .parse::<u32>()
                    .expect("Failed to parse custom frame rate")
            }
            _ => 60,
        };

        game_quality.KeyCustomFrameRate = new_frame_rate;

        let updated_value =
            serde_json::to_string(&game_quality).expect("Failed to convert struct to JSON");

        conn.execute(
            "UPDATE LocalStorage SET value = ?1 WHERE key = 'GameQualitySetting'",
            params![updated_value],
        )?;

        println!("\nKeyCustomFrameRate updated to {}", new_frame_rate);

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
    } else {
        println!("\nGameQualitySetting not found in LocalStorage");
    }

    Ok(())
}
