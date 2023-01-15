.PHONY: all
all: generate clippy format

.PHONY: generate
generate:
	openapi-generator generate \
	-i schema/openapi.yaml \
	-g rust \
	-t template \
	-o . \
	--skip-validate-spec \
	--additional-properties packageName=openai-openapi,packageVersion=0.1.0

.PHONY: clippy
clippy:
	cargo clippy --fix --allow-dirty
# HACK: You have to run it twice to get it all fixed.
	cargo clippy --fix --allow-dirty

.PHONY: format
format:
	cargo fmt
