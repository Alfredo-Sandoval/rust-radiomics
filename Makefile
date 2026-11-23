.DEFAULT_GOAL := qa
.PHONY: env check-env lint typecheck test qa ibsi build loc clean

# Python bindings live in crates/radiomics-py; the Rust core is crates/radiomics.
PYDIR ?= crates/radiomics-py
PYTHON ?= $(PYDIR)/.venv/bin/python
TESTS ?= tests
PYTEST_ARGS ?=
CARGO ?= cargo
SOURCE_DIRS ?= $(PYDIR)/python $(PYDIR)/tests
export PYTHONNOUSERSITE := 1

# IBSI and parity inputs are never stored in this repository. Point these variables at local
# copies before make ibsi:
#   IBSI1_CT_PHANTOM      IBSI 1 CT radiomics phantom directory (image/, mask/); CC BY-NC
#   IBSI1_REFERENCE_DIR   IBSI 1 consensus tables ("config A.csv" and others)
#   IBSI2_REFERENCE_DIR   IBSI 2 reference response maps
#   IBSI2_PHANTOM_DIR     IBSI 2 phase 1 digital phantoms
# Optional: W1_IBSI_CT_PHANTOM and W1_IBSI_CT_GOLDENS enable the CT interpolation check
# (goldens come from validation/w1_ct_phantom_goldens.py); IBSI2_MAPS limits the IBSI 2 maps.
IBSI_VARS := IBSI1_CT_PHANTOM IBSI1_REFERENCE_DIR IBSI2_REFERENCE_DIR IBSI2_PHANTOM_DIR

# Creates $(PYDIR)/.venv from the lock and builds the extension into it (editable, release).
env:
	uv sync --project $(PYDIR) --locked --no-install-project
	VIRTUAL_ENV="$(abspath $(PYDIR))/.venv" $(PYDIR)/.venv/bin/maturin develop --release --uv -m $(PYDIR)/Cargo.toml

check-env:
	@command -v "$(PYTHON)" >/dev/null || { echo "Missing project Python: run make env explicitly, or set PYTHON to its existing interpreter" >&2; exit 1; }
	@"$(PYTHON)" -c "import rustrad" 2>/dev/null || { echo "rustrad is not importable in $(PYTHON): run make env" >&2; exit 1; }

lint: check-env
	$(CARGO) fmt --all --check
	$(CARGO) clippy --locked --workspace --all-targets -- -D warnings
	$(PYTHON) -m ruff check $(PYDIR)

# The second step enforces the Any ban on every file and ignores noqa comments.
typecheck: check-env
	@command -v node >/dev/null || { echo "Maintained host Node is required for Pyright" >&2; exit 1; }
	cd $(PYDIR) && PYRIGHT_PYTHON_NODEJS_WHEEL=0 PYRIGHT_PYTHON_GLOBAL_NODE=1 $(abspath $(PYTHON)) -m pyright --project . --pythonpath $(abspath $(PYTHON)) --warnings
	bash -o pipefail -c 'find $(SOURCE_DIRS) -type f \( -name "*.py" -o -name "*.pyi" \) -print0 | xargs -0 $(PYTHON) -m ruff check --config $(PYDIR)/pyproject.toml --select TID251,ANN401 --ignore-noqa --no-force-exclude --'

test: check-env
	$(CARGO) test --locked --release
	cd $(PYDIR) && $(abspath $(PYTHON)) -m pytest $(TESTS) $(PYTEST_ARGS)

qa: lint typecheck

# Opt-in suites that read IBSI reference data from the variables above.
ibsi: check-env
	@missing=""; for v in $(IBSI_VARS); do eval "val=\$$$$v"; [ -n "$$val" ] || missing="$$missing $$v"; done; \
	[ -z "$$missing" ] || { echo "Set:$$missing (see the comments in this Makefile)" >&2; exit 2; }
	W8_COMPARE_PYTHON="$(abspath $(PYTHON))" $(CARGO) test --locked --release -p radiomics \
		--test w12_ibsi1_digital --test w12_ibsi1_ct --test w13_ibsi2_features \
		--test w1_ct_phantom --test w8_compare --test parity_catalog --test parity_families \
		-- --nocapture
	$(CARGO) test --locked --release -p radiomics --lib ibsi2_maps -- --nocapture
	cd $(PYDIR) && $(abspath $(PYTHON)) -m pytest tests/test_parity.py $(PYTEST_ARGS)

# Builds the sdist and one release wheel per version in WHEEL_PYTHONS for the host platform
# into dist/. uv finds each interpreter, so install any that are missing first with
# `uv python install`. Panic messages in the extension name the source file of each
# dependency, so the Cargo home is remapped to /cargo, and the build fails if a wheel still
# contains the home directory.
WHEEL_PYTHONS ?= 3.13 3.14 3.15
CARGO_HOME_DIR := $(or $(CARGO_HOME),$(HOME)/.cargo)
build: check-env
	rm -rf dist
	$(PYDIR)/.venv/bin/maturin sdist -m $(PYDIR)/Cargo.toml -o dist
	@set -e; for v in $(WHEEL_PYTHONS); do \
		interpreter="$$(uv python find "$$v")"; \
		echo "Building the wheel for Python $$v with $$interpreter"; \
		RUSTFLAGS="$$RUSTFLAGS --remap-path-prefix=$(CARGO_HOME_DIR)=/cargo" \
			$(PYDIR)/.venv/bin/maturin build --release -i "$$interpreter" -m $(PYDIR)/Cargo.toml -o dist; \
	done
	@for f in dist/*; do \
		if { case "$$f" in *.whl) unzip -p "$$f";; *) gzip -dc "$$f";; esac; } | LC_ALL=C grep -aqF "$(HOME)"; then \
			echo "$$f contains paths under $(HOME)" >&2; exit 1; \
		fi; \
	done

loc:
	@find crates validation benchmarks -type f \( -name '*.py' -o -name '*.pyi' -o -name '*.rs' \) -not -path '*/.venv/*' -not -path '*/tests/data/*' -print0 | xargs -0 wc -l

clean:
	rm -rf $(PYDIR)/.pytest_cache $(PYDIR)/.ruff_cache .pytest_cache .ruff_cache build dist $(PYDIR)/build $(PYDIR)/dist
	find . \( -name .venv -o -name target -o -name .git \) -prune -o -type d -name __pycache__ -prune -exec rm -rf {} +
