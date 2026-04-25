package ui

func BuildInfo(u, h, osName, kernel, uptime, ram, wm, server string) []string {
	return []string{
		C(Underline+Blue, u) + "@" + C(Underline+Blue, h),
		C(Red, "OS:") + " " + osName,
		C(Red, "Kernel:") + " " + kernel,
		C(Red, "Uptime:") + " " + uptime,
		C(Red, "RAM:") + " " + ram,
		C(Red, "WM:") + " " + wm,
		C(Red, "Server:") + " " + server,
	}
}