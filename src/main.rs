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
    use rayon::prelude::*;

    let outdir: PathBuf = PathBuf::from("docs").join("posts");
    std::fs::create_dir_all(&outdir).expect("can create docs/posts/ folder");

    let posts = load_posts("posts").expect("can load posts from posts/ folder");
    println!("Found {} posts, rendering them...", posts.len());
    let errors: Vec<String> = posts
        .par_iter()
        .filter_map(|post| {
            let html = match post.render() {
                Ok(h) => h,
                Err(e) => {
                    return Some(format!("failed to render `{}`: {:?}", post.source.display(), e));
                }
            };
            let outdir = outdir.join(&post.front.slug);
            std::fs::create_dir_all(&outdir).expect("can create dir for post");
            let outfile = outdir.join("index.html");
            std::fs::write(outfile, html).expect("can write post to index.html file");
            return None;
        })
        .collect();
    if errors.len() > 0 {
        eprintln!("Failed to render some posts:");
        for error in errors.iter() {
            eprintln!("  {}", error);
        }
    }
    else {
        println!("Posts rendered!");
    }

    println!("Copying assets...");
    let outdir = PathBuf::from("docs");
    let mut paths: Vec<PathBuf> = Vec::default();
    for entry in ignore::Walk::new("assets") {
        let entry = entry.expect("can get path entry");
        if let Some(t) = entry.file_type() {
            if t.is_file() {
                if let Some("md") = entry.path().extension().map(std::ffi::OsStr::to_str).flatten() {
                    // ignore markdown files
                }
                else {
                    // we found an asset to copy!
                    paths.push(entry.path().to_owned());
                }
            }
        }
    }
    paths
        .par_iter()
        .for_each(|path| {
            let dest_path: PathBuf = outdir.join(path.iter().skip(1).map(PathBuf::from).collect::<PathBuf>());
            if let Some(parent) = dest_path.parent() {
                if !parent.exists() {
                    std::fs::create_dir_all(parent).expect("can create directory");
                }
            }
            std::fs::copy(path, &dest_path).expect("can copy file");
        });
}
