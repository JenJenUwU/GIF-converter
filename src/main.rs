use clap::Parser;
use dialoguer::{theme::ColorfulTheme, Select, Input};
use std::process::Command;

/// Command-line arguments
#[derive(Parser)]
struct Cli {
    /// Frames per second
    #[arg(short, long)]
    fps: Option<String>,

    /// Scale
    #[arg(short, long)]
    scale: Option<String>,

    /// Input file
    #[arg(short, long)]
    input: Option<String>,

    /// Output file
    #[arg(short, long)]
    output: Option<String>,
}

fn main() {
    let args = Cli::parse();

    let mut fps = args.fps.unwrap_or_else(|| "10".into());
    let mut scale = args.scale.unwrap_or_else(|| "320:-1".into());
    let mut input = args.input.unwrap_or_else(|| "input.mov".into());
    let mut output = args.output.unwrap_or_else(|| "output.gif".into());

    loop {
        let selections = &[
            format!("FPS: [{}]", fps),
            format!("Scale: [{}]", scale),
            format!("Input: [{}]", input),
            format!("Output: [{}]", output),
            "Save".to_string(),
            "Exit".to_string(),
        ];

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choose an option to modify")
            .items(selections)
            .default(0)
            .interact()
            .unwrap();

        match selection {
            0 => {
                fps = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Frames per second")
                    .default(fps.clone())
                    .interact_text()
                    .unwrap();
            }
            1 => {
                scale = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Scale")
                    .default(scale.clone())
                    .interact_text()
                    .unwrap();
            }
            2 => {
                input = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Input file")
                    .default(input.clone())
                    .interact_text()
                    .unwrap();
            }
            3 => {
                output = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Output file")
                    .default(output.clone())
                    .interact_text()
                    .unwrap();
            }
            4 => {
                let status = Command::new("ffmpeg")
                    .args(&[
                        "-v", "warning",
                        "-i", &input,
                        "-vf", &format!("fps={},scale={}:flags=lanczos,split[s0][s1];[s0]palettegen[p];[s1][p]paletteuse", fps, scale),
                        &output,
                    ])
                    .status()
                    .expect("Failed to execute ffmpeg command");

                if status.success() {
                    println!("GIF created successfully.");
                } else {
                    eprintln!("Failed to create GIF.");
                }
            }
            5 => break,
            _ => unreachable!(),
        }
    }
}