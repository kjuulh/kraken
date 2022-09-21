# Roadmap

## POC:

- [x] Add cuddle
- [x] Create storage mechanism
- [x] Pull repository into storage
- [x] Create test action to run on repository
- [x] Sign commit using gpg
- [x] Push commits to branch

### Not in scope

- Pooled runners
- CLI with options
- Server app
- Git hosting providers

## Version 0.1

- [x] Setup a way to choose actions and predicates
- [x] Allow instantiation of actions, octopush template repo etc.
- [x] Implement docker action
- [x] Create pr for gitea provider
- [x] Providing query results
- [x] Create CLI to trigger action

## Version 0.2

- [x] Write README
- [x] Fix git issues
- [x] Allow octopush to run directly on the cli

## Version 0.3

- [ ] Make select depend on query
- [ ] Make configurable ssh user
- [ ] Make configurable gpg keyset
- [ ] Make configurable git provider
  - [ ] Add github
- [ ] Create templating function for easily creating new actions
- [ ] Add way to see progress of runners
- [ ] Implement global .octopush store for easy access to settings
- [ ] Move builders to start instead of every building on every action
- [ ] Setup releases on github
- [ ] Setup CI
- [ ] Setup static analysis
- [ ] Setup releases on gitea using drone
- [ ] Figure out a license (probably MIT)

## Version 0.4

- [ ] Create setup version for local actions
- [ ] Create setup version for server actions
- [ ] Create json schema
- [ ] Move roadmap to release / changelog

## Version 0.x

- Think about some sort of isolation
- Run authenticated on servers
- Create queuing system
- Setup pool of runners
