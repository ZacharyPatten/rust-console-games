use std::fs;
//use std::env;

fn main()
{
    //let directory = env::current_dir().unwrap().into_os_string().into_string().unwrap();
    //println!("{}", directory);
    if cfg!(windows)
    {
        let _ = fs::rename("../launch-windows.json", "../launch.json");
    }
    else
    {
        let _ = fs::rename("../launch-other.json", "../launch.json");
    }
}
