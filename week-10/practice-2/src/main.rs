fn main() {
    let v = vec![10, 20, 30];
    // v owns the object in the heap

    let v2 = v;
    // ownership moved from v to v2

    display(&v2); 
    // we borrow v2 instead of moving it

    println!("In main {:?}", v2);
    // v2 is still usable because it was only borrowed
}

fn display(v: &Vec<i32>) {
    println!("inside display {:?}", v);
}
