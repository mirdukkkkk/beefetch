package fetch

import (
	"bufio"
	"fmt"
	"os"
	"strings"
	"time"
)

type SystemInfo struct {
	User     string
	Host     string
	OS       string
	Kernel   string
	Uptime   string
	RAMUsed  string
	RAMTotal string
	WM       string
	Server   string
}

func GetFullInfo() *SystemInfo {
	ramUsed, ramTotal := getRAM()
	
	server := os.Getenv("XDG_SESSION_TYPE")
	if server == "" {
		server = "tty"
	}

	return &SystemInfo{
		User:     os.Getenv("USER"),
		Host:     getHost(),
		OS:       getOS(),
		Kernel:   getKernel(),
		Uptime:   getUptime(),
		RAMUsed:  ramUsed,
		RAMTotal: ramTotal,
		WM:       getWM(),
		Server:   server,
	}
}

func getKernel() string {
	data, _ := os.ReadFile("/proc/sys/kernel/osrelease")
	return strings.TrimSpace(string(data))
}

func getWM() string {
	wm := os.Getenv("XDG_CURRENT_DESKTOP")
	
	if wm == "" {
		wm = os.Getenv("DESKTOP_SESSION")
	}
	
	if wm == "" {
		if os.Getenv("WAYLAND_DISPLAY") != "" {
			wm = "Wayland"
		} else if os.Getenv("DISPLAY") != "" {
			wm = "X11"
		} else {
			wm = "N/A"
		}
	}
	return wm
}

func getOS() string {
	file, _ := os.Open("/etc/os-release")
	defer file.Close()

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		if strings.HasPrefix(scanner.Text(), "PRETTY_NAME=") {
			return strings.Trim(strings.Split(scanner.Text(), "=")[1], "\"")
		}
	}

	return "Linux"
}

func getUptime() string {
	data, _ := os.ReadFile("/proc/uptime")

	var seconds float64
	fmt.Sscanf(string(data), "%f", &seconds)
	d := time.Duration(seconds) * time.Second

	return fmt.Sprintf("%dh %dm", int(d.Hours()), int(d.Minutes())%60)
}

func getRAM() (string, string) {
	file, _ := os.Open("/proc/meminfo")
	defer file.Close()

	var total, active int64

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()
		if strings.HasPrefix(line, "MemTotal:") {
			fmt.Sscanf(line, "MemTotal: %d", &total)
		}
		if strings.HasPrefix(line, "Active:") {
			fmt.Sscanf(line, "Active: %d", &active)
		}
	}

	return fmt.Sprintf("%dMB", active/1024), fmt.Sprintf("%dMB", total/1024)
}

func getHost() string {
	host, _ := os.Hostname()
	return host
}