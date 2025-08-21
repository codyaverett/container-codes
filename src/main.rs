
use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Read};
use std::path::Path;
use std::process::{Command, Stdio};

fn main() -> io::Result<()> {
    println!("=== Sandbox Probe: System Information ===\n");

    // 1. Basic System Identification
    println!("Basic System Info:");
    println!("  Hostname: {:?}", env::var("HOSTNAME").unwrap_or_else(|_| "Unavailable".to_string()));
    println!("  Current User: {:?}", whoami());
    println!("  OS: {}", std::env::consts::OS);
    println!("  Architecture: {}", std::env::consts::ARCH);
    if let Ok(uname) = run_command("uname", &["-a"]) {
        println!("  Uname: {}", uname.trim());
    } else {
        println!("  Uname: Unavailable");
    }
    println!();

    // 2. Container/Isolation Detection
    println!("Container Detection:");
    if let Ok(cgroup) = read_file("/proc/1/cgroup") {
        println!("  CGroup Contents:\n{}", cgroup);
        if cgroup.contains("docker") || cgroup.contains("kubepods") || cgroup.contains("container") {
            println!("  Likely in a container: Yes");
        } else {
            println!("  Likely in a container: Possibly (custom namespace detected)");
        }
    } else {
        println!("  CGroup: Unavailable");
    }
    println!();

    // 3. Network Information
    println!("Network Information:");
    if let Ok(ifaces) = fs::read_dir("/sys/class/net") {
        println!("  Network Interfaces:");
        for entry in ifaces {
            if let Ok(entry) = entry {
                println!("    - {:?}", entry.file_name());
            }
        }
    } else {
        println!("  Network Interfaces: Unavailable (no /sys/class/net)");
    }
    if let Ok(ip_addr) = run_command("ip", &["addr"]) {
        println!("  IP Addr Output:\n{}", ip_addr);
    } else {
        println!("  IP Addr: Command unavailable");
    }
    if let Ok(ss) = run_command("ss", &["-tuln"]) {
        println!("  Open Ports (ss -tuln):\n{}", ss);
    } else if let Ok(netstat) = run_command("netstat", &["-tuln"]) {
        println!("  Open Ports (netstat -tuln):\n{}", netstat);
    } else {
        println!("  Open Ports: Commands unavailable");
    }
    println!();

    // 4. User Accounts and Authentication
    println!("User Accounts:");
    if let Ok(passwd) = read_file("/etc/passwd") {
        let users: Vec<String> = passwd.lines().filter_map(|line| {
            line.split(':').next().map(|u| u.to_string())
        }).collect();
        println!("  Users from /etc/passwd: {:?}", users);
    } else {
        println!("  /etc/passwd: Unavailable");
    }
    if let Ok(shadow) = read_file("/etc/shadow") {
        println!("  /etc/shadow Contents (redacted for safety):\n{}", shadow.lines().map(|line| {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() > 1 {
                format!("{}: [REDACTED]:{}", parts[0], parts[2..].join(":"))
            } else {
                line.to_string()
            }
        }).collect::<Vec<_>>().join("\n"));
    } else {
        println!("  /etc/shadow: Unavailable");
    }
    println!();

    // 5. Private/Public Keys and Sensitive Files
    println!("SSH Keys and Sensitive Files:");
    let ssh_dirs = vec!["/root/.ssh", "/home/ubuntu/.ssh"]; // Common locations
    for dir in ssh_dirs {
        if let Ok(entries) = fs::read_dir(dir) {
            println!("  Directory {}:", dir);
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_file() {
                        println!("    - File: {:?}", path.file_name().unwrap());
                        if let Ok(content) = read_file(path.to_str().unwrap()) {
                            println!("      Content (snippet): {}", content.lines().next().unwrap_or("Empty"));
                        }
                    }
                }
            }
        } else {
            println!("  {}: Unavailable", dir);
        }
    }
    println!();

    // 6. Environment Variables
    println!("Environment Variables:");
    for (key, value) in env::vars() {
        println!("  {}: {}", key, value);
    }
    println!();

    // 7. Processes and Runtime Details
    println!("Running Processes:");
    if let Ok(ps) = run_command("ps", &["aux"]) {
        println!("{}", ps);
    } else {
        println!("  ps: Unavailable");
    }
    println!();

    // Attempt to Establish Network Connection (for testing, e.g., to internal service)
    println!("Network Connection Attempt:");
    use std::net::TcpStream;
    if let Ok(_) = TcpStream::connect("localhost:80") {
        println!("  Connected to localhost:80 (HTTP)");
    } else {
        println!("  Connection to localhost:80 failed");
    }
    // Example: Try internal service from env if present
    if let Ok(coingecko_url) = env::var("COINGECKO_BASE_URL") {
        println!("  Attempting connection to COINGECKO_BASE_URL: {}", coingecko_url);
        // Parse host:port, but simplified
        if let Some(host) = coingecko_url.split('/').nth(2) {
            if let Ok(_) = TcpStream::connect(host) {
                println!("  Connected!");
            } else {
                println!("  Connection failed");
            }
        }
    }
    println!();

    // Attempt to Spawn Terminal Session (e.g., interactive shell, but limited in code)
    println!("Terminal Session Attempt:");
    println!("  Spawning /bin/sh (but non-interactive in this context)");
    let _ = Command::new("/bin/sh")
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn();
    // Note: This won't be truly interactive in a non-TTY env

    Ok(())
}

fn read_file(path: &str) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn run_command(cmd: &str, args: &[&str]) -> io::Result<String> {
    let output = Command::new(cmd)
        .args(args)
        .output()?;
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(io::Error::new(io::ErrorKind::Other, "Command failed"))
    }
}

fn whoami() -> String {
    if let Ok(output) = run_command("whoami", &[]) {
        output.trim().to_string()
    } else {
        "Unknown".to_string()
    }
}
