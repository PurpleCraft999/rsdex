use std::{fs::File, io::Read, path::Path};

fn main() {
    include_readme().unwrap();
}

fn remove_useless_markdown(string: &mut String) {
    

    // let bytes = string.bytes();
    let headers = ["<h2>","<h3>"];
    remove_from_string(string, &headers);
    // let closing_headers= ["</h2>"];
    let closing_headers = headers.iter().map(|header|{let mut h =header.to_string(); h.insert(1, '/'); h}  ).collect::<Vec<String>>();
    let closing_headers=closing_headers.iter().map(|s|s.as_str()).collect::<Vec<&str>>();
                    println!("cargo::warning={closing_headers:?}");

    remove_from_string(string, &closing_headers);


    remove_from_string(string, &["# rsdex &emsp; [![Latest Version]][crates.io]  [![Latest Release Banner]][Latest Release]"]);
}

fn remove_from_string(string: &mut String,to_remove:&[&str]){
    //I spent way to long on this only to find replace i might just be stupid

    // let chars: Vec<(usize, char)> = string.char_indices().collect();
    // //becuase we remove the char pos is no longer accurate so we offset it
    // let mut offset = 0;

    // for removeable in to_remove {
    //     for str in chars.windows(removeable.len()) {
    //         if str
    //             .iter()
    //             .map(|(_, c)| c)
    //             .collect::<String>()
    //             .contains(removeable)
    //         {
    //             println!("cargo::warning={str:?}");
    //             for idx in str {
    //                 string.remove(idx.0 - offset);
    //                 offset += 1;
    //             }
    //         }
    //     }
    // }

    for remove in to_remove{
        *string = string.replace(*remove, "");
    }

}



fn include_readme() -> std::io::Result<()> {
    println!("cargo::rerun-if-changed=README.md");
    let mut file = File::open(env!("CARGO_PKG_README"))?;
    let mut read_me = String::new();
    file.read_to_string(&mut read_me)?;
    remove_useless_markdown(&mut read_me);

    let rust_read_me = format!("pub const READ_ME:&str = r##\"{}\"##;", read_me.replace('\r', ""));
    std::fs::write(
        Path::new(&std::env::var_os("OUT_DIR").unwrap()).join("readme.rs"),
        rust_read_me,
    )?;
    Ok(())
}
