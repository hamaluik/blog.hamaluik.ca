pub fn create_plantuml_svg(src: &str) -> Result<String, Box<dyn std::error::Error>> {
    use std::process::{Command, Stdio};
    use std::io::Write;

    let mut child = match Command::new("plantuml")
        .arg("-tsvg")
        .arg("-nometadata")
        .arg("-pipe")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn() {
            Ok(c) => c,
            Err(e) => {
                eprintln!("failed to launch plantuml, not rendering plantuml block: {:?}", e);
                return Err(Box::from(e))
            }
        };

    let stdin = child.stdin.as_mut().expect("valid plantuml stdin");
    stdin.write_all(src.as_ref())?;

    let output = child.wait_with_output()?;
    if !output.status.success() {
        eprintln!("failed to generate plantuml, exit code: {:?}", output.status.code());
        eprintln!("plantuml STDOUT:");
        eprintln!("{}", String::from_utf8_lossy(output.stdout.as_ref()));
        eprintln!("plantuml STDERR:");
        eprintln!("{}", String::from_utf8_lossy(output.stdout.as_ref()));
        eprintln!("/plantuml output");
        return Err(Box::from("plantuml failed"));
    }
    let svg: String = String::from_utf8(output.stdout)?;
    Ok(svg)
}