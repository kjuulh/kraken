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
- [x] Allow instantiation of actions, kraken template repo etc.
- [x] Implement docker action
- [x] Create pr for gitea provider
- [x] Providing query results
- [ ] Create CLI to trigger action

### Not in scope

## Version 1.0

- [ ] Write README
- [ ] Make configurable ssh user
- [ ] Make configurable gpg keyset
- [ ] Make configurable git provider
- [ ] Create templating function
- [ ] Add way to see progress of runners
- [ ] Implement global .kraken store for easy access
- [ ] Move builders to start instead of every time

## Version 1.x

- Think about some sort of isolation
- Run authenticated on servers
- Create queuing system
- Setup pool of runners
