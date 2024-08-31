mod style;
use std::env;
use std::cmp;
use std::fs;
use std::path::{Path, PathBuf};
use std::io;
use std::process::exit;
use std::io::Write;
use style::{BASIC_STYLE, HEAVY_STYLE};
use clap::{Arg, Command};

fn main() -> io::Result<()> {
    let current_dir = env::current_dir()?;
    let matches = Command::new("myapp")
        .version("1.0")
        .author("Your Name <your.email@example.com>")
        .about("Generates a directory structure view and updates a markdown file.")
        .arg(
            Arg::new("root")
                .help("Sets the root directory path")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("doc")
                .help("Sets the documentation file path\nit should contain \n <!---BETTER_FILES_TREE-->\n```\n```")
                .required(false)                
                .index(2),
        )
        .arg(
            Arg::new("no-files")
                .help("Exclude files from the output")
                .short('n')
                .long("no-files")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("stack-folders")
                .help("Display folders in a stacked manner")
                .short('s')
                .long("stack-folders")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("show-hidden")
                .help("Include hidden files and folders")
                .short('H')
                .long("show-hidden")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("ignored-locations")
                .help("List of locations to ignore")
                .short('i')
                .long("ignore")
                .num_args(0..),
            )
        .get_matches();

    // Get the required arguments
    let root_input = matches.get_one::<String>("root").expect("Required argument");
    let root_path = if root_input == "./" { current_dir.clone() } else { current_dir.join(root_input) };
    let doc_input = matches.get_one::<String>("doc");
    let ignored_locations: Vec<PathBuf> = matches
        .get_many::<String>("ignored-locations")
        .unwrap_or_default()
        .map(|location| root_path.join(location))
        .collect();

    println!("Root input: {}", root_input);
    if let Some(doc_input) = doc_input {
        println!("Doc input: {}", current_dir.join(doc_input).display());
    }


    println!("Resolved root path: {}", root_path.display());

    // Get the optional flags
    let no_files = matches.get_flag("no-files");
    let stack_folders = matches.get_flag("stack-folders");
    let show_hidden = matches.get_flag("show-hidden");

    println!("No files flag: {}", no_files);
    println!("Stack folders flag: {}", stack_folders);
    println!("Show hidden flag: {}", show_hidden);

    if root_path.is_dir() {
        println!("Root path is a directory");

        // Modify the build_view function to use the flags as needed
        let folder_view = build_view(root_path.as_path(), &no_files, &stack_folders, &show_hidden, &ignored_locations);
        if let Some(doc_input) = doc_input {
            let doc_full_path = current_dir.join(doc_input);
            let doc_path = doc_full_path.as_path();
            
            if let Err(e) = update_markdown_file(doc_path, &format!("\n{}\n", folder_view.value)) {
                eprintln!("Error updating markdown file: {}", e);
                exit(1);
            } else {
                println!("File updated successfully!");
            }
        } else {
            eprintln!("{}", folder_view.value);
        }
    } else {
        eprintln!("The provided path -{}- is not a directory.", root_path.display());
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
    hidden: bool,
    add: bool,
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

fn is_ignored(path: &Path, ignored_locations: &Vec<PathBuf>) -> bool {
    for ignored_path in ignored_locations {
        if path == ignored_path {
            return true;
        }
    }
    false
}

fn build_view(path: &Path, no_files: &bool, stack_folders: &bool, show_hidden: &bool, ignored_locations: &Vec<PathBuf> ) -> View {
    
    let name = path.file_name().unwrap().to_string_lossy().to_string().trim().to_string();
    let name_width = name.len() as u128;
    let is_hidden = name.starts_with('.');

    if is_ignored(path, ignored_locations) || (is_hidden && !*show_hidden) {
        return View {
            width: 0,
            value: String::new(),
            hidden: is_hidden,
            add: false,
        };
    }

    if path.is_file() {
        return View {
            width: name_width,
            value: name,
            hidden: is_hidden,
            add: !no_files,
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
                            let child_view = build_view(&entry.path(), no_files, stack_folders, show_hidden, ignored_locations);
                            if !child_view.add { continue; }
                            if child_view.hidden {
                                if*(show_hidden){
                                    max_width = cmp::max(max_width, child_view.width);
                                    children_views.push(child_view);
                                }
                                continue;
                            }
                            max_width = cmp::max(max_width, child_view.width);
                            children_views.push(child_view);
                        }
                        Err(e) => eprintln!("Error reading entry: {}", e),
                    }
                }
            }
            Err(e) => eprintln!("Error reading directory: {}", e),
        }

        max_width = cmp::max(max_width, name_width + 1);
        let style = if name.starts_with('.') {&BASIC_STYLE} else {&HEAVY_STYLE};

        if children_views.len() == 1 && *stack_folders {
            let stacked_name = format!("{}/{}",name, children_views[0].value);
            return View{
                width: stacked_name.len() as u128,
                value: stacked_name,
                hidden: is_hidden,
                add: true,
            };
        }

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
            name,
            style.h.to_string().repeat((max_width - name_width) as usize),
            style.tr,
            if children_views.len()>0 {"\n".to_string() + &children_view_value} else {"".to_string()},
            style.bl,
            style.h.to_string().repeat((max_width + 1) as usize),
            style.br
        );

        return View {
            width: max_width + 3,
            value: val,
            hidden: is_hidden,
            add: true,
        };
    }

    // In case the path is neither a file nor a directory, return an empty View
    View {
        width: 0,
        value: String::new(),
        hidden: true,
        add: true,
    }
}
