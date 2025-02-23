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

.PHONY: clean
clean:
	./scripts/clean.sh

.PHONY: test
test:
	./scripts/test.sh
.PHONY: golden-test
golden-test:
	GOLDEN_OVERWRITE=true ./scripts/test.sh
