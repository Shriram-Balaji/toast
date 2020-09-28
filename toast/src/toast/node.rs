use color_eyre::eyre::Result;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use tracing::instrument;

#[instrument]
pub fn render_to_html(
    dir_of_input_files: String,
    output_dir: String,
    filepaths: Vec<String>,
    npm_bin_dir: PathBuf,
) -> Result<()> {
    let bin = npm_bin_dir.join("toast-render");
    let mut cmd = Command::new(bin);
    cmd.arg(dir_of_input_files).arg(output_dir);
    for arg in filepaths {
        cmd.arg(arg);
    }
    let output = cmd.output()?;
    std::io::stdout().write_all(&output.stdout);
    std::io::stderr().write_all(&output.stderr);
    Ok(())
}

#[instrument]
pub async fn source_data(toast_js_file: &PathBuf, npm_bin_dir: PathBuf) -> Result<()> {
    // not a guarantee that toast.js will exist when node
    // goes to look for it: just a sanity check to not
    // execute Command if we don't need to
    if toast_js_file.exists() {
        let mut cmd = Command::new(npm_bin_dir.join("toast-source-data"));
        cmd.arg("/var/tmp/toaster.sock").arg(toast_js_file);
        let output = cmd.output()?;
        // TODO: move stdout/stderr around so it's not just dumping to console
        std::io::stdout().write_all(&output.stdout);
        std::io::stderr().write_all(&output.stderr);
        Ok(())
    } else {
        Ok(())
    }
}