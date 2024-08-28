mod style;
use std::cmp;
use std::fs;
use std::path::Path;
use style::{BASIC_STYLE, HEAVY_STYLE};

fn main() {
    let path = std::env::args().nth(1).expect("Please provide a directory path.");
    let root_path = Path::new(&path);

    if root_path.is_dir() {
        let tree_string = build_view(root_path);
        println!("{}", tree_string.value);
    } else {
        eprintln!("The provided path is not a directory.");
    }
}

#[derive(Debug)]
struct View {
    width: u128,
    value: String,
}

fn put_view_in(view: &View, width: u128, fill: String, h: &str, is: &str) -> String {
    view.value
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let padding_width = if width > view.width { width - view.width + 1 } else { 1 } as usize;
            let l = line.trim();
            format!(
                "{}{}{}{}",
                if i == 0 { is } else { h },
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
