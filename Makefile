format:
	ormolu --mode inplace $$(git ls-files '*.hs')

.PHONY: format
