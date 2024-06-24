FRAG_SHADER_SRC := $(wildcard ${ASSETS_DIR}/shaders/*.frag)
FRAG_SHADERS := $(FRAG_SHADER_SRC:${ASSETS_DIR}/shaders/%.frag=${BUILD}/shaders/%/frag.spv)

VERT_SHADER_SRC := $(wildcard ${ASSETS_DIR}/shaders/*.vert)
VERT_SHADERS := $(VERT_SHADER_SRC:${ASSETS_DIR}/shaders/%.vert=${BUILD}/shaders/%/vert.spv)

SHADERS := ${SHADER_DIRS} ${FRAG_SHADERS} ${VERT_SHADERS}

.PHONY: shaders
shaders: ${SHADERS}

${FRAG_SHADERS}: ${BUILD}/shaders/%/frag.spv: ${ASSETS_DIR}/shaders/%.frag
	@mkdir -p $(shell dirname $@)
	glslc $< -o $@

${VERT_SHADERS}: ${BUILD}/shaders/%/vert.spv: ${ASSETS_DIR}/shaders/%.vert
	@mkdir -p $(shell dirname $@)
	glslc $< -o $@

