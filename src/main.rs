// use glfw::{Action, Context, Key};
use inquire::{InquireError, Select};
use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};
use std::io::{self, stdin, stdout, Read, Write};
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

fn get_primary_monitor_refresh_rate() -> u32 {
    let mut glfw = glfw::init(|err, desc| {
        panic!("GLFW error {:?}: {}", err, desc);
    })
    .unwrap();

    let primary_monitor = glfw.with_primary_monitor(|_, monitor| {
        monitor.map(|monitor| {
            let video_mode = monitor.get_video_mode().unwrap();
            let width = video_mode.width;
            let height = video_mode.height;
            let binding = monitor.get_video_modes();
            let refresh_rate = binding
                .iter()
                .filter(|mode| mode.width == width && mode.height == height)
                .max_by_key(|mode| mode.refresh_rate);

            refresh_rate.map(|mode| mode.refresh_rate).unwrap_or(60)
        })
    });

    primary_monitor.unwrap_or(60)
}

fn pause() {
    let mut stdin = stdin();
    let mut stdout = stdout();

    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();

    let mut buffer = [0; 1];
    match stdin.read_exact(&mut buffer) {
        Ok(_) => {}
        Err(e) => eprintln!("Failed to read from stdin: {}", e),
    };
}

fn get_custom_frame_rate() -> u32 {
    print!("Enter custom frame rate: ");
    io::stdout().flush().unwrap();

    let mut custom_frame_rate = String::new();
    io::stdin()
        .read_line(&mut custom_frame_rate)
        .expect("Failed to read line");

    custom_frame_rate.trim().parse::<u32>().unwrap_or(60)
}

fn main() {
    if let Err(e) = run() {
        eprintln!(
            "An error occured. If this is unexpected, please open a GitHub issue: {}",
            e
        );

        // attempt to write a readonly database => WW db lock
        let error_message = e.to_string();

        if error_message.contains("attempt to write a readonly database") {
            println!("\nYour error is likely due to Wuthering Waves running and locking the database. Please close the game before running this program.");
        }

        // unable to open database file => incorrect game location
        if error_message.contains("unable to open database file") {
            println!("\nYour error is likely due to an incorrect game location. Please ensure you have entered the correct path.");
        }

        pause();
    }
    pause();
}

fn check_game_is_running() -> bool {
    let sys = System::new_all();
    sys.processes().values().any(|process| {
        process.name().contains("Wuthering Waves")
            || process.name().contains("Wuthering Waves Game")
    })
}

fn run() -> Result<()> {
    println!("\nWuthering Waves FPS Unlocker - https://github.com/dromzeh/wuwa-fps-unlocker");
    println!("NOTE: Modifying the game settings in any way after running this program WILL reset the frame rate to 60 FPS.");

    if check_game_is_running() {
        println!("\nWuthering Waves has been detected as running. Please close the game before running this program.");
        pause();
    }

    println!("\nRequesting game location...");

    let game_location = tinyfiledialogs::open_file_dialog(
        "Select /Wuthering Waves/Wuthering Waves Game/Wuthering Waves.exe",
        "",
        Some((&["Wuthering Waves.exe"], "Wuthering Waves.exe")),
    );

    let game_location = match game_location {
        Some(file_path) => file_path,
        None => {
            println!("User did not select a file.");
            tinyfiledialogs::message_box_ok(
                "Error",
                "No file selected",
                tinyfiledialogs::MessageBoxIcon::Error,
            );
            return Ok(());
        }
    };

    let game_location = game_location
        .split("Wuthering Waves.exe")
        .collect::<Vec<&str>>()
        .join("");

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

        let primary_monitor_refresh_rate = get_primary_monitor_refresh_rate();

        let prompt = format!(
            "Your primary monitor's refresh rate is {}. Would you like to set the game frame rate to this value?",
            primary_monitor_refresh_rate
        );

        let select_options = vec!["Yes", "No"];
        let select_refresh_rate = Select::new(&prompt, select_options).prompt();

        let new_frame_rate = match select_refresh_rate {
            Ok("Yes") => primary_monitor_refresh_rate,
            _ => {
                let fps_options: Vec<&str> =
                    vec!["30", "60", "72", "90", "120", "144", "165", "240", "Custom"];
                let fps_selection: Result<&str, InquireError> =
                    Select::new("What FPS to set?", fps_options).prompt();

                match fps_selection {
                    Ok("Custom") => get_custom_frame_rate(),
                    Ok(selection) => selection.parse::<u32>().unwrap_or(60),
                    _ => 60,
                }
            }
        };

        game_quality.KeyCustomFrameRate = new_frame_rate;
        game_quality.KeyPcVsync = 0;

        let updated_value =
            serde_json::to_string(&game_quality).expect("Failed to convert struct to JSON");

        conn.execute(
            "UPDATE LocalStorage SET value = ?1 WHERE key = 'GameQualitySetting'",
            params![updated_value],
        )?;

        println!("\nUpdated framerate to {}", new_frame_rate);
    } else {
        println!("\nGameQualitySetting not found in LocalStorage");
    }

    Ok(())
}
