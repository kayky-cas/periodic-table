use std::{
    collections::LinkedList,
    env::Args,
    io::{stdin, stdout},
    time::Instant,
};

const PTABLE: [&str; 118] = [
    "H", "He", "Li", "Be", "B", "C", "N", "O", "F", "Ne", "Na", "Mg", "Al", "Si", "P", "S", "Cl",
    "Ar", "K", "Ca", "Sc", "Ti", "V", "Cr", "Mn", "Fe", "Co", "Ni", "Cu", "Zn", "Ga", "Ge", "As",
    "Se", "Br", "Kr", "Rb", "Sr", "Y", "Zr", "Nb", "Mo", "Tc", "Ru", "Rh", "Pd", "Ag", "Cd", "In",
    "Sn", "Sb", "Te", "I", "Xe", "Cs", "Ba", "La", "Ce", "Pr", "Nd", "Pm", "Sm", "Eu", "Gd", "Tb",
    "Dy", "Ho", "Er", "Tm", "Yb", "Lu", "Hf", "Ta", "W", "Re", "Os", "Ir", "Pt", "Au", "Hg", "Tl",
    "Pb", "Bi", "Po", "At", "Rn", "Fr", "Ra", "Ac", "Th", "Pa", "U", "Np", "Pu", "Am", "Cm", "Bk",
    "Cf", "Es", "Fm", "Md", "No", "Lr", "Rf", "Db", "Sg", "Bh", "Hs", "Mt", "Ds", "Rg", "Cn", "Nh",
    "Fl", "Mc", "Lv", "Ts", "Og",
];

const PTABLE_LOWER: [&str; 118] = [
    "h", "he", "li", "be", "b", "c", "n", "o", "f", "ne", "na", "mg", "al", "si", "p", "s", "cl",
    "ar", "k", "ca", "sc", "ti", "v", "cr", "mn", "fe", "co", "ni", "cu", "zn", "ga", "ge", "as",
    "se", "br", "kr", "rb", "sr", "y", "zr", "nb", "mo", "tc", "ru", "rh", "pd", "ag", "cd", "in",
    "sn", "sb", "te", "i", "xe", "cs", "ba", "la", "ce", "pr", "nd", "pm", "sm", "eu", "gd", "tb",
    "dy", "ho", "er", "tm", "yb", "lu", "hf", "ta", "w", "re", "os", "ir", "pt", "au", "hg", "tl",
    "pb", "bi", "po", "at", "rn", "fr", "ra", "ac", "th", "pa", "u", "np", "pu", "am", "cm", "bk",
    "cf", "es", "fm", "md", "no", "lr", "rf", "db", "sg", "bh", "hs", "mt", "ds", "rg", "cn", "nh",
    "fl", "mc", "lv", "ts", "og",
];

fn message_to_periodic(message: &str) -> Option<LinkedList<&'static str>> {
    if message.is_empty() {
        return Some(LinkedList::new());
    }

    for (p, pl) in PTABLE.iter().zip(PTABLE_LOWER.iter()) {
        if message.starts_with(pl) {
            if let Some(mut result) = message_to_periodic(&message[p.len()..]) {
                result.push_front(p);
                return Some(result);
            }
        }
    }

    return None;
}

fn remove_accent(ch: char) -> char {
    match ch {
        'á' => 'a',
        'é' => 'e',
        'í' => 'i',
        'ó' => 'o',
        'ú' => 'u',
        'ã' => 'a',
        'õ' => 'o',
        'à' => 'a',
        'è' => 'e',
        'ì' => 'i',
        'ò' => 'o',
        'ù' => 'u',
        'â' => 'a',
        'ê' => 'e',
        'î' => 'i',
        'ô' => 'o',
        'û' => 'u',
        _ => ch,
    }
}

fn trab1(args: Args) {
    let message: String = std::env::args()
        .nth(1)
        .expect("Nenhuma mensagem foi passada como argumento")
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphabetic())
        .map(remove_accent)
        .collect();

    let instant = Instant::now();

    match message_to_periodic(&message) {
        Some(result) => println!("{:?} em {:?}", result, instant.elapsed()),
        None => println!("Combinação não encontrada"),
    };
}

fn main() {
    stdin()
        .lines()
        .flatten()
        .map(|l| {
            l.to_lowercase()
                .chars()
                .filter(|c| c.is_alphabetic())
                .map(remove_accent)
                .collect::<String>()
        })
        .map(|message| {
            let periodic = message_to_periodic(&message)?;
            Some((message, periodic))
        })
        .flatten()
        .for_each(|(message, periodic)| println!("{} = {:?}", message, periodic));
}
