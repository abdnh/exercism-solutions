/// What should the type of function be?
pub fn map<T, O, F>(input: Vec<T>, mut function: F) -> Vec<O>
where
    F: FnMut(T) -> O,
{
    let mut output: Vec<_> = vec![];
    for element in input {
        output.push(function(element));
    }
    output
}
