package commands

import (
	"bytes"
	"encoding/json"
	"net/http"

	"github.com/spf13/cobra"
)

func CreateKrakenProcessCmd() *cobra.Command {

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

			client := http.Client{}

			var buf bytes.Buffer
			err := json.NewEncoder(&buf).
				Encode(struct {
					Repository string `json:"repository"`
					Branch     string `json:"branch"`
					Path       string `json:"path"`
				}{
					Repository: actionsRepo,
					Branch:     branch,
					Path:       path,
				})
			if err != nil {
				panic(err)
			}

			req, err := http.NewRequestWithContext(
				cmd.Context(),
				http.MethodPost,
				"http://localhost:3000/commands/processRepos",
				&buf,
			)
			if err != nil {
				panic(err)
			}

			resp, err := client.Do(req)
			if err != nil {
				panic(err)
			}
			if resp.StatusCode >= 300 {
				panic(resp.Status)
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
