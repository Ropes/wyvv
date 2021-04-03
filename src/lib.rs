#![warn(dead_code)]
use phf::phf_map;

static ALPHABET: phf::Map<char, &'static str> = phf_map! {
    'a' => "alpha",
    'b'=> "bravo",
    'c'=> "charlie",
    'd'=> "delta",
    'e'=> "echo",
    'f'=> "foxtrot",
    'g'=> "golf",
    'h'=> "hotel",
    'i'=> "india",
    'j'=> "juliet",
    'k'=> "kilo",
    'l'=> "lima",
    'm'=> "mike",
    'n'=> "november",
    'o'=> "oscar",
    'p'=> "papa" ,
    'q'=> "quebec",
    'r'=> "romeo",
    's'=> "sierra",
    't'=> "tango",
    'u'=> "uniform",
    'v'=> "victor",
    'w'=> "whiskey",
    'x'=> "xray",
    'y'=> "yankee",
    'z'=> "zulu",
};

fn nato_alpha(c: char) -> &'static str {
    let result = ALPHABET[&c];
    return result;
}

/*
static DIGITS: phf::Map<u32, &'static str> = phf_map! {
    0 => "zero",
    1 => "one",
    2 => "two",
};

lazy_static!{
    static ref MAP: HashMap<char, &'static str> = [
    ("a", "alpha"),
    ("b", "bravo"),
    ("c", "charlie"),
    ("d", "delta"),
    ("e", "echo"),
    ("f", "foxtrot"),
    ("g", "golf"),
    ("h", "hotel"),
    ("i", "india"),
    ("j", "juliet"),
    ("k", "kilo"),
    ("l", "lima"),
    ("m", "mike"),
    ("n", "november"),
    ("o", "oscar"),
    ("p","papa" ),
    ("q", "quebec"),
    ("r", "romeo"),
    ("s", "sierra"),
    ("t", "tango"),
    ("u", "uniform"),
    ("v", "victor"),
    ("w", "whiskey"),
    ("x", "xray"),
    ("y", "yankee"),
    ("z", "zulu"),
    ].iter().copied().collect();
}
*/

#[cfg(test)]
mod tests {
    #[test]
    fn lookup_alpha() {
        use crate::nato_alpha;
        let a = 'a';
        assert_eq!(nato_alpha(a), "alpha");
    }

    #[test]
    fn lookup_delta(){
        use crate::nato_alpha;
        let a = 'd';
        assert_eq!(nato_alpha(a), "delta");
    }
}
