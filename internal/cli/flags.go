package cli

type Flags struct {
	Help    bool
	NoColor bool
}

func parseFlags(args []string) Flags {
	f := Flags{}

	for _, arg := range args {
		switch arg {
		case "--help", "-h":
			f.Help = true
		case "--no-color", "-n":
			f.NoColor = true
		}
	}

	return f
}