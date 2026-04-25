package cli

import (
	"fmt"
	"os"

	"github.com/mirdukkkkk/beefetch/internal/fetch"
	"github.com/mirdukkkkk/beefetch/internal/ui"
)

func Run() {
	flags := parseFlags(os.Args[1:])

	if flags.Help {
		printHelp()
		return
	}

	ui.NoColor = flags.NoColor

	data := fetch.GetFullInfo()

	ramLine := fmt.Sprintf("%s MB / %s MB", data.RAMUsed, data.RAMTotal)

	ui.Show(
		data.User,
		data.Host,
		data.OS,
		data.Kernel,
		data.Uptime,
		ramLine,
		data.WM,
		data.Server,
	)
}