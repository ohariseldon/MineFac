"""Real rendered factory benchmark. Uses a disposable world, never a player's instance.

Requires psutil for host/process memory samples. Run an optimized native Horizon
containing this Veio, not the vanilla executable or a dev build. Raw results and
the source/binary fingerprints are retained for comparison.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import math
import os
from pathlib import Path
import re
import shutil
import socket
import statistics
import subprocess
import threading
import time

import psutil

ROOT = Path(__file__).resolve().parents[1]


def save(path, value):
    path.write_text(json.dumps(value, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")


class Control:
    """The documented local TCP transport, authenticated without logging its token."""
    def __init__(self, endpoint, timeout=60):
        address = json.loads(endpoint.read_text(encoding="utf-8"))
        self.socket = socket.create_connection(("127.0.0.1", address["port"]), timeout=timeout)
        self.socket.setsockopt(socket.IPPROTO_TCP, socket.TCP_NODELAY, 1)
        self.stream = self.socket.makefile("rwb")
        self.call("auth " + address["token"])

    def call(self, line):
        self.stream.write((line + "\n").encode("utf-8"))
        self.stream.flush()
        raw = self.stream.readline()
        if not raw:
            raise RuntimeError("Game disconnected while running " + line.split(" ", 1)[0])
        reply = json.loads(raw)
        if "error" in reply:
            raise RuntimeError(f"{line}: {reply['error']}")
        return reply

    def command(self, command):
        return self.call("command " + command)

    def close(self):
        self.stream.close()
        self.socket.close()


def isolated_environment(folder):
    env = os.environ.copy()
    for key in ("HORIZON_SHADER_PACK", "HORIZON_STILL", "HORIZON_BENCHMARK", "HORIZON_SERVER_THREAD"):
        env.pop(key, None)
    for key in ("HORIZON_HOME", "HORIZON_LAUNCHER_HOME", "XDG_DATA_HOME", "XDG_CONFIG_HOME",
                "HORIZON_LAUNCHER_TRASH", "HORIZON_LAUNCHER_CLIPBOARD", "HORIZON_LAUNCHER_DESKTOP"):
        directory = folder / key.lower()
        directory.mkdir(parents=True, exist_ok=True)
        env[key] = str(directory)
    env.update(HORIZON_NO_AUDIO="1", HORIZON_LAUNCHER_NO_SELF_UPDATE="1")
    return env


class Game:
    def __init__(self, args, directory, shader, scale=85, shadow_resolution=None, shadow_distance=None):
        self.directory = directory
        directory.mkdir(parents=True, exist_ok=True)
        content = args.source / "content"
        if not (directory / "horizon-mods/veio").exists():
            shutil.copytree(content, directory / "horizon-mods/veio")
        (directory / "horizon-mods.txt").write_text("mod.veio:true\n", encoding="utf-8")
        pack = args.source / "visuals/veio-industrial"
        if shader:
            shutil.copytree(pack, directory / "shaderpacks/veio-industrial", dirs_exist_ok=True)
        (directory / "options.txt").write_text(
            f"renderDistance:8\nsimulationDistance:5\nmaxFps:{args.fps_cap}\nenableVsync:false\n"
            'graphicsPreset:fast\nrenderClouds:"fast"\ncloudRange:8\nparticles:1\nao:true\n'
            "mipmapLevels:2\nentityDistanceScaling:0.75\nweatherRadius:3\nlang:en_us\n", encoding="utf-8")
        (directory / "horizon-graphics.txt").write_text(
            "preset:custom\nlodDistance:0\ntextureBudgetMiB:256\nworkers:2\nhdr:false\n"
            f"resolutionScale:{scale}\ntaa:false\nfxaa:false\nupscaling:native\nframeGeneration:off\n"
            "caveCulling:true\nframePacing:efficient\npickedShaderPack:"
            + ("horizon:veio_industrial" if shader else "") + "\n", encoding="utf-8")
        if shadow_resolution or shadow_distance:
            options = directory / "shaderpacks/.horizon-options/veio_industrial.txt"
            options.parent.mkdir(parents=True, exist_ok=True)
            options.write_text((f"shadow_resolution={shadow_resolution}\n" if shadow_resolution else "")
                               + (f"shadow_distance={shadow_distance}\n" if shadow_distance else ""), encoding="utf-8")
        self.endpoint = directory / "control.json"
        self.stdout = (directory / "stdout.log").open("w", encoding="utf-8")
        self.stderr = (directory / "stderr.log").open("w", encoding="utf-8")
        self.process = subprocess.Popen([
            str(args.engine), "--offscreen", "--control-socket", str(self.endpoint),
            "--game-dir", str(directory), "--resources", str(args.resources),
            "--world", "FactoryBench", "--seed", "1234", "--creative", "--render-distance", "8",
            "--width", str(args.width), "--height", str(args.height),
        ], cwd=directory, env=isolated_environment(directory), stdout=self.stdout, stderr=self.stderr,
            creationflags=subprocess.CREATE_NO_WINDOW if os.name == "nt" else 0)
        self.control = None
        self.samples = []
        self.stop_sampling = threading.Event()
        self.sampler = threading.Thread(target=self.sample_memory, daemon=True)
        self.sampler.start()

    def sample_memory(self):
        process = psutil.Process(self.process.pid)
        while not self.stop_sampling.is_set():
            try:
                memory = process.memory_info()
                system = psutil.virtual_memory()
                self.samples.append({"time": time.monotonic(), "rss": memory.rss,
                                     "private": getattr(memory, "private", memory.vms),
                                     "system_available": system.available,
                                     "system_used": system.total - system.available})
            except (psutil.NoSuchProcess, psutil.AccessDenied):
                break
            self.stop_sampling.wait(0.5)

    def wait_ready(self, seconds=240):
        deadline = time.monotonic() + seconds
        last = None
        while time.monotonic() < deadline:
            if self.process.poll() is not None:
                raise RuntimeError("Game exited: " + (self.directory / "stderr.log").read_text(encoding="utf-8")[-4000:])
            try:
                if self.control is None:
                    self.control = Control(self.endpoint)
                state = self.control.call("status")
                if state.get("player") and state.get("screen") in (None, "playing"):
                    return state
                # The old protocol's status has a nested game state; state is authoritative.
                state = self.control.call("state")
                if state.get("screen") == "playing":
                    return state
            except (OSError, ValueError, RuntimeError) as error:
                last = error
            time.sleep(0.5)
        raise RuntimeError(f"Game was not ready: {last}")

    def close(self):
        if self.control:
            try:
                self.control.call("quit")
                self.control.close()
            except (OSError, RuntimeError):
                pass
        try:
            self.process.wait(timeout=25)
        except subprocess.TimeoutExpired:
            self.process.terminate()
            self.process.wait(timeout=10)
        self.stop_sampling.set()
        self.sampler.join(timeout=2)
        self.stdout.close()
        self.stderr.close()
        save(self.directory / "process-memory.json", self.samples)


def layout(lanes):
    return [(8 * (i % 16), 200, 4 * (i // 16)) for i in range(lanes)]


def build_factory(game, lanes):
    ctl = game.control
    # Ordinary terrain remains loaded. The platform avoids destroying natural terrain and
    # uses the same visible scene for every run. Force-load is a fixture, never a player setting.
    ctl.command("forceload add -16 -32 143 95")
    ctl.command("tp @s 64 205 24")
    deadline = time.monotonic() + 90
    while time.monotonic() < deadline:
        if all(ctl.call(f"block {x} 199 {z}").get("loaded") for x, z in ((0, 0), (127, 0), (0, 63), (127, 63))):
            break
        time.sleep(0.5)
    else:
        raise RuntimeError("The whole factory footprint must be loaded before placement")
    ctl.command("fill -2 199 -2 129 199 65 minecraft:smooth_stone")
    ctl.command("weather clear 1000000")
    ctl.command("time set 4000")
    positions = layout(lanes)
    for index, (x, y, z) in enumerate(positions):
        ctl.command(f"setblock {x} {y-1} {z} minecraft:copper_ore")
        for dx, block in [(0, "veio:drill"), (1, "veio:belt"), (2, "veio:press"),
                          (3, "veio:belt"), (4, "veio:assembler"), (5, "veio:belt")]:
            ctl.command(f"setblock {x+dx} {y} {z} {block}[facing=west]")
        ctl.command(f"setblock {x+6} {y} {z} minecraft:chest[facing=north]")
        ctl.command(f"fill {x} {y} {z+1} {x+4} {y} {z+1} veio:cable")
        # Real fuel, then real energy distribution: no infinite energy NBT or simulated production.
        ctl.command(f'setblock {x} {y} {z+2} veio:generator[facing=south]{{Items:[{{Slot:0b,id:"minecraft:coal",count:64}}]}}')
        if index == 0:
            print("Checking fuel, extraction, power, processing and the first gear before scaling up", flush=True)
            for attempt in range(3):
                ctl.call("fly 64 221 -12 64 221 -11.999 12 35")
                proof = production_evidence(ctl, positions[:1])
                counts = chest_counts(ctl, positions[:1])
                save(game.directory / f"first-line-{attempt}.json", {"proof": proof, "chests": counts})
                if counts[0]["items"] > 0:
                    break
            else:
                raise RuntimeError("The first line did not produce a gear; see first-line evidence before benchmarking")
        if (index + 1) % 32 == 0:
            print(f"Factory: {index+1}/{lanes} lines placed", flush=True)
    ctl.call("save-all")
    save(game.directory / "factory-layout.json", {"lanes": lanes, "positions": positions,
        "processing_machines": lanes * 3, "generators": lanes, "belts": lanes * 3,
        "cables": lanes * 5, "chests": lanes, "recipe": "ore -> copper plate -> gear -> chest"})
    return positions


def production_evidence(ctl, positions):
    evidence = []
    for x, y, z in positions:
        evidence.append({"position": [x, y, z],
            "drill": ctl.command(f"data get block {x} {y} {z}"),
            "press": ctl.command(f"data get block {x+2} {y} {z}"),
            "assembler": ctl.command(f"data get block {x+4} {y} {z}"),
            "generator": ctl.command(f"data get block {x} {y} {z+2}"),
            "chest": ctl.command(f"data get block {x+6} {y} {z}")})
    return evidence


def chest_counts(ctl, positions):
    counts = []
    for x, y, z in positions:
        reply = ctl.command(f"data get block {x+6} {y} {z}")
        text = "\n".join(reply.get("chat", []))
        # Data is read from an ordinary output chest, never the visual display NBT.
        counts.append({"position": [x+6, y, z], "items": sum(map(int, re.findall(r"(?:count|Count):\s*(\d+)", text))), "reply": reply})
    return counts


def percentile(values, q):
    ordered = sorted(values)
    at = (len(ordered) - 1) * q
    a, b = math.floor(at), math.ceil(at)
    return ordered[a] + (ordered[b] - ordered[a]) * (at - a)


def summarize(frames, memory):
    values = [float(x) for x in frames["frame_ms"] if float(x) > 0]
    p99 = percentile(values, 0.99)
    return {"frames": len(values), "fps": 1000 / statistics.mean(values),
        "p50_ms": percentile(values, 0.5), "p95_ms": percentile(values, 0.95), "p99_ms": p99,
        "p99_fps": 1000 / p99, "worst_ms": max(values),
        "over_50_ms": sum(x > 50 for x in values), "over_100_ms": sum(x > 100 for x in values),
        "rss_peak_mib": max(x["rss"] for x in memory) / 2**20,
        "private_peak_mib": max(x["private"] for x in memory) / 2**20,
        "system_available_min_mib": min(x["system_available"] for x in memory) / 2**20,
        "server_tick_mean_ms": frames.get("tick_ms"), "server_tick_smoothed_max_ms": frames.get("tick_ms_max")}


def measure(args, game, label, run):
    ctl = game.control
    ctl.command("time set 4000")
    ctl.call(f"fly 64 221 -12 64 221 64 {args.warmup} 35")
    # Cover every row and column diagonally, plus count the real outputs of ALL lines.
    proof_positions = layout(args.lanes)
    selected = [proof_positions[round(i * (len(proof_positions)-1)/15)] for i in range(16)]
    before = production_evidence(ctl, selected)
    chests_before = chest_counts(ctl, proof_positions)
    ctl.command("time set 4000")
    started = time.monotonic()
    frames = ctl.call(f"fly 64 221 -12 64 221 64 {args.seconds} 35 raw")
    ended = time.monotonic()
    memory = [entry for entry in game.samples if started <= entry["time"] <= ended]
    after = production_evidence(ctl, selected)
    chests_after = chest_counts(ctl, proof_positions)
    growing = sum(b["items"] > a["items"] for a, b in zip(chests_before, chests_after))
    result = {"case": label, "run": run, "summary": summarize(frames, memory), "raw": frames,
              "memory_samples": memory, "production_before": before, "production_after": after,
              "chests_before": chests_before, "chests_after": chests_after, "productive_lines": growing,
              "memory_report": ctl.call("memory"), "status": ctl.call("status")}
    save(args.output / f"{label}-{run}.json", result)
    ctl.call("tp 64 221 -12")
    ctl.call("look 0 35")
    ctl.call("screenshot " + str(args.output / f"{label}-{run}.png"))
    if growing != args.lanes:
        raise RuntimeError(f"Invalid benchmark: only {growing}/{args.lanes} production lines increased their output")
    print(json.dumps({"case": label, "run": run, **result["summary"]}), flush=True)
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--engine", type=Path, required=True)
    parser.add_argument("--resources", type=Path, required=True)
    parser.add_argument("--source", type=Path, default=ROOT)
    parser.add_argument("--output", type=Path, default=ROOT / ".build/large-factory-bench")
    parser.add_argument("--lanes", type=int, default=256)
    parser.add_argument("--seconds", type=float, default=30)
    parser.add_argument("--warmup", type=float, default=15)
    parser.add_argument("--runs", type=int, default=3)
    parser.add_argument("--width", type=int, default=1920)
    parser.add_argument("--height", type=int, default=1080)
    parser.add_argument("--fps-cap", type=int, default=60)
    parser.add_argument("--cases", nargs="+", default=["off", "on"])
    parser.add_argument("--build-only", action="store_true")
    args = parser.parse_args()
    for name in ("engine", "resources", "source", "output"):
        setattr(args, name, getattr(args, name).resolve())
    args.output.mkdir(parents=True, exist_ok=True)
    if not 1 <= args.lanes <= 256:
        parser.error("lanes must be between 1 and 256 for the fixed footprint")
    metadata = {"created": time.strftime("%Y-%m-%dT%H:%M:%S%z"), "engine": str(args.engine),
        "engine_sha256": hashlib.sha256(args.engine.read_bytes()).hexdigest(), "ram_total_gib": psutil.virtual_memory().total/2**30,
        "logical_cpus": psutil.cpu_count(), "width": args.width, "height": args.height,
        "fps_cap": args.fps_cap, "seconds": args.seconds, "warmup": args.warmup, "lanes": args.lanes,
        "same_scene_comparison": True, "native_profile": "fast/performance; supplied engine must be optimized"}
    save(args.output / "method.json", metadata)
    fixture = args.output / "fixture"
    if not (fixture / "factory-layout.json").exists():
        game = Game(args, fixture, False)
        try:
            game.wait_ready()
            build_factory(game, args.lanes)
        finally:
            game.close()
    if args.build_only:
        return
    results = []
    for run in range(args.runs):
        # Alternate order so one case does not always get the first cold cache.
        cases = args.cases if run % 2 == 0 else list(reversed(args.cases))
        for label in cases:
            directory = args.output / f"run-{run+1}-{label}"
            if not directory.exists():
                shutil.copytree(fixture / "saves", directory / "saves")
            game = Game(args, directory, label != "off")
            try:
                game.wait_ready()
                results.append(measure(args, game, label, run + 1))
            finally:
                game.close()
    save(args.output / "summary.json", {"method": metadata, "runs": [
        {"case": item["case"], "run": item["run"], **item["summary"]} for item in results]})


if __name__ == "__main__":
    main()
