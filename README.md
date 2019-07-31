# Progress - a customizable progressbar

This is a customizable progressbar module for Rust.

## Example

```rust
pub mod progress;


fn main() {
    use progress::progress;
    
    // make instance
    let mut progress_bar = progress::ProgressBar::new(2);
    
    // print status
    // [ 0% : 0 / 2 ]
    println!("{}", progress_bar.to_string());
    
    // proceed 1 step
    progress_bar.increase();
    // [ 50% : 1 / 2 ]
    println!("{}", progress_bar.to_string());
    
    // proceed again
    progress_bar.increase();
    // [ 100% : 2 / 2 ]
    println!("{}", progress_bar.to_string());
    
    // increase is no longer available (limit reached)
    progress_bar.increase();  // has no effects
    // [ 100% : 2 / 2 ]
    println!("{}", progress_bar.to_string());
    
    // user-defined formats are available
    // %C : cur, %F : full, %P : percent
    progress_bar.set_format("%C of %F are done (%P\\%)".to_string());
    // 2 of 2 are done (100%)
    println!("{}", progress_bar.to_string());
}

```

## Made by
Tetsuya Hori 2019/07/31
