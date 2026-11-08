# 2026-11-08 · Áudio das máquinas · volume 2,5×

**O que mudou:** todos os volumes de perfil das máquinas foram multiplicados por 2,5.

- `tools/machine_audio.json` e `native/api/src/soundscape/data.rs` (a tabela gerada, editada à mão com os mesmos valores para dispensar o ffmpeg):
  - prensa 0,168 → 0,42
  - britador 0,154 → 0,385
  - montagem, corte, colheitadeira e silvicultora 0,126 → 0,315
  - moinho 0,11 → 0,275
  - braços 0,1 → 0,25
  - motores (perfuratrizes, fornos, geradores e drones) 0,09 → 0,225
  - cozinha, semeadora e rodas/turbinas 0,084 → 0,21
  - eletrônica 0,07 → 0,175
  - esteiras 0,055 → 0,1375
- Aviso de avaria (`soundscape.rs`): 0,16 → 0,4.
- Os limites dos testes subiram: perfil ≤ 0,5 em `check_machine_audio.py` e no teste da prensa; esteira vazia < 0,1.
- `frentes/AMBIENTE-E-MOVIMENTO.md` agora traz a nova faixa dos perfis de fundo.

**Por quê:** o Luiz testou no jogo e achou o som das máquinas baixo demais. Ele pediu cerca de 2,5×: "tipo ASMR", perceptível, mas sem incomodar.

**O que não mudou:** atenuação (8 blocos), cadências, OGGs, limites de vozes e as proporções relativas (conclusão ×0,6, transporte vazio ×0,65). Todos os volumes continuam abaixo de 1,0, então o alcance no jogo não muda.

**Verificação:** `check_machine_audio.py` passou. O `cargo test` não rodou no PC do Luiz, que não tem a toolchain Rust; precisa rodar na nuvem/CI.

**Branch:** `claude/som-maquinas-2.5x` (a partir de `claude/project-thread-l94frq` @ 53bccaa).
