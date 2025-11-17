use libc;
use libc::{close, dup, dup2};
use std::io::Error;
use std::io::Result;

pub struct FdRedirectGuard {
    saved_fd: i32,
    target_fd: i32,
}

impl FdRedirectGuard {
    pub fn new(target_fd: i32, file_fd: i32) -> Result<Self> {
        let saved = unsafe { dup(target_fd) };
        if saved < 0 {
            return Err(Error::last_os_error());
        }

        if unsafe { dup2(file_fd, target_fd) } < 0 {
            let e = Error::last_os_error();
            unsafe { close(saved) };
            return Err(e);
        }

        Ok(FdRedirectGuard {
            target_fd,
            saved_fd: saved,
        })
    }
}

impl Drop for FdRedirectGuard {
    fn drop(&mut self) {
        if self.saved_fd > 0 {
            unsafe { dup2(self.saved_fd, self.target_fd) };
            unsafe { close(self.saved_fd) };
            self.saved_fd = -1;
        }
    }
}
