use std::{fs::File, io::Read, path::Path};

fn main() {
    include_readme().unwrap();
}

fn remove_useless_markdown(string: &mut String) {
    // let bytes = string.bytes();
    let headers = ["<h2>", "<h3>"];
    replace_in_string(string, &headers, "");
    // let closing_headers= ["</h2>"];
    let closing_headers = headers
        .iter()
        .map(|header| {
            let mut h = header.to_string();
            h.insert(1, '/');
            h
        })
        .collect::<Vec<String>>();
    let closing_headers = closing_headers
        .iter()
        .map(|s| s.as_str())
        .collect::<Vec<&str>>();
    // println!("cargo::warning={closing_headers:?}");

    replace_in_string(string, &closing_headers, "");

    // the  stuff at top
    replace_in_string(
        string,
        &[
            "# rsdex &emsp; [![Latest Version]][crates.io]  [![Latest Release Banner]][Latest Release]",
            "[Latest Release Banner]:https://img.shields.io/badge/Latest-release-blue",
            "[Latest Version]: https://img.shields.io/crates/v/rsdex.svg",
            "[Latest Release]:https://github.com/PurpleCraft999/rsdex/releases/latest/",
            "[crates.io]: https://crates.io/crates/rsdex",
        ],
        "",
    );
    replace_in_string(string, &["<code>", "</code>"], "`");
}

fn replace_in_string(string: &mut String, to_remove: &[&str], replace_with: &str) {
    for remove in to_remove {
        *string = string.replace(*remove, replace_with);
    }
}

fn include_readme() -> std::io::Result<()> {
    println!("cargo::rerun-if-changed=README.md");
    let mut file = File::open(env!("CARGO_PKG_README"))?;
    let mut read_me = String::new();
    file.read_to_string(&mut read_me)?;
    remove_useless_markdown(&mut read_me);

    let rust_read_me = format!(
        "pub const READ_ME:&str = r##\"{}\"##;",
        read_me.replace('\r', "").trim()
    );
    std::fs::write(
        Path::new(&std::env::var_os("OUT_DIR").unwrap()).join("readme.rs"),
        rust_read_me,
    )?;
    Ok(())
}
