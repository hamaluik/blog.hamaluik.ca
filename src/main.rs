mod frontmatter;
mod post;
use post::Post;

use std::path::{Path, PathBuf};

fn load_posts<P: AsRef<Path>>(src: P) -> Result<Vec<Post>, Box<dyn std::error::Error>> {
    let mut posts: Vec<Post> = Vec::default();

    for entry in src.as_ref().read_dir()? {
        let entry = entry?;
        let path = entry.path();
        if let Some("md") = path.extension().map(std::ffi::OsStr::to_str).flatten() {
            let name = path.file_stem().map(std::ffi::OsStr::to_str).flatten();
            if name.is_none() { continue; }
            match Post::load(&path) {
                Ok(Some(p)) => posts.push(p),
                Ok(None) => (),
                Err(e) => eprintln!("skipping `{}` as it failed to parse: {:?}", path.display(), e),
            };
        }
    }
    
    posts.sort_by(|a, b| b.front.date.cmp(&a.front.date));
    Ok(posts)
}

fn main() {
    let outdir: PathBuf = PathBuf::from("docs").join("posts");
    std::fs::create_dir_all(&outdir).expect("can create docs/posts/ folder");

    let posts = load_posts("posts").expect("can load posts from posts/ folder");
    println!("Found {} posts, rendering them...", posts.len());
    for post in posts.iter() {
        print!("  Rendering `{}`...", post.front.title);
        use std::io::Write;
        std::io::stdout().flush().expect("can flush stdout");

        let html = match post.render() {
            Ok(h) => h,
            Err(e) => {
                eprintln!(" failed: {:?}", e);
                continue;
            }
        };
        let outdir = outdir.join(&post.front.slug);
        std::fs::create_dir_all(&outdir).expect("can create dir for post");
        let outfile = outdir.join("index.html");
        std::fs::write(outfile, html).expect("can write post to index.html file");

        println!(" done!");
    }

    println!("Copying assets...");
    let outdir = PathBuf::from("docs");
    for entry in ignore::Walk::new("assets") {
        let entry = entry.expect("can get path entry");
        if let Some(t) = entry.file_type() {
            if t.is_file() {
                if let Some("md") = entry.path().extension().map(std::ffi::OsStr::to_str).flatten() {
                    // ignore markdown files
                }
                else {
                    // we found an asset to copy!
                    let dest_path: PathBuf = outdir.join(entry.path().iter().skip(1).map(PathBuf::from).collect::<PathBuf>());
                    if let Some(parent) = dest_path.parent() {
                        if !parent.exists() {
                            std::fs::create_dir_all(parent).expect("can create directory");
                        }
                    }
                    std::fs::copy(entry.path(), &dest_path).expect("can copy file");
                    println!("  Copied `{}` to `{}`...", entry.path().display(), dest_path.display());
                }
            }
        }
    }
}
