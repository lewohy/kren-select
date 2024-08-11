use clap::{Parser, ValueEnum};
use windows::Win32::{
    Foundation::{HWND, LPARAM, WPARAM},
    UI::{
        Input::{
            Ime::ImmGetDefaultIMEWnd,
            KeyboardAndMouse::{
                SendInput, INPUT, INPUT_0, INPUT_KEYBOARD, KEYBDINPUT, KEYBD_EVENT_FLAGS,
                KEYEVENTF_KEYUP, VIRTUAL_KEY,
            },
        },
        WindowsAndMessaging::{GetForegroundWindow, SendMessageW, WM_IME_CONTROL},
    },
};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(value_enum)]
    mode: Mode,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Mode {
    KR,
    EN,
}

fn main() {
    let cli = Cli::parse();

    match cli.mode {
        Mode::EN => {
            if is_korean_ime() {
                change_ime_state();
            }
        }
        Mode::KR => {
            if !is_korean_ime() {
                change_ime_state();
            }
        }
    }
}

fn is_korean_ime() -> bool {
    let hwnd: HWND = unsafe { GetForegroundWindow() };
    let ime_hwnd: HWND = unsafe { ImmGetDefaultIMEWnd(hwnd) };

    let state = unsafe { SendMessageW(ime_hwnd, WM_IME_CONTROL, WPARAM(0x0005), LPARAM(0)) };

    state.0 == 1
}

fn change_ime_state() {
    let down_input = INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: VIRTUAL_KEY(0x15), // VK15 key code
                wScan: 0,
                dwFlags: KEYBD_EVENT_FLAGS(0),
                time: 0,
                dwExtraInfo: 0,
            },
        },
    };

    let up_input = INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: VIRTUAL_KEY(0x15), // VK15 key code
                wScan: 0,
                dwFlags: KEYEVENTF_KEYUP,
                time: 0,
                dwExtraInfo: 0,
            },
        },
    };

    unsafe {
        SendInput(&[down_input], std::mem::size_of::<INPUT>() as i32);
        SendInput(&[up_input], std::mem::size_of::<INPUT>() as i32);
    }
}
