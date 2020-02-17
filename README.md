# learning-rust

- https://doc.rust-jp.rs/book/second-edition/
- Cargo
  - https://doc.rust-lang.org/cargo/
- from here
  - https://doc.rust-jp.rs/book/second-edition/ch10-00-generics.html

```rust
fn largest<T>(list: &[T]) -> T {
```

- この定義は以下のように解読します
  - 関数largestは、なんらかの型Tに関してジェネリックであると
  - この関数にはlistという引数が1つあり、これは型Tの値のスライスです
  - largest関数は同じT型の値を返します
