package commands

import (
	"errors"

	"github.com/spf13/cobra"
	"go.uber.org/zap"
)

func NewStartServerCommand(logger *zap.Logger) *cobra.Command {
	cmd := &cobra.Command{
		Use:   "start",
		Short: "Start the kraken server",
		RunE: func(cmd *cobra.Command, args []string) error {

			return errors.New("some error")
		},
	}

	return cmd
}
