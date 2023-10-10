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

fn message_to_periodic(message: &str) -> Option<Vec<String>> {
    if message.is_empty() {
        return Some(Vec::new());
    }

    for p in PTABLE.iter() {
        if message.starts_with(&p.to_lowercase()) {
            if let Some(mut result) = message_to_periodic(&message[p.len()..]) {
                result.push(p.to_string());
                return Some(result);
            }
        }
    }

    None
}

fn main() {
    let message: String = std::env::args()
        .nth(1)
        .expect("Nenhuma mensagem foi passada como argumento")
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphabetic())
        .collect();

    let result = message_to_periodic(&message);

    match result {
        Some(result) => {
            let result = result.iter().rev().collect::<Vec<_>>();
            println!("{:?}", result)
        }
        None => println!("Combinação não encontrada"),
    };
}
