package commands

import (
	"github.com/spf13/cobra"
	"go.uber.org/zap"
)

func CreateServerCmd(logger *zap.Logger) *cobra.Command {
	cmd := &cobra.Command{
		Use: "octopushserver",
	}

	cmd.AddCommand(NewStartServerCommand(logger))

	return cmd
}
