ROOT_DIR := $(realpath $(dir $(realpath $(lastword $(MAKEFILE_LIST)))))
BUILD := ${ROOT_DIR}/build
ASSET_MANIFEST := ${BUILD}/asset_manifest.xml
ASSETS_DIR := ${ROOT_DIR}/assets
ASSETS := $(shell find ${ASSETS_DIR} -type f)
TARGET := ${ROOT_DIR}/target
ASSET_MANAGER := ${TARGET}/debug/asset-manager

.PHONY: all
all: ${ASSET_MANIFEST}
	cargo run --bin renderer -- --assets-manifest $<

.PHONY: assets
assets: ${ASSET_MANIFEST}

${ASSET_MANIFEST}: ${ASSETS} ${ASSET_MANAGER} ${BUILD}
	${ASSET_MANAGER} -x -a ${ASSETS_DIR} -m $@

${ASSET_MANAGER}: ${BUILD}
	cargo build --bin asset-manager

${BUILD}:
	@mkdir -p $@

#This is mostly here b/c I couldn't find a way to
#easily have the asset-manager depend on w/e source files
#it requires, this will just force a recompile if I'm
#iterating on it
.PHONY: clean_asset_manager
clean_asset_manager:
	@rm -rf ${ASSET_MANAGER}

.PHONY: clean
clean:
	@rm -rf ${BUILD}
	@cargo clean
