package ui

func Bee() []string {
	return []string{"",
		"     " + C(White, "__"),
		"    " + C(White, "// \\"),
		"    " + C(White, "\\\\_/") + " " + C(Yellow, "/"),
		"  " + C(Yellow, "--(||)(") + C(White, "'") + C(Yellow, ")"),
		"     " + C(Yellow, "'''"),
		"",
	}
}