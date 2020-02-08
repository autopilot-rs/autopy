.PHONY: build
build: ## Build debug target for development.
	rustup default nightly-2019-10-05
	pip install -r requirements.txt
	python setup.py build

.PHONY: help
help: ## Print help information.
	@awk 'BEGIN { \
        FS = ":.*?## " \
    } /^[a-zA-Z_-]+:.*?## / { \
        printf "\033[36m%-30s\033[0m %s\n", $$1, $$2 \
    }' $(MAKEFILE_LIST)

.PHONY: install
install: ## Install local target.
	pip install .

.PHONY: mac
mac: ## Build wheel distributions for macOS.
	scripts/mac

.PHONY: linux
linux: ## Build wheel distributions for Linux.
	docker run --rm -v `pwd`:/io quay.io/pypa/manylinux1_x86_64 /io/scripts/build-wheels.sh

.PHONY: upload
upload: ## Upload binary wheel distributions.
	scripts/upload
