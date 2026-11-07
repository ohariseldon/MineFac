"""Type-check the native module against an already built, real Horizon API.

No downloads or engine rebuild. Requires matching rustc and dependency metadata
from the launcher build. This checks API compatibility, not linking or gameplay.
"""
import argparse
from pathlib import Path
import shutil
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[1]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--deps", required=True, type=Path)
    parser.add_argument("--rustc", default="rustc")
    args = parser.parse_args()
    deps = args.deps.resolve()
    candidates = list(deps.glob("libhorizon_client-*.rmeta"))
    if not candidates:
        parser.error("--deps must contain the real libhorizon_client-*.rmeta")
    client = max(candidates, key=lambda path: path.stat().st_mtime_ns)
    source = (ROOT / "native/api/src/lib.rs").as_posix()
    build = ROOT / ".build"
    build.mkdir(exist_ok=True)
    with tempfile.TemporaryDirectory(prefix="api-check-", dir=build) as directory:
        wrapper = Path(directory) / "lib.rs"
        # Native integration includes Veio as a module, as does this wrapper.
        wrapper.write_text(f'#[path = r#"{source}"#] mod veio; pub use veio::hooks;\n', encoding="utf-8")
        subprocess.run([
            shutil.which(args.rustc) or args.rustc, "--crate-name", "veio_api_check",
            "--edition=2024", "--crate-type", "lib", "--emit", "metadata",
            "-L", f"dependency={deps}", "--extern", f"horizon_client={client}",
            str(wrapper), "--out-dir", directory,
        ], cwd=ROOT, check=True)
    print(f"Native API types checked against {client.name}; no linking or game launch.")


if __name__ == "__main__":
    main()
