pub mod progress;

#[cfg(test)]
mod test_progress_bar {
    use super::*;

    #[test]
    fn test_increase() {
        let mut progress_bar = progress::progress::ProgressBar::new(100);
        assert_eq!(100, progress_bar.get_full());
        assert_eq!(0, progress_bar.get_cur());

        for i in 1..4 {
            progress_bar.increase();
            assert_eq!(i, progress_bar.get_cur());
        }
    }

    #[test]
    fn test_increase_stopping() {
        let mut progress_bar = progress::progress::ProgressBar::new(1);
        assert_eq!(1, progress_bar.get_full());
        assert_eq!(0, progress_bar.get_cur());

        for _ in 1..4 {
            progress_bar.increase();
            assert_eq!(1, progress_bar.get_cur());
        }
    }

    #[test]
    fn test_to_string_default() {
        let mut progress_bar = progress::progress::ProgressBar::new(100);

        assert_eq!(
            "[ 0% : 0 / 100 ]",
            progress_bar.to_string(),
        );
        
        progress_bar.increase();
        assert_eq!(
            "[ 1% : 1 / 100 ]",
            progress_bar.to_string(),
        );
    }
    
    #[test]
    fn test_to_string_custom() {
        let mut progress_bar = progress::progress::ProgressBar::new(5);
        progress_bar.set_format("%C | %F : %P\\%".to_string());
        
        
        assert_eq!(
            "0 | 5 : 0%",
            progress_bar.to_string()
        );
        
        progress_bar.increase();
        assert_eq!(
            "1 | 5 : 20%",
            progress_bar.to_string()
        );
    
        progress_bar.increase();
        assert_eq!(
            "2 | 5 : 40%",
            progress_bar.to_string()
        );
    
        progress_bar.increase();
        assert_eq!(
            "3 | 5 : 60%",
            progress_bar.to_string()
        );
    
        progress_bar.increase();
        assert_eq!(
            "4 | 5 : 80%",
            progress_bar.to_string()
        );
    
        progress_bar.increase();
        assert_eq!(
            "5 | 5 : 100%",
            progress_bar.to_string()
        );
    }
}
