pub fn create_code_block(src: &str, lang: &str) -> Result<String, Box<dyn std::error::Error>> {
    use std::process::{Command, Stdio};
    use std::io::Write;

    let mut child = match Command::new("pygmentize")
        .arg("-l")
        .arg(lang)
        .arg("-f")
        .arg("html")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn() {
            Ok(c) => c,
            Err(e) => {
                eprintln!("failed to launch pygmentize, not rendering math block: {:?}", e);
                return Err(Box::from(e));
            }
        };

    let stdin = child.stdin.as_mut().expect("valid pygmentize stdin");
    stdin.write_all(src.as_ref())?;

    let output = child.wait_with_output()?;
    if !output.status.success() {
        eprintln!("failed to generate pygmentize, exit code: {:?}", output.status.code());
        eprintln!("pygmentize STDOUT:");
        eprintln!("{}", String::from_utf8_lossy(output.stdout.as_ref()));
        eprintln!("pygmentize STDERR:");
        eprintln!("{}", String::from_utf8_lossy(output.stdout.as_ref()));
        eprintln!("/pygmentize output");
        return Err(Box::from("pygmentize failed"));
    }
    let rendered: String = String::from_utf8(output.stdout)?;
    Ok(rendered)
}