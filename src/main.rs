use std::fs;
use std::io;



fn main() {
    std::process::exit(real_main());
    //real_main();
}


fn real_main() -> i32 {

    let args: Vec<_> = std::env::args().collect();

    for (index,arg) in args.iter().enumerate(){
        println!("{}: {}", index, arg);
    }

    if args.len() < 2 {
        println!("Usage: {} <filename>", args[0]);
        return 1;
    }else {
        let fname = std::path::Path::new(&*args[1]);
        let file = match fs::File::open(&fname) {
            Ok(f) => f,
            Err(e) => {
                println!("Error opening file: {}", e);
                return 1;
            }
        };
        let mut archive = zip::ZipArchive::new(file).unwrap();

        for i in 0..archive.len() {
            let mut file = archive.by_index(i).unwrap();

            let outpath = match file.enclosed_name() {
                Some(path) => path.to_owned(),
                None => continue,
            };
            {
                let comment = file.comment();
                if !comment.is_empty() {
                    println!("File {} comment: {}", i, comment);
                }
            }

            if (&*file.name()).ends_with('/') {
                println!("File {} extracted to \"{}\"", i, outpath.display());
                fs::create_dir_all(&outpath).unwrap();
            } else {
                println!("File {} extracted to \"{}\" ({} bytes)", i, outpath.display(), file.size());
                if let Some(p) = outpath.parent() {
                    if !p.exists() {
                        fs::create_dir_all(&p).unwrap();
                    }
                }
                let mut outfile = fs::File::create(&outpath).unwrap();
                io::copy(&mut file, &mut outfile).unwrap();
            }
            #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            //let perms = archive.by_index(0).unwrap().unix_mode().unwrap();
            
            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
            }
            
        }
            
        }
        

        return 0;
    }


}
