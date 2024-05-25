use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};
use std::io::{self, Write};
// use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use inquire::{InquireError, Select};
use sysinfo::System;

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
    KeyNvidiaSuperSamplingSharpness: f32,
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
        eprintln!(
            "An error occured. If this is unexpected, please open a GitHub issue: {}",
            e
        );
        // attempt to write a readonly database => WW db lock
        if e.to_string()
            .contains("attempt to write a readonly database")
        {
            println!("\nYour error is likely due to Wuthering Waves running and locking the database. Please close the game before running this program.");
        }
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
    }
}

fn check_game_is_running() -> bool {
    let sys = System::new_all();
    sys.processes().values().any(|process| {
        process.name().contains("Wuthering Waves") || process.name().contains("WutheringWavesGame")
    })
}

fn run() -> Result<()> {
    println!("\nWuthering Waves FPS Unlocker");
    println!("NOTE: Modifying the game settings in any way after running this program WILL reset the frame rate to 60 FPS.");

    if check_game_is_running() {
        println!("\nWuthering Waves has been detected as running. Please close the game before running this program.");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        return Ok(());
    }

    print!(
        "\nEnter Wuthering Waves Location (e.g. C:/Games/Wuthering Waves/Wuthering Waves Game): "
    );
    io::stdout().flush().unwrap();

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

        let fps_options: Vec<&str> =
            vec!["30", "60", "72", "90", "120", "144", "165", "240", "Custom"];
        let fps_selection: Result<&str, InquireError> =
            Select::new("What FPS to set?", fps_options).prompt();

        let new_frame_rate = match fps_selection {
            Ok("30") => 30,
            Ok("60") => 60,
            Ok("72") => 72,
            Ok("90") => 90,
            Ok("120") => 120,
            Ok("144") => 144,
            Ok("165") => 165,
            Ok("240") => 240,
            Ok("Custom") => {
                print!("Enter custom frame rate: ");
                io::stdout().flush().unwrap();

                let mut custom_frame_rate = String::new();
                io::stdin()
                    .read_line(&mut custom_frame_rate)
                    .expect("Failed to read line");

                custom_frame_rate.trim().parse::<u32>().unwrap()
            }
            _ => 60,
        };

        game_quality.KeyCustomFrameRate = new_frame_rate;
        game_quality.KeyPcVsync = 0;

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
