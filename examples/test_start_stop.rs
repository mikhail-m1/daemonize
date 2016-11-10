extern crate daemonize;

use std::io::prelude::*;
use daemonize::Daemonize;
use std::env;
use std::fs;

fn main() {
    if env::args().len() != 4 {
        panic!("use {} pidfile <start/stop> result_file",
               env::args().next().unwrap());
    }

    let args: Vec<_> = env::args().collect();
    let (pid, action, result_file) = (&args[1], &args[2], &args[3]);
    let mut file = fs::File::create(result_file).unwrap();
    let daemonize = Daemonize::new().pid_file(pid);

    let (result, sleep_time) = match action.as_str() {
        "start" => {
            (match daemonize.start() {
                Ok(()) => "ok".to_string(),
                Err(e) => format!("{:?}", e),
            },
             10)
        }
        "stop" => {
            (match daemonize.stop() {
                Ok(()) => "ok".to_string(),
                Err(e) => format!("{:?}", e),
            },
             0)
        }
        _ => panic!("unknown command"),
    };

    file.write_all(&result.into_bytes()).unwrap();
    std::thread::sleep(std::time::Duration::from_secs(sleep_time));
}
