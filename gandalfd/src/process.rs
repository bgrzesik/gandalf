
extern crate std;

#[cfg(windows)]
extern crate user32;
#[cfg(windows)]
extern crate kernel32;
#[cfg(windows)]
extern crate winapi;

#[cfg(unix)]
extern crate libc;


pub struct Process {
    #[cfg(windows)]
    handle: u32,

    #[cfg(unix)]
    handle: std::process::Child,
}

impl Process {
    #[cfg(windows)]
    pub fn start(what: &str, console: bool) -> Option<Process> {
        use std::mem::zeroed;
        use std::ptr::{null, null_mut};
        use std::os::windows::ffi::OsStrExt;
        use std::ffi::OsStr;
        use std::iter::once;
        use self::winapi::winbase::CREATE_NO_WINDOW;
        use self::winapi::processthreadsapi::{STARTUPINFOW, PROCESS_INFORMATION};

        let mut si = unsafe { zeroed::<STARTUPINFOW>() };
        let mut pi = unsafe { zeroed::<PROCESS_INFORMATION>() };

        let mut wide: Vec<u16> = OsStr::new(what).encode_wide().chain(once(0)).collect();

        let flags = if !console { CREATE_NO_WINDOW } else { 0 };

        unsafe {
            if kernel32::CreateProcessW(null(),
                                        wide.as_mut_ptr(),
                                        null_mut(),
                                        null_mut(),
                                        1,
                                        flags,
                                        null_mut(),
                                        null_mut(),
                                        &mut si,
                                        &mut pi) == 0 {
                println!("ERROR UNABLE TO START PROCCESS {}",
                         kernel32::GetLastError());
                return None;
            }
        };

        return Some(Process { handle: pi.dwProcessId });
    }

    #[cfg(unix)]
    pub fn start(what: &str, _console: bool) -> Option<Process> {
        use std::process::Command;

        let mut args = what.split(' ');

        let mut cmd = if let Some(exec) = args.next() {
            Command::new(exec)
        } else {
            return None;
        };

        for arg in args {
            cmd.arg(arg);
        }

        if let Ok(child) = cmd.spawn() {
            return Some(Process { handle: child });
        } else {
            return None;
        }
    }
}

impl std::ops::Drop for Process {
    #[cfg(windows)]
    fn drop(&mut self) {
        use self::winapi::winnt::{SYNCHRONIZE, PROCESS_TERMINATE, HANDLE};
        use self::kernel32::{OpenProcess, TerminateProcess};

        let explorer: HANDLE;

        unsafe {
            explorer = OpenProcess(SYNCHRONIZE | PROCESS_TERMINATE, 1, self.handle);
            TerminateProcess(explorer, 0);
        };
    }

    #[cfg(unix)]
    fn drop(&mut self) {
        let _ = self.handle.kill();
    }
}
