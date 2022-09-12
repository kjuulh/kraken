package commands

import "github.com/spf13/cobra"

func CreateKrakenCmd() *cobra.Command {
	cmd := &cobra.Command{
		Use: "kraken",
		// Run: func(cmd *cobra.Command, args []string) { },
	}

	cmd.AddCommand(CreateKrakenProcessCmd())

	return cmd
}
