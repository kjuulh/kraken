package commands

import (
	"git.front.kjuulh.io/kjuulh/octopush/cmd/octopush/commands/server"
	"github.com/spf13/cobra"
	"go.uber.org/zap"
)

func CreateOctopushCmd(logger *zap.Logger) *cobra.Command {
	cmd := &cobra.Command{
		Use: "octopush",
	}

	cmd.AddCommand(CreateOctopushProcessCmd(logger))
	cmd.AddCommand(server.CreateOctopushServerCmd(logger))

	return cmd
}
