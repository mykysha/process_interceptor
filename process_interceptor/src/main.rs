use std::{fs::File, io::Read};
use std::io::Seek;


fn read_memory(pid: i32, address: usize, buffer: &mut [u8]) -> Result<(), std::io::Error> {
    let mut file = File::open(format!("/proc/{}/mem", pid))?;
    file.seek(std::io::SeekFrom::Start(address as u64))?;
    file.read_exact(buffer)?;
    Ok(())
}

fn lock_page(address: *mut u8, length: usize) -> Result<(), std::io::Error> {
    let result = unsafe { libc::mlock(address as *const libc::c_void, length) };
    if result == -1 {
        Err(std::io::Error::last_os_error())
    } else {
        Ok(())
    }
}

fn intercept_input(device_id: usize) -> Result<(), std::io::Error> {
    let mut file = File::open(format!("/dev/input/event{}", device_id))?;
    let mut buffer = [0; 24];

    loop {
        file.read_exact(&mut buffer)?;
        let time_sec = i64::from_ne_bytes(buffer[0..8].try_into().unwrap());
        let time_usec = i64::from_ne_bytes(buffer[8..16].try_into().unwrap());
        let event_type = u16::from_ne_bytes(buffer[16..18].try_into().unwrap());
        let event_code = u16::from_ne_bytes(buffer[18..20].try_into().unwrap());
        let value = i32::from_ne_bytes(buffer[20..24].try_into().unwrap());

        println!(
            "Device {}: {:>8} sec {:>6} usec type {:>4} code {:>4} value {}",
            device_id, time_sec, time_usec, event_type, event_code, value
        );
    }
}

fn main() -> Result<(), std::io::Error> {
    let pid = 74171;
    // Read memory from another process
    let mut buffer = [0; 4];
    read_memory(pid, 0x7fff_ffff_fff0usize, &mut buffer)?;
    println!("Read value: {:?}", i32::from_ne_bytes(buffer));

    // Make page non-paged
    let mut data = vec![0u8; 4096];
    lock_page(data.as_mut_ptr(), data.len())?;
    println!("Page locked");

    // Intercept input events
    intercept_input(0)?;

    Ok(())
}

