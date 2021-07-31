fn main() {
    println!("{}",solution("hosseinShahrokhi"));
}
fn solution(s: &str)->String{
    let mut res = String::new();
    for c in s.chars(){
        if c.is_uppercase(){
            res.push(' ');
        }
        res.push(c);
    }
    res

}
