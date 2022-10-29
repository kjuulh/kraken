module ci

go 1.19

require github.com/spf13/cobra v1.6.1

require (
	git.front.kjuulh.io/kjuulh/dagger-go v0.0.1 // indirect
	github.com/inconshreveable/mousetrap v1.0.1 // indirect
	github.com/spf13/pflag v1.0.5 // indirect
)

replace github.com/docker/docker => github.com/docker/docker v20.10.3-0.20220414164044-61404de7df1a+incompatible
