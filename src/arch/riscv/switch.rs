use crate::consts::SwitchMode;

extern "C" {
    fn _switch_to_user(frame: usize) -> !;
}

pub fn switch(frame: usize, mode: SwitchMode) -> ! {
    match mode {
        SwitchMode::User => {
            unsafe { _switch_to_user(frame) };
        }
        _ => {
            panic!("{} not supported", mode);
        }
    }
}
