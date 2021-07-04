// コピートレイト(Copy Trait)のサンプルプログラムです
fn main() {
    let num1 = 1; // immutable object
    let num1_copy = num1; // 値をコピーして新しいオブジェクトと所有権を作成
    let num1_ref = &num1; // reference
    let num1_ref_copy = num1_ref; // 不変参照(&)はコピートレイトを実装している
    println!(
        "num1: {}, num1_copy: {}, num1_ref: {}, num1_ref_copy: {}",
        num1, num1_copy, num1_ref, num1_ref_copy
    );

    let mut num2 = 2; // mutable object
    let num2_mut_ref = &mut num2; // mutable reference
    let num2_mut_ref_move = num2_mut_ref; // 可変参照は複数存在できないため、コピートレイトを実装していない
                                                  // 可変参照の場合は所有権が移動する
    println!("num2_mut_ref_copy: {}", num2_mut_ref_move);
}
