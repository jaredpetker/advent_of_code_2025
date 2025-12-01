#[macro_export]
macro_rules! include_file {
    ( $file:literal ) => {
        {
            println!("Loading file: {}", $file);
            include_str!($file)
        }
    };
}
