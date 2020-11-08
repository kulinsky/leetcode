pub fn reverse_words(s: String) -> String {
    let mut v: Vec<&str> = s
    .split(' ')
    .filter(|x| x != &"")
    .collect();
    v.reverse();
    v.join(" ")
}
