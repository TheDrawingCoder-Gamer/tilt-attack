use skyline::hooks::InlineCtx;
use std::fmt::Display;

#[skyline::from_offset(0x37a1270)]
pub unsafe fn set_text_string(pane: u64, string: *const u8);

pub unsafe fn get_pane_by_name(arg: u64, arg2: *const u8) -> [u64; 4] {
    let func_addr =
        (skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as *mut u8).add(0x37752e0);
    let callable: extern "C" fn(u64, *const u8, ...) -> [u64; 4] = std::mem::transmute(func_addr);
    callable(arg, arg2)
}

unsafe fn set_room_text(arg: u64, string: String) {
    let func_addr =
        (skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as *mut u8).add(0x3778c50);
    let callable: extern "C" fn(u64, *const u8, usize, *const u16, ...) =
        std::mem::transmute(func_addr);
    callable(
        arg,
        b"mnu_online_room_inside_room_id\0".as_ptr(),
        1,
        string.encode_utf16().collect::<Vec<u16>>().as_ptr(),
    )
}

static mut CURRENT_PANE_HANDLE: usize = 0;
static mut CURRENT_ARENA_ID: String = String::new();
static mut CURRENT_INPUT_BUFFER: isize = -1;
static mut MOST_RECENT_AUTO: isize = -1;

const MAX_INPUT_BUFFER: isize = 25;
const MIN_INPUT_BUFFER: isize = -1;

#[skyline::hook(offset = 0x1a12460)]
unsafe fn update_css2(arg: u64) {
    static mut CURRENT_COUNTER: usize = 0;
    if ninput::any::is_press(ninput::Buttons::X) {
        if CURRENT_COUNTER == 0 {
            CURRENT_INPUT_BUFFER += 1;
        }
        CURRENT_COUNTER = (CURRENT_COUNTER + 1) % 10;
    } else if ninput::any::is_press(ninput::Buttons::Y) {
        if CURRENT_COUNTER == 0 {
            CURRENT_INPUT_BUFFER -= 1;
        }
        CURRENT_COUNTER = (CURRENT_COUNTER + 1) % 10;
    } else {
        CURRENT_COUNTER = 0;
    }

    CURRENT_INPUT_BUFFER = CURRENT_INPUT_BUFFER.clamp(MIN_INPUT_BUFFER, MAX_INPUT_BUFFER);
    set_text_string(
        *((*((arg + 0xe58) as *const u64) + 0x10) as *const u64),
        format!("Input Latency: {}\0", CURRENT_INPUT_BUFFER).as_ptr(),
    );
    set_text_string(
        *((*((arg + 0xe68) as *const u64) + 0x10) as *const u64),
        format!("Input Latency: {}\0", CURRENT_INPUT_BUFFER).as_ptr(),
    );
    call_original!(arg)
}
