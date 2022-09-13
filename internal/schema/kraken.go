package schema

import "gopkg.in/yaml.v3"

type KrakenSchema struct {
	ApiVersion string `yaml:"apiVersion"`
	Name       string `yaml:"name"`
	Select     struct {
		Repositories []string `yaml:"repositories"`
		Providers    []struct {
			Gitea        string `yaml:"gitea"`
			Organisation string `yaml:"organisation"`
		} `yaml:"providers"`
	} `yaml:"select"`
	Actions []string `yaml:"actions"`
}

func Unmarshal(raw string) (*KrakenSchema, error) {
	k := &KrakenSchema{}
	err := yaml.Unmarshal([]byte(raw), k)
	if err != nil {
		return nil, err
	}
	return k, nil
}
