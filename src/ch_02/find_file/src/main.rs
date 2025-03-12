use std::{env, path};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("find_file (path) (keyword)");
        return;
    }

    let target_dir = &args[1];
    let keyword = &args[2];

    let target = path::PathBuf::from(target_dir);
    find_file(&target, keyword);
}


fn find_file(target: &path::PathBuf, keyword: &str) {
    let files = target.read_dir().expect("존재하지 않는 경로");

    for dir_entry in files {
        let path = dir_entry.unwrap().path();

        if path.is_dir() {
            find_file(&path, keyword);
            continue;
        }

        let file_name = path.file_name().unwrap().to_string_lossy();

        if None == file_name.find(keyword) {
            continue;
        }

        println!("{}", path.to_string_lossy());
    }
}
