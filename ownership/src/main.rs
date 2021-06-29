fn main() {
    let mut num = 1;
    let _num_ref1 = &num; 
    let _num_ref2 = &num;
    let _borrow_num_ref2 = &_num_ref2;
    let _num_mut_ref1 = &mut num;
    let num_mut_ref2 = &mut num;
    *num_mut_ref2 = 2;
    println!("{}", &num_mut_ref2);
}
