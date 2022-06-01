use std::mem;
use std::ptr;

fn main() {
    let mut v = vec![1, 2, 3];

    // さまざまな`v`の情報の重要な断片を抜き出します
    let p = v.as_mut_ptr();
    let len = v.len();
    let cap = v.capacity();

    unsafe {
        // `v`をvoidにキャストします: デストラクタは走りません。
        // よって`p`が指す確保されたメモリを完全に管理することになります。
        mem::forget(v);

        // メモリを4, 5, 6で上書きします
        for i in 0..len as isize {
            ptr::write(p.offset(i), 4 + i);
        }

        // 全てを合わせてVecに戻します
        let rebuilt = Vec::from_raw_parts(p, len, cap);
        assert_eq!(rebuilt, [4, 5, 6]);
    }
    println!("Hello, world!");
}
