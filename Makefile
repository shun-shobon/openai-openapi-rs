.PHONY: generate
generate:
	openapi-generator generate \
	-i schema/openapi.yaml \
	-g rust \
	-t template \
	-o . \
	--skip-validate-spec \
	--additional-properties packageName=openai-openapi,packageVersion=0.1.0
