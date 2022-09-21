package server

import (
	"github.com/spf13/cobra"
	"go.uber.org/zap"
)

func CreateOctopushServerCmd(logger *zap.Logger) *cobra.Command {
	cmd := &cobra.Command{
		Use: "server",
	}

	cmd.AddCommand(CreateOctopushProcessCmd())

	return cmd
}
