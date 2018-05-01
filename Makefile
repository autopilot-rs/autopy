.PHONY: help
help: ## Print help.
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?## / {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}' \
	$(MAKEFILE_LIST) | sort

.PHONY: mac
mac: ## Build wheel distributions for macOS.
	scripts/mac

.PHONY: linux
linux: ## Build wheel distributions for Linux.
	docker run --rm -v `pwd`:/io quay.io/pypa/manylinux1_x86_64 /io/scripts/build-wheels.sh

.PHONY: upload
upload: ## Upload binary wheel distributions.
	scripts/upload
