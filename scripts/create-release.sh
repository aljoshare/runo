#! sh

cargo release -c .cargo/release.toml --execute $(convco version --bump)