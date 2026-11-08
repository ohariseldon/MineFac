"""Write a self-contained, offline HTML sound sampler; no game launch required."""
import base64
import html
import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


def main():
    config = json.loads((ROOT / 'tools/machine_audio.json').read_text())
    cards = []
    for name, event in config['events'].items():
        volume = max((p['volume'] for p in config['profiles'] if name in (p['work'], p['finish'])), default=.16)
        players = []
        for i, _ in enumerate(event['files']):
            data = (ROOT / f'content/assets/veio/sounds/machines/{name}_{i}.ogg').read_bytes()
            encoded = base64.b64encode(data).decode('ascii')
            players.append(f'<label>Variação {i+1}<audio controls preload="none" data-volume="{volume}" src="data:audio/ogg;base64,{encoded}"></audio></label>')
        cards.append(f'<article><h2>{html.escape(event["pt"])}</h2><p>{html.escape(name)} · {event["duration"]} s · CC0 / Kenney</p>'+''.join(players)+'</article>')
    out = Path(sys.argv[1]) if len(sys.argv)>1 else ROOT / '.build/machine-audio.html'
    out.parent.mkdir(parents=True,exist_ok=True)
    out.write_text('''<!doctype html><html lang="pt-BR"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1">
<title>Veio · Sons das máquinas</title><style>
body{font:16px system-ui;background:#17211d;color:#edf2e7;max-width:1100px;margin:auto;padding:32px}h1{font-size:32px}p{line-height:1.6;color:#bbcabc}main{display:grid;grid-template-columns:repeat(auto-fit,minmax(285px,1fr));gap:16px}article{background:#25342c;border:1px solid #485d4b;border-radius:12px;padding:20px}h2{font-size:19px;margin:0}label{display:block;margin-top:16px}audio{display:block;width:100%;margin-top:6px}a{color:#b1e2bd}
</style><h1>Veio · Sons das máquinas</h1><p>Prévia dos timbres isolados, sem simulação da distância ou do ambiente do jogo.
O volume inicial segue o perfil mais alto de cada evento. Os controles permitem comparar as duas variações.
No jogo, os sons usam a categoria Blocos e limites de sobreposição.</p><main>'''+''.join(cards)+'''</main><p>Fontes CC0: <a href="https://kenney.nl/assets/impact-sounds">Impact Sounds</a>, <a href="https://kenney.nl/assets/rpg-audio">RPG Audio</a> e <a href="https://kenney.nl/assets/sci-fi-sounds">Sci-Fi Sounds</a> — Kenney. Cortes, filtros, afinação, repetição de impactos e fades pelo Veio.</p><script>
const sounds=[...document.querySelectorAll('audio')];for(const sound of sounds){sound.volume=Number(sound.dataset.volume);sound.addEventListener('play',()=>{for(const other of sounds)if(other!==sound)other.pause()})}
</script></html>''',encoding='utf-8')
    print(out)


if __name__ == '__main__':
    main()
