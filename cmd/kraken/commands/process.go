package commands

import (
	"bytes"
	"encoding/json"
	"net/http"

	"github.com/spf13/cobra"
)

func CreateKrakenProcessCmd() *cobra.Command {
	cmd := &cobra.Command{
		Use: "process",
		Run: func(cmd *cobra.Command, _ []string) {
			client := http.Client{}

			var buf bytes.Buffer
			err := json.NewEncoder(&buf).
				Encode(struct {
					Repository string `json:"repository"`
					Branch     string `json:"branch"`
					Path       string `json:"path"`
				}{
					Repository: "git@git.front.kjuulh.io:kjuulh/kraken.git",
					Branch:     "feature/query-results",
					Path:       "_examples/queries/scrabe_readme/",
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
		},
	}

	return cmd
}
