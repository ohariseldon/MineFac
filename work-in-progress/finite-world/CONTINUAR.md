# Checkpoint do mapa finito — 8 de novembro de 2026

**Interrompido a pedido do usuário para commit e integração por outro coordenador.
Não ativado no jogo e não incluído no pacote jogável.**

O bundle ativo em native/ permanece na 0.16.1. As melhorias estão nesta pasta,
em [world_map_probe.rs](../../tools/world_map_probe.rs) e no preparador de testes.
Nenhum mundo real foi aberto, convertido, recortado ou regenerado. Requisitos em
[MAPA.md](../../MAPA.md). A integração anterior da casa do fundador está preservada.

## Melhorias preservadas

- Wrapper real do gerador por RETURN de OverworldGenerator::worker, com cache
  compartilhado de referências fracas. Resolve o bloqueio anterior com Self.
- Retornos e bindings dos helpers ajustados à ferramenta. Entrada movida para
  módulo filho de world_loading.rs, usando overlays reais de geração.
- Geração sob demanda, muralhas, recorte por coluna, altura dos mapas de superfície
  recalculada e filtro de entidades/ticks na borda. Overworld/End de 15.000;
  Nether lógico de 1.875, sem arredondar a área para cima.
- Catálogo real de biomas/estruturas, busca determinística de seeds derivadas,
  regiões verificadas e checagem adicional de espaço em cavernas/Nether.
- Estruturas reservadas com peças nativas; leitura correta de strings NBT para
  reconhecer navios do End. A extensão mais recente reserva variantes do bastião
  extraídas do pool real, mas ainda precisa de execução.
- world_resources.rs: catálogo experimental, habitats e veios suplementares,
  paleta salva, regras nativas de sobrevivência, abelhas e arqueologia.
  **A auditoria do Nether ainda falha: não anunciar recursos garantidos.**
- Preservação dos habitats já finalizados, evitando repor minério ou apagar
  construções ao gerar um vizinho. O motor restaura chunks completos em
  VanillaTerrain::proto_beside_final; não reaplicar reservas nesses chunks.
  Os casos novos do probe para mineração/construção ainda não foram executados.
- Certificado formato 2, impressão digital do catálogo, trava de sessão nativa,
  arquivo temporário único e gravação atômica. Saves sem certificado mantêm o
  fluxo original. Detecção inclui players/, dimensions/ e dados sem level.dat.
- Portais com alvo na dimensão de destino, checagem de candidatos/corpo/saída,
  alternativas a saídas inseguras, espera na origem, espaço conservador para
  montados e preservação de árvores de entidades. A origem das entidades fica
  salva para retorno após falha/reinício. **Sem teste em partida.**
- Limite por dimensão, proteção de escrita/mineração, teletransporte, pérolas e
  fruta do coro; contenção autoritativa do jogador/espectador e borda fixa.

## Evidências e pendências exatas

1. Aplicação real de mixins passou com **22 injeções e 9 módulos**. O manifesto
   atual tem **24 injeções**: os dois hooks finais de origem de entidades ainda
   não passaram por nova aplicação.
2. horizon-world e finite_map_probe compilaram no motor real, Rust 1.99.1,
   perfil fast, offline. Não é a API substituta de testes.
3. Antes dos habitats, a seed -2152535657050944081 passou com **56 + 5 + 5
   biomas**, reservas de **47 + 4 + 1 estruturas**, oito sondagens de faces/cantos
   por dimensão em três alturas e ar no exterior distante. Esse certificado é
   do formato antigo 1 e não certifica o código completo mais recente.
4. Último cargo check do cliente: Arc<Resources> onde se esperava &Resources.
   Corrigido no código para size(&game.res, ...), mas **não verificado novamente**.
5. Última auditoria nativa dos habitats: **Overworld passou; Nether falhou** ao
   exigir beetroots, carrots, pitcher_crop, potatoes, sugar_cane, torchflower_crop
   e wheat. O End não chegou a ser auditado nessa execução. Revisar a extração
   do grafo: entradas/predicados/provedores condicionais não podem ser tratados
   indistintamente como saídas produzidas. Não contornar criando água e lavouras
   do Overworld no Nether.
6. A falha anterior em lily_pad foi corrigida com suporte de água; a auditoria
   seguinte passou no Overworld. Falhas de habitat independentes da seed agora
   interrompem a busca para não repetir o mesmo erro 64 vezes.
7. Faltam: executar variantes extras do bastião, geração/habitats em várias seeds
   (inclusive extremos de i64), preservação após mineração/vizinhos e reabertura,
   testes geométricos embutidos e saves antigos descartáveis.
8. Faltam testes de portais em partida: dois sentidos, faces/cantos, coordenadas
   negativas, saídas obstruídas/externas, passageiros, itens, reinício e LAN.
   O motor já tem runner sem interface gráfica em game/command_harness.rs e
   tools/test_command_exec_parity.py: HORIZON_COMMAND_SCRIPT/OUTPUT, operações
   JSON cmd, portal, settle e tick com n/pace_ms.
9. Só depois promover para native/, atualizar versões/traduções, conferir a API
   do Veio, compilar executável completo e empacotar. Não há nova entrega jogável.
   A versão 0.15.2 do manifesto de rascunho é histórica; atualizar ao promover.

## Reprodução local

Caches ignorados pelo Git; nunca incluir dados do Minecraft no commit:

- Fonte: .build/runtime/native/source/66f3981f522b24c250b21523a70080965b940424.
- Rust: .build/runtime/rust/1.99.1-x86_64-pc-windows-gnu/bin/; acrescentar também
  mingw64/bin/ ao PATH. CARGO_HOME=.build/runtime/cargo.
- CARGO_TARGET_DIR=.build/runtime/native/target; perfil fast, --offline.
- Ferramenta: .build/world-map-tools/target/release/horizon-mixins.exe, MIT,
  revisão 5a64af938ec86b00d53d16a408aabc0038fb3e57, versão 0.6.0.
- Recursos próprios: .launcher/versions/native-66f3981-915a4701db5b/.local/horizon-client/26.3.
- Estágio: .build/world-map-stage, com sincronizações de módulos posteriores;
  não equivale à aplicação do manifesto final de 24 hooks.
- Logs: .build/world-map-apply.log, .build/world-map-client-check.log,
  .build/world-map-compile.log, .build/map-habitat-audit.log,
  .build/map-seed-1.log e .build/map-resource-probe.log.
- Certificado inicial: .build/map-seed-1.json, formato 1 incompatível com o 2
  atual. Gerar certificados novos; não reutilizar esse arquivo nos testes finais.

python tools/prepare_world_map_check.py copia o rascunho para
.build/world-map-bundles/veio, retirando somente [api], tratado pelo launcher
na integração final. Com Cargo no PATH, aplicar:

    horizon-mixins apply --workspace <fonte-Horizon> --mods .build/world-map-bundles --out <estágio-novo>

**Windows:** a ferramenta não substitui atomicamente estágio existente. Usar
saída nova ou remover apenas o estágio gerado, conferindo antes o caminho
absoluto e o marcador .horizon-mixins-stage.json.

No estágio, copiar world_plan.rs para
crates/horizon-world/examples/finite_plan_support.rs. Copiar
tools/world_map_probe.rs para examples/finite_map_probe.rs, trocando a linha
// PLANNER_MODULE por #[path="finite_plan_support.rs"] mod planner;.
Compilar com cargo build --offline --profile fast -p horizon-world --example
finite_map_probe.

Argumentos: <recursos-preparados> <certificado-novo.json> <seed>
[--audit-only|--habitats|--sources]. --audit-only verifica os habitats antes da
busca; --habitats confere o terreno nativo decorado. Usar diretórios de jogo e
configuração descartáveis, nunca .game/, saves reais ou contas pessoais.
