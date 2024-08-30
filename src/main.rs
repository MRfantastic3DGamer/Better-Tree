mod style;
use std::cmp;
use std::fs;
use std::path::Path;
use std::io;
use std::env;
use std::process::exit;
use std::io::Write;
use style::{BASIC_STYLE, HEAVY_STYLE};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <directory path> <documentation path>", args[0]);
        exit(1);
    }

    let root_input = &args[1];
    let doc_input = &args[2];
    let root_path = Path::new(root_input);
    let doc_path = Path::new(doc_input);

    if root_path.is_dir() {
        let folder_view = build_view(root_path);
        if let Err(e) = update_markdown_file(doc_path, &format!("\n{}\n", folder_view.value)) {
            eprintln!("Error updating markdown file: {}", e);
            exit(1);
        } else {
            println!("File updated successfully!");
        }
    } else {
        eprintln!("The provided path -{}- is not a directory.", root_path);
        exit(1);
    }

    Ok(())
}

fn update_markdown_file<P: AsRef<Path>>(file_path: P, diagram: &String) -> io::Result<()> {
    // Read the contents of the file
    let mut content = fs::read_to_string(&file_path)?;

    // Find the position of the comment
    if let Some(comment_start) = content.find("<!---BETTER_FILES_TREE-->") {
        // Find the start of the code block after the comment
        if let Some(code_block_start) = content[comment_start..].find("```") {
            let start_idx = comment_start + code_block_start + 3; // skip over ```
            // Find the end of the code block
            if let Some(code_block_end) = content[start_idx..].find("```") {
                let end_idx = start_idx + code_block_end;

                // Replace the content of the code block with "Hello world"
                content.replace_range(start_idx..end_idx, diagram);

                // Write the updated content back to the file
                let mut file = fs::File::create(file_path)?;
                file.write_all(content.as_bytes())?;
            }
        }
    }

    Ok(())
}

#[derive(Debug)]
struct View {
    width: u128,
    value: String,
}

fn put_view_in(view: &View, width: u128, fill: String, h: &str, p: &str) -> String {
    view.value
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let padding_width = if width > view.width { width - view.width + 1 } else { 1 } as usize;
            let l = line.trim();
            format!(
                "{}{}{}{}",
                if i == 0 { p } else { h },
                l,
                fill.repeat(padding_width),
                h
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn build_view(path: &Path) -> View {
    if path.is_file() {
        // Get the file name and calculate its length
        let file_name = path.file_name().unwrap().to_string_lossy().to_string().trim().to_string();
        let width = file_name.len() as u128;
        return View {
            width,
            value: file_name,
        };
    } else if path.is_dir() {
        // Directory case
        let mut max_width = 0;
        let mut children_views = Vec::new();

        match fs::read_dir(path) {
            Ok(entries) => {
                for entry in entries {
                    match entry {
                        Ok(entry) => {
                            let child_view = build_view(&entry.path());
                            max_width = cmp::max(max_width, child_view.width);
                            children_views.push(child_view);
                        }
                        Err(e) => eprintln!("Error reading entry: {}", e),
                    }
                }
            }
            Err(e) => eprintln!("Error reading directory: {}", e),
        }

        let dir_name = path.file_name().unwrap().to_string_lossy().to_string();
        // Adjust max_width calculation to avoid overflow
        let dir_name_len = dir_name.len() as u128;
        max_width = cmp::max(max_width, dir_name_len + 1);
        let style = if dir_name.starts_with('.') {&BASIC_STYLE} else {&HEAVY_STYLE};

        let children_view_value = children_views
            .iter()
            .map(|child_view| put_view_in(
                    child_view,
                    max_width,
                    format!("{}"," ").to_string(),
                    style.v,
                    style.is
                ))
            .collect::<Vec<_>>()
            .join("\n");

        let val = format!(
            "{} {}{}{}{}\n{}{}{}", 
            style.tl,
            dir_name,
            style.h.to_string().repeat((max_width - dir_name_len) as usize),
            style.tr,
            if children_views.len()>0 {"\n".to_string() + &children_view_value} else {"".to_string()},
            style.bl,
            style.h.to_string().repeat((max_width + 1) as usize),
            style.br
        );

        return View {
            width: max_width + 3,
            value: val,
        };
    }

    // In case the path is neither a file nor a directory, return an empty View
    View {
        width: 0,
        value: String::new(),
    }
}
