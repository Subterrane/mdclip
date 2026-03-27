mod clipboard;
mod markdown;

use std::env;
use std::fs;
use std::io::{self, Read, Write};
use std::process::{Command, ExitCode, Stdio};

fn run() -> Result<(), String> {
    // Read input: file arg or stdin
    let input = match env::args().nth(1) {
        Some(path) => fs::read_to_string(&path)
            .map_err(|e| format!("{path}: {e}"))?,
        None => {
            let mut buf = String::new();
            io::stdin().read_to_string(&mut buf)
                .map_err(|e| format!("stdin: {e}"))?;
            buf
        }
    };

    if input.trim().is_empty() {
        return Err("no input".into());
    }

    // Markdown -> styled HTML
    let html = markdown::to_styled_html(&input);

    // HTML -> RTF via textutil
    let rtf = html_to_rtf(html.as_bytes())?;

    // RTF -> clipboard
    clipboard::set_rtf(&rtf)?;

    eprintln!("Copied to clipboard.");
    Ok(())
}

fn html_to_rtf(html: &[u8]) -> Result<Vec<u8>, String> {
    let mut child = Command::new("textutil")
        .args(["-stdin", "-inputencoding", "UTF-8", "-format", "html", "-convert", "rtf", "-stdout"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("textutil: {e}"))?;

    child.stdin.take().unwrap()
        .write_all(html)
        .map_err(|e| format!("textutil stdin: {e}"))?;

    let output = child.wait_with_output()
        .map_err(|e| format!("textutil: {e}"))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("textutil failed: {stderr}"));
    }

    Ok(output.stdout)
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("mdclip: {e}");
            ExitCode::FAILURE
        }
    }
}
