.DEFAULT_GOAL:=help

##@ Testing

test: ## Run unit test suite
	cargo test --no-fail-fast --verbose --bin wapc -- --nocapture

test-integration: ##Run integration test suite
	cargo test --no-fail-fast --verbose --test "integration*" -- --nocapture

test-all: test test-integration ## Run all tests

##@ Helpers

help:  ## Display this help
	@awk 'BEGIN {FS = ":.*##"; printf "\nUsage:\n  make \033[36m<target>\033[0m\n"} /^[a-zA-Z_\-.*]+:.*?##/ { printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2 } /^##@/ { printf "\n\033[1m%s\033[0m\n", substr($$0, 5) } ' $(MAKEFILE_LIST)
