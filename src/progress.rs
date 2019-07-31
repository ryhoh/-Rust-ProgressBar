///
/// 2019/07/31 written by Tetsuya Hori
///
/// Progress - A customizable progressbar
///
///
///
/// # Example
///
/// make instance
/// ```
/// let mut progress_bar = progress::ProgressBar::new(2);
/// ```
///
/// print status -> [ 0% : 0 / 2 ]
/// ```
/// println!("{}", progress_bar.to_string());
/// ```
///
/// proceed 1 step
/// ```
/// progress_bar.increase();
/// ```
///
/// print status -> [ 50% : 1 / 2 ]
/// ```
/// println!("{}", progress_bar.to_string());
/// ```
///
/// proceed again
/// ```
/// progress_bar.increase();
/// ```
///
/// print status -> [ 100% : 2 / 2 ]
/// ```
/// println!("{}", progress_bar.to_string());
/// ```
///
/// increase is no longer available (limit reached)
/// ```
/// progress_bar.increase();  // has no effects
/// ```
///
/// print status -> [ 100% : 2 / 2 ]
/// ```
/// println!("{}", progress_bar.to_string());
/// ```
///
/// user-defined formats are available
/// (%C : cur, %F : full, %P : percent)
/// to use '%', escaping '\' is needed (and one more '\' needed to escape first '\').
/// ```
/// progress_bar.set_format("%C of %F are done (%P\\%)".to_string());
/// ```
///
/// print status -> 2 of 2 are done (100%)
/// ```
/// println!("{}", progress_bar.to_string());
/// ```
///

pub mod progress {

    #[derive(Debug)]
    pub struct ProgressBar {
        full: u32,
        cur: u32,
        format: String,
    }

    impl ProgressBar {
        pub fn new(full: u32) -> ProgressBar {
            ProgressBar {
                full,
                cur: 0,
                format: "[ %P\\% : %C / %F ]".to_string(),
                //    as [ %P\% : %C / %F ]
            }
        }
    
        pub fn set_format(&mut self, format: String) -> &ProgressBar {
            self.format = format;
            self
        }
    
        pub fn get_full(&self) -> u32 {
            self.full
        }
    
        pub fn get_cur(&self) -> u32 {
            self.cur
        }
    
        pub fn increase(&mut self) -> &ProgressBar {
            self.cur = std::cmp::min(self.full, self.cur + 1);
            self
        }
    
        pub fn to_string(&self) -> String {
            let mut res = "".to_string();
            let mut escaped = false;
            let mut magic_chr = false;
        
            for chr in self.format.as_str().chars() {
                if magic_chr {
                    magic_chr = false;
                    res += match chr {
                        'P' => (self.cur as f64 / self.full as f64 * 100.0).to_string(),
                        'C' => self.cur.to_string(),
                        'F' => self.full.to_string(),
                        _ => format!("{}{}", "%", chr),
                    }.as_str()
                } else if escaped {
                    escaped = false;
                    res += &chr.to_string()
                } else {
                    if chr == '\\' {
                        escaped = true;
                    } else if chr == '%' {
                        magic_chr = true;
                    } else {
                        res += &chr.to_string()
                    }
                }
            }
        
            if magic_chr {
                res += "%";
            }
        
            if escaped {
                res += "\\";
            }
        
            res
        }
    }
}
