package main

import "github.com/bitfield/script"

func main() {

	releaseRc := `
  branches: 
  - "main"
  - "v0.x"

  plugins:
  - "@semantic-release/commit-analyzer"
  - "@semantic-release/release-notes-generator"
  - "@semantic-release/changelog"
  - "@semantic-release/git"
  `

	_, err := script.
		Echo(releaseRc).
		WriteFile(".releaserc.yml")
	if err != nil {
		panic(err)
	}
}
