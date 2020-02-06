///
/// Marker of bastion API.
#[doc(hidden)]
macro_rules! bastion {
    ($($block:item)*) => {
        $(
            #[cfg(feature = "runtime_bastion")]
            $block
        )*
    }
}

///
/// Marker of async-std API.
#[doc(hidden)]
macro_rules! asyncstd {
    ($($block:item)*) => {
        $(
            #[cfg(feature = "runtime_asyncstd")]
            $block
        )*
    }
}

///
/// Marker of async-std API.
#[doc(hidden)]
macro_rules! tokio {
    ($($block:item)*) => {
        $(
            #[cfg(feature = "runtime_tokio")]
            $block
        )*
    }
}

///
/// Marker of no-std API.
#[doc(hidden)]
macro_rules! nostd {
    ($($block:item)*) => {
        $(
            #[cfg(feature = "runtime_nostd")]
            $block
        )*
    }
}
