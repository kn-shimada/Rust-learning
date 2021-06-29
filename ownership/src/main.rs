// Rustの所有権、参照、借用等のサンプルプログラム
fn main() {
    let mut num = 1; // immutable(不変) object
    let _num_ref1 = &num; // reference(参照)
    let _num_ref2 = &num; // 不変参照(&)は複数存在できる
    let _borrow_num_ref2 = &_num_ref2; // 仮の所有権からも仮の所有権を作成できる(原本は作成できない)
    let _num_mut_ref1 = &mut num; // mutable（可変）reference
    let num_mut_ref2 = &mut num; // 可変参照(&mut)は不変参照(&)と同時に存在できず、複数存在できない。
    *num_mut_ref2 = 2; // dereference(参照外し)
                       // 参照(仮の所有権)でオブジェクト使用する際は参照外しが必要
    println!("{}", &num_mut_ref2);
}
