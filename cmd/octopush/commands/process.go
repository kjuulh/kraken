package commands

import (
	"context"
	"time"

	"git.front.kjuulh.io/kjuulh/octopush/internal/cli"
	"git.front.kjuulh.io/kjuulh/octopush/internal/commands"
	"github.com/spf13/cobra"
	"go.uber.org/zap"
)

func CreateOctopushProcessCmd(logger *zap.Logger) *cobra.Command {

	var (
		actionsRepo string
		branch      string
		path        string
	)
	cmd := &cobra.Command{
		Use: "process",
		RunE: func(cmd *cobra.Command, args []string) error {
			if err := cmd.ParseFlags(args); err != nil {
				return err
			}

			ctx := cmd.Context()

			deps, cleanupFunc, err := cli.Start(ctx, logger)
			if err != nil {
				return err
			}

			defer func() {
				ctx, _ = context.WithTimeout(ctx, time.Second*5)
				if err := cleanupFunc(ctx); err != nil {
					panic(err)
				}
			}()

			err = commands.
				NewProcessRepos(logger, deps).
				Process(ctx, actionsRepo, branch, path)
			if err != nil {
				return err
			}

			return nil
		},
	}

	pf := cmd.PersistentFlags()

	pf.StringVar(&actionsRepo, "actions-repo", "", "actions repo is the location of your actions, not where to apply the actions themselves, that should be self contained")
	cmd.MarkPersistentFlagRequired("actions-repo")
	pf.StringVar(&branch, "branch", "main", "which branch to look for actions in, will default to main")
	pf.StringVar(&path, "path", "", "the location of the path inside the repository")
	cmd.MarkPersistentFlagRequired("path")

	return cmd
}
