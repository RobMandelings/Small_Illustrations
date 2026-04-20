/// "The original referent does not get used until the lifetime of the loan has expired"
///
/// This is because you borrow mutably and later also borrow immutably, which is not allowed
fn main() {

    let mut a = String::new();
    let b = &a;

    a.push('a');
    println!("{b}");
}