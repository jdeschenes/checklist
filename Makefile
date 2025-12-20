.PHONY: initial-setup
initial-setup:
	./scripts/setup.sh
.PHONY: start-db
start-db:
	./scripts/start_db.sh

.PHONY: stop-db
stop-db:
	./scripts/stop_db.sh

.PHONY: run
run:
	./scripts/run.sh

.PHONY: build
build:
	./scripts/build.sh

.PHONY: deploy
deploy:
	./scripts/deploy.sh

.PHONY: prepare-sqlx
prepare-sqlx:
	./scripts/prepare_sqlx.sh

.PHONY: clean
clean:
	./scripts/clean.sh

.PHONY: test
test:
	./scripts/test.sh
.PHONY: golden-test
golden-test:
	GOLDEN_OVERWRITE=true ./scripts/test.sh

.PHONY: check
check:
	ansible-playbook infra/check.yml -i infra/inventory/hosts.yml

.PHONY: setup
setup:
	ansible-playbook infra/setup.yml -i infra/inventory/hosts.yml
