package cli

import (
	"fmt"

	"github.com/mirdukkkkk/beefetch/internal/ui"
)

type option struct {
	flags string
	desc  string
}

func printHelp() {
	opts := []option{
		{"-h, --help", "Show this help message"},
		{"-n, --no-color", "Disable ANSI colors"},
	}

	fmt.Println(
		ui.C(ui.Yellow, "beefetch") +
			" - neofetch clone written in Go with 🐝\n",
	)

	fmt.Println(ui.C(ui.Blue, "Usage:"))
	fmt.Println("  beefetch [options]\n")

	fmt.Println(ui.C(ui.Blue, "Options:"))

	for _, o := range opts {
		fmt.Printf("  %-16s %s\n", o.flags, o.desc)
	}
}