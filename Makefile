.PHONY: test_coverage
	.PHONY: test_coverage

test_coverage:
	cargo tarpaulin --out Xml
	pycobertura show --format html --output coverage.html cobertura.xml
