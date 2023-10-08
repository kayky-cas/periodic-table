use std::io::{self, stdin};

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

fn substitute_accents(letter: char) -> char {
    match letter {
        'á' => 'a',
        'à' => 'a',
        'ã' => 'a',
        'â' => 'a',
        'é' => 'e',
        'ê' => 'e',
        'í' => 'i',
        'ó' => 'o',
        'ô' => 'o',
        'õ' => 'o',
        'ú' => 'u',
        'ü' => 'u',
        'ç' => 'c',
        _ => letter,
    }
}

fn message_to_periodic(message: &str) -> Option<String> {
    if message.is_empty() {
        return Some(String::new());
    }

    if let Some(val) = PTABLE.iter().find(|x| message[0..1].eq(&x.to_lowercase())) {
        let rest = message_to_periodic(&message[1..]);

        if let Some(rest) = rest {
            let mut result = val.to_string();
            result.push_str(&rest);

            return Some(result);
        }
    }

    if message.len() < 2 {
        return None;
    }

    if let Some(val) = PTABLE.iter().find(|x| message[0..2].eq(&x.to_lowercase())) {
        let rest = message_to_periodic(&message[2..]);

        if let Some(rest) = rest {
            let mut result = val.to_string();
            result.push_str(&rest);

            return Some(result);
        }
    }

    None
}

fn main() -> io::Result<()> {
    let mut message = String::new();
    stdin().read_line(&mut message)?;

    let message: String = message
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphabetic())
        .map(substitute_accents)
        .collect();

    let result = message_to_periodic(&message);

    match result {
        Some(result) => println!("{}", result),
        None => println!("Combinação não encontrada"),
    };

    Ok(())
}
