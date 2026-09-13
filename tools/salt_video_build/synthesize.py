"""Generate lesson narration with VoiceStudio's real KittenTTS backend.

This file does not use a hosted TTS API, clone a voice, or modify course/app data.
It runs only in the dedicated build branch. Output is local WAV plus a manifest.
"""
from __future__ import annotations

import hashlib
import importlib.metadata
import json
import os
from pathlib import Path
import re
import sys
import time

import numpy as np
import soundfile as sf
import torch

VOICE_STUDIO_REF = "eaf8bb953855cab3b687d547b3835f86fa38b308"
VOICE = "expr-voice-2-m"  # Kitten's published Jasper preset, not a cloned voice.
SPEED = 0.94

LESSONS = {
    "000": [
        [
            "Lesson zero zero zero is the map. Salt is not a replacement for Linux, and this course will not teach it as one.",
            "We will move upward from packages, files, services, permissions, and processes, into execution modules, states, highstate, orchestration, events, and finally external workflow ownership.",
            "At every layer, ask what the abstraction hides, what it still depends on, and what evidence it can honestly provide.",
        ],
        [
            "Every lesson will reuse one loop. What is true?",
            "What should be true? What operation could close the gap?",
            "Salt supplies mechanisms for observation and convergence. Policy still decides whether the change is safe, correctly targeted, and worth making.",
        ],
        [
            "A command can execute perfectly and still be the wrong command.",
            "Restarting every node at once might return success from every minion while taking the application offline.",
            "So we separate mechanical correctness from operational correctness. Automation reduces toil only when its policy is defensible.",
        ],
        [
            "Submitted, running, completed, reported success, and independently verified are different states.",
            "A client timeout means the required evidence did not arrive in time. It does not prove the remote work stopped.",
            "A state result says what Salt observed and reported. Your application check asks whether the real objective works.",
        ],
        [
            "The exercises assume one Salt master and two disposable minions: lab RHEL nine and lab Debian, isolated from production.",
            "Repository setup and supported operating-system combinations depend on the exact Salt three thousand six patch and package source you install.",
            "The examples are teaching instruments. They are not permission to replace production configuration or accept every pending key.",
        ],
        [
            "Thought experiment. Salt reports the package installed, the file correct, the service enabled, and the process running.",
            "Yet a real client receives HTTP five oh three. Which layer succeeded? Which claim was never tested?",
            "What evidence would you add before calling the change complete? Pause here and decide what you would check next.",
        ],
    ],
    "001": [
        [
            "Lesson zero zero one begins before Salt.",
            "One Linux server is understandable: packages install software, files hold configuration, systemd supervises services, and permissions decide access.",
            "Salt does not replace that machine. It gives you a way to coordinate it without pretending the operating system vanished.",
        ],
        [
            "A script works when the world is small and obedient.",
            "At fleet scale, the same command becomes a question mark. Some machines already changed. Some are offline. One repository is broken. Someone edited another file by hand.",
            "Salt starts where the script stops being enough: evidence, targeting, and repeatable convergence.",
        ],
        [
            "Use this three question loop. What is true?",
            "What should be true? What operation could close the gap?",
            "Salt can help you ask and enforce those questions across machines. It does not know your operational goal by magic. The real skill is choosing the smallest justified change against the correct target set.",
        ],
        [
            "Now separate the actors. The Salt master publishes work and collects returns. The minion is the agent process on a managed machine.",
            "The salt command sends work through the master. Salt-call invokes local Salt functions. Salt-call local avoids the master.",
            "Most confusion clears once you ask: where does this code actually run?",
        ],
        [
            "A minion name is not proof of identity. A new machine presents a key. The master has to decide whether to trust it.",
            "During a repave, the machine may return with the same logical ID and a new key.",
            "That is not automatically good or bad. It is an identity transition that needs evidence, not muscle memory.",
        ],
        [
            "A Salt command is a distributed function call. Salt lab-rhel9 test dot ping does not send an ICMP ping.",
            "It asks the matched minion to run a small Salt function and return data.",
            "The command line is only the client layer. The work happens on the minion, through Salt's loaded execution modules.",
        ],
        [
            "Salt gives you a stable interface, not a universal operating system.",
            "The pkg interface may resolve to DNF on RHEL and APT on Debian. The service interface may resolve to systemd.",
            "This saves you from rewriting every operation, but it does not erase package names, paths, SELinux, or distribution behavior.",
        ],
        [
            "Execution functions perform operations and observations. States express conditions.",
            "Package install nginx says: perform this package operation. Package installed name nginx says: make the condition true, then report whether anything had to change.",
            "That contract is why states become the foundation for safe repetition.",
        ],
        [
            "A useful Salt state is a graph, not a shell script with YAML perfume.",
            "Install the package before managing the config. Validate the candidate config before replacing the live file. Enable boot behavior separately from current runtime.",
            "Restart only when the watched configuration actually changed.",
        ],
        [
            "Read Salt output as evidence, not reality itself. Test mode predicts changes.",
            "Result true says the state reported success. Changes empty says the state reported no mutation.",
            "None of that proves the application works. The final check must test the objective you actually care about, not merely a green process flag.",
        ],
        [
            "Once the single-machine model is clear, orchestration becomes less mystical. A minion state asks what should be true here.",
            "An orchestration asks how work across machines should depend on other work. Use it for ordering and gates.",
            "Do not mistake a long orchestration file for durable workflow ownership.",
        ],
        [
            "Thought experiment. A node is being repaved. Salt ping stops working.",
            "Thirty minutes later, the same minion ID returns with a new key.",
            "What evidence would convince you it is the correct rebuilt machine, fully configured, healthy in the cluster, and safe to return to service?",
        ],
    ],
}

# These substitutions change spoken pronunciation, not the displayed lesson text.
PRONUNCIATION = {
    "lab-rhel9": "lab, rel nine",
    "Salt-call": "salt call",
    "systemd": "system dee",
    "nginx": "engine x",
    "SELinux": "S E Linux",
    "RHEL": "Red Hat Enterprise Linux",
    "DNF": "D N F",
    "APT": "A P T",
    "ICMP": "I C M P",
    "HTTP": "H T T P",
    "pkg": "package",
    "ID": "I D",
    "YAML": "yamul",
}


def spoken(text: str) -> str:
    for old, new in PRONUNCIATION.items():
        text = re.sub(r"(?<!\w)" + re.escape(old) + r"(?!\w)", new, text)
    return text


def prepare(wave: np.ndarray, rate: int) -> np.ndarray:
    wave = np.asarray(wave, dtype=np.float32).reshape(-1)
    if not wave.size or not np.isfinite(wave).all():
        raise ValueError("Empty or non-finite synthesized audio")
    if float(np.sqrt(np.mean(wave ** 2))) < 1e-5:
        raise ValueError("Synthesizer returned silence")
    active = np.flatnonzero(np.abs(wave) > 0.00015)
    if active.size:
        pad = int(rate * 0.10)
        wave = wave[max(0, active[0] - pad): min(wave.size, active[-1] + pad + 1)].copy()
    peak = float(np.max(np.abs(wave)))
    if peak > 0.98:
        wave *= 0.98 / peak
    fade = min(int(rate * 0.006), wave.size // 4)
    if fade:
        wave[:fade] *= np.linspace(0.0, 1.0, fade, dtype=np.float32)
        wave[-fade:] *= np.linspace(1.0, 0.0, fade, dtype=np.float32)
    return wave


def main() -> None:
    root = Path("voice-output")
    root.mkdir(exist_ok=True)
    torch.set_num_threads(2)
    torch.set_num_interop_threads(1)
    sys.path.insert(0, str(Path("_vendor/VoiceStudio/backend").resolve()))
    from services.tts_backend import KittenTTSBackend

    engine = KittenTTSBackend()
    assert engine.id == "kittentts"
    assert VOICE in engine.PRESET_VOICES
    engine.ensure_ready()
    rate = engine.sample_rate
    assert rate == 24000
    manifest = {
        "status": "generating",
        "backend": "VoiceStudio.KittenTTSBackend",
        "voicestudio_revision": VOICE_STUDIO_REF,
        "model": "KittenML/kitten-tts-mini-0.8",
        "voice": VOICE,
        "voice_alias": "Jasper",
        "speed": SPEED,
        "sample_rate": rate,
        "hosted_tts_api_used": False,
        "clips": [],
        "package_versions": {},
    }
    for package in ("kittentts", "torch", "onnxruntime", "soundfile", "huggingface-hub"):
        manifest["package_versions"][package] = importlib.metadata.version(package)
    manifest_path = root / "manifest.json"
    manifest_path.write_text(json.dumps(manifest, indent=2))
    full_lesson_files = []
    for lesson, slides in LESSONS.items():
        folder = root / lesson
        folder.mkdir(exist_ok=True)
        full = []
        for slide_index, segments in enumerate(slides, 1):
            for stage_index, text in enumerate(segments, 1):
                begin = time.monotonic()
                speech = spoken(text)
                sentences = re.split(r"(?<=[.!?])\s+", speech.strip())
                parts = []
                for sentence in sentences:
                    wav = engine.generate(sentence, voice=VOICE, speed=SPEED, language="en")
                    parts.append(prepare(wav.detach().cpu().numpy(), rate))
                    parts.append(np.zeros(round(rate * 0.23), dtype=np.float32))
                samples = np.concatenate(parts[:-1])
                duration = len(samples) / rate
                if not 0.35 <= duration <= 90:
                    raise ValueError(f"Implausible clip duration: {duration}")
                name = f"{slide_index:02d}_{stage_index:02d}.wav"
                path = folder / name
                sf.write(path, samples, rate, subtype="PCM_16")
                record = {
                    "lesson": lesson,
                    "slide": slide_index,
                    "stage": stage_index,
                    "path": f"{lesson}/{name}",
                    "text": text,
                    "spoken_text": speech,
                    "duration_seconds": duration,
                    "sample_count": len(samples),
                    "peak": float(np.max(np.abs(samples))),
                    "rms": float(np.sqrt(np.mean(samples ** 2))),
                    "sha256": hashlib.sha256(path.read_bytes()).hexdigest(),
                    "generation_seconds": time.monotonic() - begin,
                }
                manifest["clips"].append(record)
                manifest_path.write_text(json.dumps(manifest, indent=2))
                full.extend([samples, np.zeros(round(rate * 0.6), dtype=np.float32)])
                print(json.dumps({k: record[k] for k in ("lesson", "slide", "stage", "duration_seconds", "generation_seconds")}), flush=True)
        full_path = root / f"{lesson}_narration.wav"
        sf.write(full_path, np.concatenate(full), rate, subtype="PCM_16")
        full_lesson_files.append((lesson, full_path))
    manifest["status"] = "complete"
    manifest["clip_count"] = len(manifest["clips"])
    manifest_path.write_text(json.dumps(manifest, indent=2))
    (root / "lesson_text.json").write_text(json.dumps(LESSONS, indent=2))

    # Independent ASR is a heuristic review aid, not a listening-quality score.
    try:
        from faster_whisper import WhisperModel
        recognizer = WhisperModel("tiny.en", device="cpu", compute_type="int8", cpu_threads=2)
        reports = []
        for lesson, path in full_lesson_files:
            segments, info = recognizer.transcribe(str(path), language="en", beam_size=3, vad_filter=False, condition_on_previous_text=False)
            result = [{"start": s.start, "end": s.end, "text": s.text} for s in segments]
            reports.append({"lesson": lesson, "segments": result})
        (root / "asr_review.json").write_text(json.dumps(reports, indent=2))
    except Exception as exc:
        (root / "asr_review.json").write_text(json.dumps({"status": "unavailable", "error_type": type(exc).__name__, "message": str(exc)[:1000]}, indent=2))


if __name__ == "__main__":
    main()
