use colored::{ColoredString, Colorize};
use pdf_extract::extract_text_from_mem;

pub fn split_text(pdf: &str, chunk_size: usize, show_on_console: bool) -> Vec<String> {
    let res_read = std::fs::read(pdf);
    let bytes = match res_read {
        Ok(res_read) => res_read,
        Err(err) => panic!("\nUnable to read file: {:?}", err),
    };
    let res_extract = extract_text_from_mem(&bytes);
    if let Ok(text) = res_extract {
        return process_paragraph(
            &text.split("\n\n").map(str::to_string).collect(),
            chunk_size,
            show_on_console,
        );
    } else {
        panic!("\nUnable to parse text\n");
    }
}

fn process_paragraph(
    document: &Vec<String>,
    chunk_size: usize,
    show_on_console: bool,
) -> Vec<String> {
    let mut chunks_result: Vec<String> = Vec::new();
    for text in document {
        if text.trim().len() > 0 {
            let t = text.trim().to_string().replace("\n", "");
            if t.chars().count() <= chunk_size {
                chunks_result.push(t.clone());
                if show_on_console {
                    color_console_output(&vec![t.clone()]);
                }
            } else {
                let chunks = process_words(t, chunk_size);
                for chunk in chunks.clone() {
                    chunks_result.push(chunk);
                }
                if show_on_console {
                    color_console_output(&chunks);
                }
            }
        }
    }
    chunks_result.clone()
}

fn color_console_output(paragraph: &Vec<String>) {
    let mut i = 0;
    let mut current = "".to_string();
    for v in paragraph {
        match i {
            0 => current.push_str(v.blue().to_string().as_str()),
            1 => current.push_str(v.red().to_string().as_str()),
            2 => current.push_str(v.green().to_string().as_str()),
            3 => current.push_str(v.yellow().to_string().as_str()),
            _ => current.push_str(v.purple().to_string().as_str()),
        }
        current.push_str(" ");
        i += 1;
        if i == 4 {
            i = 0;
        }
    }
    println!(" {}\n", current);
}

fn process_words(text: String, chunk_size: usize) -> Vec<String> {
    let mut chunks_intermediate: Vec<(usize, String)> = Vec::new();
    let words: Vec<String> = text.split(" ").map(str::to_string).collect();
    let mut n = 0;
    let mut chunk = 0;
    for word in words {
        if word.len() > 0 {
            n = n + word.chars().count();
            if n >= chunk_size {
                chunk += 1;
                n = 0;
            }
            chunks_intermediate.push((chunk, word));
        }
    }
    let mut chunks: Vec<String> = Vec::new();
    let mut i = 0;
    while i <= chunk {
        let grouped = chunks_intermediate
            .iter()
            .filter(|(group, word)| *group == i as usize)
            .map(|(_, word)| word.to_owned())
            .collect::<Vec<String>>();
        let chunk_text: String = grouped.join(" ");
        chunks.push(chunk_text);
        i += 1;
    }
    chunks
}
