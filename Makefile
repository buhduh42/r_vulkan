ROOT_DIR := $(realpath $(dir $(realpath $(lastword $(MAKEFILE_LIST)))))
BUILD := ${ROOT_DIR}/build
ASSET_MANIFEST := ${BUILD}/asset_manifest.xml
ASSETS_DIR := ${ROOT_DIR}/assets
ASSETS := $(shell find ${ASSETS_DIR} -type f)
TARGET := ${ROOT_DIR}/target
ASSET_MANAGER := ${TARGET}/debug/asset-manager

.PHONY: assets
assets: ${ASSET_MANIFEST}

${ASSET_MANIFEST}: ${ASSETS} ${ASSET_MANAGER} ${BUILD}
	${ASSET_MANAGER} -x -a ${ASSETS_DIR} -m $@

${ASSET_MANAGER}:
	cargo build --bin asset-manager

${BUILD}:
	@mkdir -p $@

.PHONY: clean
clean:
	@rm -rf ${BUILD}
	@cargo clean
