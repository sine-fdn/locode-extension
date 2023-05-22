all: specs/index.html

specs/index.html: specs/tech_spec.bs
	bikeshed spec $< $@
