package commands

import (
	"git.front.kjuulh.io/kjuulh/kraken/internal/server"
	"github.com/spf13/cobra"
	"go.uber.org/zap"
)

func NewStartServerCommand(logger *zap.Logger) *cobra.Command {
	cmd := &cobra.Command{
		Use:   "start",
		Short: "Start the kraken server",
		RunE: func(cmd *cobra.Command, args []string) error {
			return server.Start(logger)
		},
	}

	return cmd
}
