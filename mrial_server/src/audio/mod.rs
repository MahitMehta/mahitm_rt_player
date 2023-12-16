cfg_if::cfg_if! {
    if #[cfg(target_os = "linux")] {
        mod linux; 
        pub use self::linux::*; 
    } else if #[cfg(target_os = "windows")] {
        mod windows;
        pub use self::windows::*;
    } else {
        //TODO: Fallback implementation.
    }
}

pub struct AudioController {

}