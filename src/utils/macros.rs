#[macro_export]
macro_rules! flatten {
    ($expr:expr) => ({
        let size = {
            let mut size = 0;

            for e in &$expr {
                size += e.len();
            }

            size
        };
        size
    })
}