//! Robust handling of `gecko_driver` and `shared_http` as Linux processes.
//! 
use anyhow::{anyhow, bail, Error, Result};
use std::{
    env,
    fmt,
    io::{BufRead},
    path::{Path, PathBuf},
    process::{Command, Output},
    str::FromStr,
};

// === linux tool outputs =========================================================================

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Pid(pub u32);

impl FromStr for Pid {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> { Ok(Pid(s.parse()?)) }
}

impl fmt::Display for Pid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Port(pub u32);

impl FromStr for Port {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> { Ok(Port(s.parse()?)) }
}

impl fmt::Display for Port {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, PartialEq)]
pub struct Cmd(pub String);

impl From<&str> for Cmd {
    fn from(s: &str) -> Self { Cmd(s.to_string()) }
}

impl fmt::Display for Cmd {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// === helper functions ===========================================================================

// Takes regex::byte::Regex captures and returns `String` for a given capture position. 
fn byte_capture(captures: &regex::bytes::Captures, pos: usize) -> Result<Option<String>> {
    let a_match = match captures.get(pos) {
        Some(mat) => mat,
        None => { return Ok(None) },
    };
    let bytes = a_match.as_bytes();
    let s = match std::str::from_utf8(bytes) {
        Ok(s) => s.to_string(),
        Err(e) => { return Err(anyhow!(e.to_string())) },
    };
    Ok(Some(s))
}

// Takes regex::Regex captures and returns `String` for a given capture position.
fn capture(captures: &regex::Captures, pos: usize) -> Result<Option<String>> {
    let a_match = match captures.get(pos) {
        Some(mat) => mat,
        None => { return Ok(None) },
    };
    Ok(Some(a_match.as_str().to_string()))
}

// Returns the path to the root of the shared data directory.
fn full_path(path: &Path) -> Result<PathBuf> {
    let current = env::current_dir()?;
    match path.is_absolute() {
        true => Ok(path.to_path_buf()),
        false => {
            let path = current.join(path);
            path.canonicalize().map_err(|_| {
                anyhow!("Failed to find directory {}", path.display())
            })
        }
    }
}

// === Linux commands =============================================================================

/// Return `(PID, port, user)` for all listening TCP sockets.
pub fn pids_ports_cmds() -> Result<Vec<(Pid, Port, Cmd)>> {
    let Output {stdout,..} = Command::new("ss")
        .arg("-lntp")
        .output()?;
    let mut acc = Vec::new();
    for res_line in stdout.lines().skip(1) {
        let line = res_line?;
        let words: Vec<&str> = line.split_whitespace().collect();
        if words.len() < 6 { continue };

        let port: Port = match words[3].split(':').last() {
            Some(p) => {
                match p.parse() {
                    Ok(p) => p,
                    Err(_) => bail!("Failed"),
                }
            },
            None => bail!("Failed to parse port"),
        };
        // users:(("geckodriver",pid=24018,fd=3))
        let re = regex::Regex::new(r#"users:\(\("([a-z_].*)",pid=(\d.*),.*"#)?;
        let captures = match re.captures(words[5]) {
            Some(cap) => cap,
            None => bail!("Failed"),
        };
        let cmd: Cmd = captures.get(1)
            .ok_or(anyhow!("Failed to parse command"))?
            .as_str()
            .into();
        let pid: Pid = captures.get(2)
            .ok_or(anyhow!("Failed to parse pid"))?
            .as_str()
            .parse()?;
        acc.push((pid, port, cmd));
    }
    Ok(acc)
}

/// Return response from Linux ps command for all processes.
pub fn pids_cmds() -> Result<Vec<(Pid, Cmd)>> {
    let Output {stdout,..} = Command::new("ps")
        .arg("-e")
        .output()?;
    let mut acc = Vec::new();
    for res_line in stdout.lines().skip(1) {
        let line = res_line?;
        let pid_str = &line[0..7];
        let pid: Pid = pid_str.trim().parse()?;
        let cmd: Cmd = line[26..].into();
        acc.push((pid, cmd));
    }
    Ok(acc)
}

/// Returns response from the Linux ps command for a given PID.
pub fn cmd_from_pid(pid: Pid) -> Result<Option<Cmd>> {
    let Output {stdout,..} = Command::new("ps")
        .arg("--pid")
        .arg(&format!("{}", pid))
        .output()?;
    match stdout.lines().nth(1) {
        Some(Ok(line)) => {
            let cmd: Cmd = line[26..].into();
            return Ok(Some(cmd))
        },
        None => Ok(None),
        _ => Err(anyhow!("Error getting PID")),
    }
}

// fn shutdown(cmd: Cmd, pid: Pid) -> Result<()> {
//     if let Some(_) = cmd_from_pid(pid)? {
//         Command::new("kill")
//             .arg(pid.to_string())
//             .output()?;
//         println!("{} ({}) was shut down.", cmd, pid);
//         Ok(())
//     } else { 
//         bail!("Process {} does not exist.", pid)
//     }
// }

// === Higher level commands ======================================================================

/// Start `geckodriver`.
pub fn start_geckodriver() -> Result<()> {
    match pids_ports_cmds()?
        .iter()
        .find(|(_, port, cmd)| {
            (port == &Port(4444)) && (cmd == &Cmd::from("geckodriver"))
        })
    {
        Some((_, port, cmd)) => {
            if cmd != &Cmd::from("geckodriver") {
                bail!("4444 is already used by '{}'", cmd)
            } else {
                println!("geckodriver ({}) already listening on 4444.", port)
            }
        },
        None => {
            Command::new("geckodriver").spawn()?;
        },
    }
    Ok(())
}

/// Shutdown `shared_http`.
pub fn shutdown_geckodriver() -> Result<()> {
    let pids = pids_cmds()?;
    match pids.iter().find(|(_, cmd)| cmd == &Cmd::from("geckodriver")) {
        Some((pid, _)) => {
            Command::new("kill").arg(pid.to_string()).output()?;
            println!("geckodriver ({}) was shut down.", pid);
        },
        None => {
            println!("geckodriver not running.");
        },
    }
    Ok(())
}


/// Start `shared_http`.
pub fn start_shared_http<P: AsRef<Path>>(root_dir: P) -> Result<()> {
    let root = root_dir.as_ref().to_path_buf();
    let path = full_path(&root)?;

    // If shared_http is running then shut it down.
    if let Some(_) = pids_cmds()?.iter().find(|(_, cmd)| cmd == &Cmd::from("shared_http")) {
        shutdown_shared_http().unwrap();
    };
    Command::new("shared_http")
        .arg(path)
        .spawn()?;
    println!("shared_http running");
    Ok(())
}

/// Shutdown `shared_http`.
pub fn shutdown_shared_http() -> Result<()> {
    match pids_cmds()?.iter().find(|(_, cmd)| cmd == &Cmd::from("shared_http")) {
        Some((pid, _)) => {
            Command::new("kill").arg(pid.to_string()).output()?;
            println!("shared_http ({}) was shut down.", pid);
        },
        None => {
            println!("shared_http not running.");
        },
    }
    Ok(())
}

// === Tests ======================================================================================

#[cfg(test)]
pub mod test {
    use crate::os_setup::*;
    use regex::Regex;
    use std::{thread, time};
    
    #[test]
    fn regex_should_pick_this_up() {
        let s = r#"tcp   LISTEN 0      128        127.0.0.1:4444    0.0.0.0:*    users:(("geckodriver",pid=27563,fd=3))"#;
        let re = Regex::new(r#"^.*tcp.*LISTEN.*127.0.0.1:4444.*users:\(\(["][a-z].*",pid=([0-9]*),.*$"#).unwrap();
        assert!(re.is_match(s));
        assert_eq!(
            re.captures(s)
                .unwrap()
                .get(1)
                .map_or("", |m| m.as_str()),
            "27563"
        );
    }

    #[test]
    fn regex_should_extract_pid() {
        let re4 = Regex::new("pid=[0-9]*,").unwrap();
        assert!(re4.is_match("pid=211,"));
    }

    #[test]
    fn should_return_pid_and_cmd() {
        assert_eq!(
            cmd_from_pid(Pid(1)).unwrap().unwrap(),
            Cmd::from("systemd"),
        )
    }

    #[test]
    fn should_return_port_pid_cmd() {
        // Test this using gecko_driver
    }

    #[test]
    fn should_start_shared_http() {
        let path = std::env::current_dir().unwrap()
            .join("../../shared_data");
        start_shared_http(path).unwrap();

        // Give shared_http time to connect to port
        thread::sleep(time::Duration::from_secs(1));

        assert!(
            pids_ports_cmds().unwrap()
                .iter()
                .find(|(_, port, cmd)| {
                    port == &Port(8080) &&
                    cmd == &Cmd::from("shared_http")
                }).is_some()
        )
    }

    #[test]
    fn should_shutdown_shared_http() {
        shutdown_shared_http().unwrap();
        assert!(
            pids_ports_cmds().unwrap()
                .iter()
                .find(|(_, port, cmd)| {
                    port == &Port(8080) &&
                    cmd == &Cmd::from("shared_http")
                }).is_none()
        )
    }

    #[test]
    fn should_start_geckodriver() {
        // We want to check two conditions. If we can start geckodriver when it is
        // not already running and when it is already running. We divide the test
        // into these two cases.
        //
        match pids_ports_cmds().unwrap()
            .iter()
            .find(|(_, port, cmd)| (port == &Port(4444)) && (cmd == &Cmd::from("geckodriver")))
        {
            Some((_, _, _)) => {

                // If geckodriver is already running
                if let Ok(_) = start_geckodriver() {assert!(true)} else {assert!(false)}

                // And then test if it starts from shutdown 
                shutdown_geckodriver().unwrap();

                if let Ok(_) = start_geckodriver() {assert!(true)} else {assert!(false)}
            },
            None => {

                // If geckodriver is already shutdown
                if let Ok(_) = start_geckodriver() {assert!(true)} else {assert!(false)}

                // And then test if it starts when already running
                if let Ok(_) = start_geckodriver() {assert!(true)} else {assert!(false)}
            }
        }
    }

    #[test]
    fn should_shutdown_geckodriver() {
        start_geckodriver().unwrap();
        if let Ok(_) = shutdown_geckodriver() {assert!(true)} else {assert!(false)}

        assert!(
            pids_ports_cmds().unwrap()
                .iter()
                .find(|(_pid, port, cmd)| (port == &Port(4444)) && (cmd == &Cmd::from("geckodriver")))
                .is_none()
        )
    }
}
