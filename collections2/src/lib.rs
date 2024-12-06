
// const braucht immer eine explizite Typenangabe
const VOWELS : [char;5] = ['A', 'E', 'I', 'O', 'U']; 

pub fn piglify_word(s: &str) -> Option<String> {
    //let vowels: Vec<char> = vec!['A', 'E', 'I', 'O', 'U'];
   
    // if s.len() < 1 {
    //     return None;
    // }

    
    // let chars = s.to_uppercase().chars().collect::<Vec<char>>();

    // according to clippy use next instead of nth(0)
    if let Some(c0) = s.to_uppercase().chars().next() {
        if VOWELS.contains(&c0) {
            let res = format!("{s}-hay");
            Some(res)
        } else {
            let res = format!("{}-{}ay", &s[1..], &s[0..1]);
            Some(res)
        }
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let word = "";
        match piglify_word(&word) {
            Some(_) => assert!(false),
            None => assert!(true),
        }
    }

    #[test]
    fn check_consonant() {
        let word = "schwein";
        if let Some(result) = piglify_word(&word) {
            assert_eq!(result, "chwein-say");
        } else {
            assert!(false);
        }
    }

    #[test]
    fn check_vowel() {
        let word = "Apfel";
        if let Some(result) = piglify_word(&word) {
            assert_eq!(result, "Apfel-hay");
        } else {
            assert!(false);
        }
    }
}
