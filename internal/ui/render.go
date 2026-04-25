package ui

import "fmt"

func Show(u, h, osName, kernel, uptime, ram, wm, server string) {
	bee := Bee()
	info := BuildInfo(u, h, osName, kernel, uptime, ram, wm, server)

	maxLines := max(len(bee), len(info))

	const width = 14

	for i := 0; i < maxLines; i++ {
		var left, right string

		if i < len(bee) {
			left = bee[i]
		}

		if i < len(info) {
			right = info[i]
		}

		fmt.Printf("%s  %s\n", padRight(left, width), right)
	}
}