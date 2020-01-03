pub fn create_katex_block(src: &str) -> Result<String, Box<dyn std::error::Error>> {
    use std::io::Write;
    use std::process::{Command, Stdio};

    let mut child = match Command::new("katex")
        .arg("-d")
        .arg("-t")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
    {
        Ok(c) => c,
        Err(e) => {
            eprintln!("failed to launch katex, not rendering math block: {:?}", e);
            return Err(Box::from(e));
        }
    };

    let stdin = child.stdin.as_mut().expect("valid katex stdin");
    stdin.write_all(src.as_ref())?;

    let output = child.wait_with_output()?;
    if !output.status.success() {
        eprintln!(
            "failed to generate katex, exit code: {:?}",
            output.status.code()
        );
        eprintln!("katex STDOUT:");
        eprintln!("{}", String::from_utf8_lossy(output.stdout.as_ref()));
        eprintln!("katex STDERR:");
        eprintln!("{}", String::from_utf8_lossy(output.stdout.as_ref()));
        eprintln!("/katex output");
        return Err(Box::from("katex failed"));
    }
    let rendered: String = String::from_utf8(output.stdout)?;

    Ok(format!(r#"<figure class="math">{}</figure>"#, rendered))
}

pub fn create_katex_inline(src: &str) -> Result<String, Box<dyn std::error::Error>> {
    use std::io::Write;
    use std::process::{Command, Stdio};

    let mut child = match Command::new("katex")
        .arg("-t")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
    {
        Ok(c) => c,
        Err(e) => {
            eprintln!("failed to launch katex, not rendering math block: {:?}", e);
            return Err(Box::from(e));
        }
    };

    let stdin = child.stdin.as_mut().expect("valid katex stdin");
    stdin.write_all(src.as_ref())?;

    let output = child.wait_with_output()?;
    if !output.status.success() {
        eprintln!(
            "failed to generate katex, exit code: {:?}",
            output.status.code()
        );
        eprintln!("katex STDOUT:");
        eprintln!("{}", String::from_utf8_lossy(output.stdout.as_ref()));
        eprintln!("katex STDERR:");
        eprintln!("{}", String::from_utf8_lossy(output.stdout.as_ref()));
        eprintln!("/katex output");
        return Err(Box::from("katex failed"));
    }
    let rendered: String = String::from_utf8(output.stdout)?;

    Ok(rendered)
}
