mod board;
mod game_manager;
mod sandbox;

fn main() {}

struct Example {
    value: i32,
}
impl Example {
    fn change(&mut self) {}
    fn get(&self, i: &i32) {}
    fn unchange(&self) {}
}

fn test(a: &mut Example, b: &Example) {
    // - ✅変数に参照を代入 → その変数が最後に使われるところまで借用される
    // - ✅関数の引数にimmutable借用で渡す（戻り値なし） → 関数スコープを抜ければ借用終了
    // - ✅関数の引数にimmutable借用で渡す（戻り値あり）→ 戻り値が最後に使われるところまで借用される
    // - ✅メンバー変数を借用 → その親も借用される
    // - 特定のオブジェクトにimmutable借用 → そのオブジェクトが生きている限り借用される？

    let value = b.get(&a.value);
    a.change();
    a.unchange();
}

fn change(x: &mut i32) {}

fn unchange(x: &i32) {}

fn get(x: &i32) -> &i32 {
    x
}
