const FD_STDOUT: usize = 1;

fn in_range(buf: *const u8, len: usize, range: (usize, usize)) -> bool {
    let buf = buf as usize;
    buf >= range.0 && buf + len <= range.1
}

fn in_app_bin_range(buf: *const u8, len: usize) -> bool {
    in_range(buf, len, crate::batch::get_current_app_bin_range())
}

fn in_app_stack_range(buf: *const u8, len: usize) -> bool {
    in_range(buf, len, crate::batch::get_current_app_stack_range())
}

fn check_app_mem_space(buf: *const u8, len: usize) -> bool {
    in_app_bin_range(buf, len) || in_app_stack_range(buf, len)
}

pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    match fd {
        FD_STDOUT => {
            log::debug!("sys_write: buf = 0x{:016x}, len = {}", buf as usize, len);

            // Check that the specified buffer is fully within the application's memory space.
            if !check_app_mem_space(buf, len) {
                log::warn!("[kernel] Invalid user address range: [0x{:016x}, 0x{:016x})", buf as usize, buf as usize + len);
                return -1isize;
            }

            let slice = unsafe { core::slice::from_raw_parts(buf, len) };
            let str = core::str::from_utf8(slice).unwrap();
            print!("{}", str);

            log::debug!("sys_write normally returns with return value be {}", len);
            len as isize
        },
        _ => -1isize,
    }
}