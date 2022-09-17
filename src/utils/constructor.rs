pub trait Constructor<I> {
    fn new(input: I) -> Self;
}