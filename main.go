package main

import (
	"fmt"
	"strings"
)

var ptable = []string{
    "H", "He", "Li", "Be", "B", "C", "N", "O", "F", "Ne", "Na", "Mg", "Al", "Si", "P", "S", "Cl",
    "Ar", "K", "Ca", "Sc", "Ti", "V", "Cr", "Mn", "Fe", "Co", "Ni", "Cu", "Zn", "Ga", "Ge", "As",
    "Se", "Br", "Kr", "Rb", "Sr", "Y", "Zr", "Nb", "Mo", "Tc", "Ru", "Rh", "Pd", "Ag", "Cd", "In",
    "Sn", "Sb", "Te", "I", "Xe", "Cs", "Ba", "La", "Ce", "Pr", "Nd", "Pm", "Sm", "Eu", "Gd", "Tb",
    "Dy", "Ho", "Er", "Tm", "Yb", "Lu", "Hf", "Ta", "W", "Re", "Os", "Ir", "Pt", "Au", "Hg", "Tl",
    "Pb", "Bi", "Po", "At", "Rn", "Fr", "Ra", "Ac", "Th", "Pa", "U", "Np", "Pu", "Am", "Cm", "Bk",
    "Cf", "Es", "Fm", "Md", "No", "Lr", "Rf", "Db", "Sg", "Bh", "Hs", "Mt", "Ds", "Rg", "Cn", "Nh",
    "Fl", "Mc", "Lv", "Ts", "Og",
};

func message_to_periodic(message string) (periodic string, passed bool) {
	if message == "" {
		return "", true
	}

	for _, v := range ptable {
		p := strings.ToLower(v)

		if strings.HasPrefix(message, p) {
			rest, passed := message_to_periodic(message[len(p):])

			if passed {
				return v + rest, true
			}
		}
	}

	return "", false
}

func main() {
	message := "naag"

	periodic, passed := message_to_periodic(message)

	if passed {
		fmt.Println("Message:", message)
		fmt.Println("Periodic:", periodic)
	} else {
		fmt.Println("Message:", message)
		fmt.Println("Periodic: No solution")
	}
}
