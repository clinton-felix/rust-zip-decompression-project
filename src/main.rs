use std::fs;
use std::io;

fn main() {
    std::process::exit(extract_file())
}

fn extract_file() -> i32{
    let args: Vec<_> = std::env::args().collect();
    //if args less than 2, there's an issue because  you need to send the name of the 
    // zip file and it'll show you how to use
    if args.len() < 2 {
        // args[0] is cargo new line
        eprintln!("Usage: {} <filename>", args[0]);
        return 1;
    }

    //args at the 2nd position, denoted by 1st index is the file name
    let fname = std::path::Path::new(&*args[1]);
    //open the file using standard fs
    let file = fs::File::open(&fname).unwrap();

    //using the archive reader function
    let mut archive = zip::ZipArchive::new(file).unwrap();  // creating the destination folder

//start from 0 and cover the entire length of archive
// there will be multiple files in the zip archive and we need to extract all
    for i in 0..archive.len() {
        // go through all the files in the zip folder 1 by 1 using by_index() method and store every instance in mutable file variable
        let mut file = archive.by_index(i).unwrap();

//setting the path where the files will be extracted
        let outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };
        {
            let comment = file.comment();
            if !comment.is_empty(){
                println!("File {} comment: {}", i, comment)
            }
        }

        //the zip can contain other folders too
        /* Maintain the file structure in reading through the zip file content */
        if (*file.name()).ends_with('/'){
            println!("File {} extracted to \"{}\"", i, outpath.display());
            fs::create_dir_all(&outpath).unwrap()
        } else {
            println!(
                "FIle {} extracted to \"{}\" ({} bytes)",
                i,
                outpath.display(),
                file.size()
            );
            if let Some(p) = outpath.parent(){
                if !p.exists(){
                    fs::create_dir_all(&p).unwrap()
                }
            }


            /* Copy from the file into the outfile recursively */
            let mut outfile = fs::File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }

        /* Get and set permisions to allow the user to read the fileS */
        #[cfg(unix)]
        {
            use::std::os::unix::fs::PermisionsExt;

            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&outpath, Permissions::from_mode(mode)).unwrap();
            }
        }
    }

    0
}