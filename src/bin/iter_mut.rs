fn main() {
    let mut values = vec![1, 2, 3];

    // Problem: iterating over &mut Vec moves the reference due to into_iter
    let v = &mut values;
    for x in v {
        println!("x = {}", x);
    }
    // for x in v {          // ERROR: use of moved value `v`
    //     println!("x = {}", x);
    // }

    // Correct: iterate using iter_mut on the Vec itself
    for x in values.iter_mut() {
        *x *= 2;
    }
    println!("values = {:?}", values);
}