# 2026-11-08 · Realismo visual (PR #51)

Nota curta de mudança. Entra no próximo compilado da consolidação mestra (SIS-PES / visual).

| Commit | O que mudou | Por quê |
|---|---|---|
| f85b918 | Pacote `veio-industrial` 3.0.1: 4 saídas de pós (`glow_h`, `glow`, `light` = AO + raios, `sky_mirror` = nuvens no céu e reflexos no chão). `check_shaders.py` barra mais de 4 saídas. | O Horizon recusou o pacote no PC do Luiz: "6 [[post]] outputs; a pack may write 4". |
| 9eeccba | O launcher apaga os `.wgsl` da versão anterior antes de copiar; `check_shaders.py` recusa `.wgsl` que nenhum estágio usa. | O Horizon recusou de novo: "ao.wgsl is not used by any stage or in shared". O #47 também passou a refazer a pasta do zero. |

## Teste no jogo (Luiz, 2026-11-08, RTX 3050, Equilibrado, resolutionScale 85)

- A 3.0.1 carrega.
- Água: "perfeita". Sombras: "muito boas". Ficam como estão.
- Céu: "completamente quebrado". Brilho, contraste e iluminação: "esquisitos". A diferença geral parece pequena e as texturas parecem iguais.
- Direção do Luiz: em vez de inventar, refazer com técnicas conhecidas e comprovadas, bem documentadas. Pode usar pacotes de texturas de terceiros.

## 3.1.0 (próximo commit desta nota)

- `sky.wgsl`: céu físico (Rayleigh, Mie, ozônio; coeficientes de Hillaire 2020/Bruneton), 8/12/16
  passos por pixel e 2/4 rumo ao sol, exposição 20 e ACES (Narkowicz), depois gama 2,2. Sol
  com o disco atenuado pelo ar. Faixa baixa misturada à névoa do jogo para não haver costura.
  Prévia numpy: `/mnt/project-files/visual/previa-ceu-fisico.png` (meio-dia, 25°, 6°, 1°, -4°).
- `blur_h.wgsl`: o céu só brilha acima do branco; no chão o joelho subiu para 0,72–0,95.
- `finish.wgsl`: contraste 0,26 → 0,10, calor dourado ×2,1 → ×1, saturação 0,16 → 0,10, sem
  tonalização dividida, bloom 0,55 → 0,45.
- Pacote de texturas indicado (não incluído): Vanilla Normals Renewed (LabPBR, uso gratuito).
- Água e sombras intactas, porque o Luiz aprovou as duas.
- Não visto no jogo ainda: falta o Luiz testar.
