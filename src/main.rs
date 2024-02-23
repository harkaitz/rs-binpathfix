use getopts::Options;
use std::env;
use std::fs;

const HELP: &str =
r#"Usage: binpathfix [-v][-o OLDPATH][-n NEWPATH] [FILES...]

Replace OLDPATH (by default current directory) with "/tmp/HASH" in
the specified binary files (normally ELF executables).

The hash is computed from the OLDPATH, If -n is specified a symlink
is created from "/tmp/HASH" to the NEWPATH.

If -v is specified, the program will print the changes it makes."#;

pub fn main() {
    let tmppath: &str = "/tmp";

    // Parse command line arguments.
    let mut optcfg = Options::new(); {
        optcfg.optflag("h", "help", "print this help message");
        optcfg.optflag("v", "verbose", "print changes made");
        optcfg.optopt("o", "oldpath", "old path to replace", "OLDPATH");
        optcfg.optopt("n", "newpath", "new path to replace with", "NEWPATH");
    }
    let cmdargs = match optcfg.parse(env::args().skip(1)) {
        Ok(m) => m,
        Err(f) => { panic!("{}", f.to_owned()) }
    };
    if cmdargs.opt_present("h") {
        println!("{}", HELP);
        return;
    }
    let verbose: bool = cmdargs.opt_present("v");

    // Get path to replace in binary.
    let oldpath: String =
        if cmdargs.opt_present("o") {
            cmdargs.opt_str("o").unwrap().trim_end_matches('/').to_string()
        } else {
            std::env::current_dir().unwrap().as_path().to_str().unwrap().to_string()
        };
    let oldpath_b = oldpath.as_bytes();

    // Calculate subpath.
    let oldpath_hash: &str = &format!("{:x}", md5::compute(oldpath.clone()))[0..4];
    let oldpath_sz: usize = oldpath.len();
    let subpath_sz: usize = tmppath.len() + oldpath_hash.len() + 1;
    if oldpath_sz < subpath_sz {
        panic!("{}: Path specified with -o is too short to be replaced in binary.", oldpath);
    }
    let subpath: String = "/".repeat(oldpath_sz - subpath_sz).to_string() + tmppath + "/" + oldpath_hash;
    
    // Create symlink in Unix systems.
    if cmdargs.opt_present("n") {
        let path: String = cmdargs.opt_str("n").unwrap().trim_end_matches('/').to_string();
        let path_r: String = std::fs::canonicalize(path).unwrap().to_str().unwrap().to_string();
        if verbose {
            println!("NOT IMPLEMENTED - Creating symlink from {} to {}", path_r, subpath);
        }
    }

    // For each argument.
    for arg in cmdargs.free {
        
        // Read file.
        let mut idata = match fs::read(&arg) {
            Ok(f) => f,
            Err(e) => { panic!("{}", e) }
        };
        
        // Replace oldpath with subpath.
        let mut pos: usize = 0;
        let mut found: bool = false;
        while pos < idata.len() {
            if idata[pos..].starts_with(oldpath_b) {
                if verbose {
                    println!("{}: Replacing in byte {}.", arg, pos);
                }
                idata.splice(pos..pos+oldpath_sz, subpath.as_bytes().iter().cloned());
                pos += subpath_sz;
                found = true;
            } else {
                pos += 1;
            }
        }
        
        // Write file.
        if found {
            match fs::write(&arg, idata) {
                Ok(_) => (),
                Err(e) => { panic!("{}", e) }
            };
        }
        
    }

}
