package ui

const (
	Underline = "\033[4m"
	Red       = "\033[1;31m"
	Yellow    = "\033[1;33m"
	Blue      = "\033[1;34m"
	White     = "\033[1;37m"
	Reset     = "\033[0m"
)

var NoColor = false

func C(color, text string) string {
	if NoColor {
		return text
	}
	return color + text + Reset
}