pub trait Substring{
    fn substr(&self, start: usize, end: usize) -> String;
    // fn substr_i(&self, start: isize, end: isize) -> String;

}

impl<'a> Substring for &'a str{
    fn substr(&self, start: usize, end: usize) -> String{
        if start > end || start == end{
            return String::new();
        }
        self.chars().skip(start).take(end-start).collect()
    }
}

impl Substring for String{
    fn substr(&self, start: usize, end: usize) -> String{
        if start > end || start == end{
            return String::new();
        }
        self.chars().skip(start).take(end-start).collect()
    }
    
}