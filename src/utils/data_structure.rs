pub trait DataStructure<T> {
    fn push_vec(&mut self, values: Vec<T>);
    fn push_array(&mut self, array: &[T]);
}

impl <T: Copy> DataStructure<T> for Vec<T> {
    fn push_vec(&mut self, values: Vec<T>) {
        for value in values {
            self.push(value);
        }
    }

    fn push_array(&mut self, array: &[T]) {
        for value in array {
            self.push(*value);
        }
    }
}