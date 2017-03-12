//adjusted this tutorial for windows
//https://damienradtke.com/post/rusty-dynamic-loading/

extern crate libloading;
extern crate fs_extra;

use std::time::SystemTime;

use fs_extra::file::{copy , CopyOptions};
use libloading::Library;  

const LIB_PATH: &'static str = "../app/target/debug/app.dll";
const LIB_TEMP_PATH: &'static str = "../app_temp.dll";

struct Application(Library);
impl Application {
    fn get_message(&self) -> &'static str {
        unsafe {
            let f = self.0.get::<fn() -> &'static str> (
                b"get_message\0"
            ).unwrap();
            f()
        }
    }
}

fn main() {
    
    let options: CopyOptions = CopyOptions {
        overwrite: true,
        skip_exist: false,
        buffer_size: 0
    };

    copy(LIB_PATH, LIB_TEMP_PATH, &options).unwrap();

    let mut app = Application(Library::new(LIB_TEMP_PATH)
        .unwrap_or_else(|error| panic!("{}", error)));

    let mut last_modified = std::fs::metadata(LIB_PATH).unwrap().modified().unwrap();
    let dur = std::time::Duration::from_secs(3);
    
    loop {
        std::thread::sleep(dur);
        let result = swap_module_on_windows(app, last_modified).unwrap();
        app = result.0;
        last_modified = result.1;
        println!("message: {}", app.get_message());
    }
}

fn swap_module_on_windows(mut app: Application, last_modified: SystemTime) -> Result<(Application, SystemTime), fs_extra::error::Error> {
    let modified = std::fs::metadata(LIB_PATH)?.modified()?;

    if modified > last_modified {
        drop(app);
        let options: CopyOptions = CopyOptions {
            overwrite: true,
            skip_exist: false,
            buffer_size: 0
        };

        copy(LIB_PATH, LIB_TEMP_PATH, &options)?;
        app = Application(Library::new(LIB_TEMP_PATH)?);
    }
    Ok((app, modified))
}