enum OS {
    Windows(u32, String),
    Mac(u32, String),
    Linux(u32, String),
}

pub fn run() {
    let linux = OS::Linux(1991, String::from("linux"));
    let windows = OS::Windows(1995, String::from("Microsoft"));
    let mac = OS::Mac(2001, String::from("Mac"));

    print_os_info(mac);
}

fn print_os_info(os: OS) {
    match os {
        OS::Windows(year, who) => {
            println!("{}{}", year, who)
        }
        OS::Mac(year, who) => {
            println!("{}{}", year, who)
        }
        OS::Linux(year, who) => {
            println!("{}{}", year, who)
        }
    }
}
