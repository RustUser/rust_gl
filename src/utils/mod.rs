pub mod constructor;
pub mod data_structure;
pub mod macros;

pub fn flatten<T: Copy>(values: Vec<&[T]>) -> Vec<T> {
    let mut output = vec![];
    for row in values {
        for column in row {
            output.push(*column);
        }
    }
    output
}

pub fn flatten_3<T: Copy>(values: Vec<[T; 3]>) -> Vec<T> {
    let mut output = vec![];
    for row in values {
        for column in row {
            output.push(column);
        }
    }
    output
}