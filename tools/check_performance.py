"""Offline, instrumented scale benchmark; reports allocations/work, never game FPS."""
import argparse
import json
from pathlib import Path
import re
import statistics
import subprocess

ROOT = Path(__file__).resolve().parents[1]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--runs", type=int, default=3)
    parser.add_argument("--output", type=Path, default=ROOT / ".build/performance.json")
    args = parser.parse_args()
    if args.runs < 1:
        parser.error("--runs must be positive")
    records, checks = {}, []
    for _ in range(args.runs):
        result = subprocess.run([
            "cargo", "test", "--manifest-path", str(ROOT / "native/offline-check/Cargo.toml"),
            "--release", "--offline", "-p", "veio-check", "profile_scale", "--",
            "--ignored", "--test-threads=1", "--nocapture",
        ], cwd=ROOT, capture_output=True, text=True, encoding="utf-8", errors="replace")
        if result.returncode:
            raise SystemExit(result.stdout + result.stderr)
        batch = [json.loads(raw) for raw in re.findall(r"PERF (\{[^\n]+\})", result.stdout)]
        if len(batch) != 8:
            raise SystemExit("Expected all eight scale scenarios")
        for entry in batch:
            records.setdefault(entry["scenario"], []).append(entry)
        checks.append(re.findall(r"CHECK ([^\n\r]+)", result.stdout))
    if any(check != checks[0] for check in checks):
        raise SystemExit("Conservation/work checks changed between samples")
    report = {
        "revision": subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=ROOT, text=True).strip(),
        "scope": "offline instrumented simulation; timings include counters; no renderer or FPS measurement",
        "samples": args.runs,
        "checks": checks[0],
        "scenarios": [{
            "scenario": name,
            **{key: statistics.median(sample[key] for sample in samples)
               for key in ("runs", "ns", "allocations", "allocated_bytes", "nbt_keys", "fallback_locks")},
        } for name, samples in records.items()],
    }
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8")
    print(f"{len(records)} scenarios, {args.runs} samples; checks identical. {args.output}")


if __name__ == "__main__":
    main()
