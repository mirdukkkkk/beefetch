package ui

import (
	"regexp"
	"strings"
)

var ansiRegex = regexp.MustCompile(`\x1b\[[0-9;]*m`)

func stripANSI(s string) string {
	return ansiRegex.ReplaceAllString(s, "")
}

func padRight(s string, width int) string {
	visible := len(stripANSI(s))
	
	if visible >= width {
		return s
	}

	return s + strings.Repeat(" ", width-visible)
}