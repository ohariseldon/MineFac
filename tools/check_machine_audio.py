"""Portable sound/resource/license checks; --decode additionally requires ffmpeg/ffprobe."""
import argparse
import array
import hashlib
import json
import math
import struct
import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
ASSETS = ROOT / 'content/assets/veio'


def check(decode=False):
    config = json.loads((ROOT / 'tools/machine_audio.json').read_text())
    sources = ROOT / 'third_party/audio/kenney'
    provenance = json.loads((sources / 'SOURCES.json').read_text())
    for pack, meta in provenance.items():
        assert meta['license'] == 'CC0-1.0'
        assert (sources / pack / 'LICENSE.txt').is_file()
        for filename, sha in meta['members'].items():
            assert hashlib.sha256((sources / pack / filename).read_bytes()).hexdigest() == sha, filename
    registry = json.loads((ASSETS / 'sounds.json').read_text())
    langs = [json.loads((ASSETS / f'lang/{code}.json').read_text()) for code in ('pt_br', 'en_us')]
    assert set(registry) == {'machine.'+name for name in config['events']}
    expected = set()
    for name, event in config['events'].items():
        entry = registry['machine.'+name]
        assert all(entry['subtitle'] in lang for lang in langs)
        assert len(entry['sounds']) == len(event['files']) >= 2
        assert 0 < event['duration'] <= (2.4 if event.get('ambient') else .9)
        for index, variant in enumerate(entry['sounds']):
            assert variant['attenuation_distance'] == 8
            assert variant['name'] == f'veio:machines/{name}_{index}'
            path = ASSETS / 'sounds' / (variant['name'].split(':')[1]+'.ogg')
            expected.add(path)
            data = path.read_bytes()
            assert data[:4] == b'OggS', path
            offset = data.index(b'\x01vorbis', 0, 256)
            assert data[offset+11] == 1, f'{path}: spatial audio must be mono'
            assert struct.unpack_from('<I', data, offset+12)[0] == 22050
            if decode:
                probe = json.loads(subprocess.check_output(['ffprobe','-v','error','-show_format','-show_streams','-of','json',str(path)]))
                assert float(probe['format']['duration']) <= event['duration'] + .02
                assert probe['streams'][0]['codec_name'] == 'vorbis'
                pcm = subprocess.check_output(['ffmpeg','-v','error','-i',str(path),'-f','s16le','-'])
                samples = array.array('h', pcm)
                peak = max(map(abs,samples)) / 32768
                rms = math.sqrt(sum(v*v for v in samples)/len(samples))/32768
                assert .01 < peak < .75 and rms > .001, (path,peak,rms)
                if event.get('ambient'): assert rms <= .115, (path,rms)
                # Fade endpoints avoid abrupt machine loop boundaries.
                edge = round(22050*.003)
                assert max(map(abs,samples[:edge]+samples[-edge:])) < 8192, path
    assert set((ASSETS / 'sounds/machines').glob('*.ogg')) == expected
    seen = set()
    for profile in config['profiles']:
        assert 24 <= profile['interval'] <= 80 and 0 < profile['volume'] <= .75
        if profile['ambient']:
            assert profile['interval'] == 48 and config['events'][profile['work']]['duration'] == 2.4
        for kind in ('work','finish'):
            assert profile[kind] is None or profile[kind] in config['events']
        for block in profile['blocks']:
            assert block not in seen, block
            seen.add(block)
            assert (ASSETS / f'blockstates/{block}.json').exists(), block
    return len(expected)


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--decode',action='store_true')
    args = parser.parse_args()
    print(f'{check(args.decode)} licensed spatial sound files checked' + (' and decoded' if args.decode else ''))
