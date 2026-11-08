# Checkpoint do mapa finito — 8 de novembro de 2026

Pedido: Overworld de 15.000 × 15.000 blocos, muralha de bedrock, todos os biomas
e recursos garantidos, Nether com portais seguros, preservando mundos existentes.
Contrato completo em [MAPA.md](../../MAPA.md).

**Rascunho não compilado, não ativado e não incluído no pacote jogável.** O usuário
pediu um checkpoint e a integração de `E:/veio-casa-do-fundador.zip` antes de
continuar o mapa. `native/horizon-mixin.toml` mantém apenas o mod estável; a
configuração experimental está em `horizon-mixin.toml.draft` nesta pasta.

## Código preparado

- `map_core.rs`: geometria exata (inclusive a fração do Nether), detecção de saves
  anteriores e estado da sessão. Contém testes geométricos ainda não executados.
- `world_catalog.rs`: catálogo obtido das fontes reais de biomas e estruturas do
  Horizon, sem manter uma cópia manual da lista do Minecraft.
- `world_plan.rs`: busca determinística de seeds derivadas, testemunhos de regiões
  utilizáveis e reservas de estruturas. É uma proposta inicial; ainda não comprova
  todos os recursos nem a obtenção de todos os conteúdos de sobrevivência.
- `world_pipeline.rs`: proposta de wrapper do gerador nativo, geração sob demanda,
  muralha, recorte de chunks parciais e referências das estruturas reservadas.
- `world_entry.rs`: proposta de certificado por mundo, sem converter saves antigos.
- `world_portals.rs`, `world_travel.rs`, `world_level.rs` e `hooks/`: proposta de
  proteção dos destinos, espera na origem e limites por dimensão.

## Bloqueio confirmado e próximos passos

1. O Horizon usado pelo Veio é `66f3981f522b24c250b21523a70080965b940424`.
   Sua API de trait não tem `native_chunk_generate`; o rascunho usa mixins de
   funções internas e precisa conferir os tipos e os contratos reais.
2. `horizon-mixins` compara a escrita dos tipos de retorno, não apenas equivalência
   Rust. O primeiro planejamento falhou em `finite_world_open`: ajustar o retorno
   do helper para `ControlFlow<Result<PreparedWorld>>` no escopo do alvo.
   Revisar todos os demais helpers pelo mesmo critério.
3. O alvo `OverworldGenerator::for_world_with_settings` retorna `Result<Self, ...>`.
   A ferramenta rejeita cancelamento com `Self`. Trocar essa integração: uma opção
   é um hook `RETURN` em `OverworldGenerator::worker`, que retorna `GeneratorWorker`
   concreto, com wrapper compartilhado entre os workers do mesmo pipeline.
   Não ativar o construtor recursivo atual sem resolver essa incompatibilidade.
4. Compilar o código restante; conferir privacidade dos módulos, tipos de NBT,
   geração e decoração de estruturas, cidades com navio e todos os recursos. A
   validação de seeds/biomas por amostragem ainda não certifica o contrato completo.
5. Passar os overlays e parâmetros reais de geração ao planejador; hoje ele lê
   somente a base Normal. Resolver retomada de certificado temporário interrompido,
   troca de mundos, memória dos caches e persistência das reservas.
6. Testar portais com saída obstruída, ambos os sentidos, faces e cantos, entidades
   com passageiros, reinício, comandos, respawn e voo. Ainda não houve compilação
   nem teste no jogo dessa implementação.
7. Provar que abrir um mundo antigo não escreve configuração de mapa nem modifica
   terreno. Usar mundos descartáveis fora de `.game`, nunca saves reais.

## Ferramentas locais disponíveis

Os arquivos abaixo são cache local ignorado pelo Git, não dependências distribuídas:

- Rust: `.build/runtime/rust/1.99.1-x86_64-pc-windows-gnu/bin/`;
  acrescentar também `mingw64/bin/` ao PATH e usar o Cargo cache em
  `.build/runtime/cargo`.
- Fonte original do Horizon: `.build/runtime/native/source/66f3981f522b24c250b21523a70080965b940424`.
- Metadados reais: `.build/runtime/native/target/fast/deps`.
- Ferramenta compilada: `.build/world-map-tools/target/release/horizon-mixins.exe`
  (fonte MIT `5a64af938ec86b00d53d16a408aabc0038fb3e57`).
- Tentativas e diagnóstico: `.build/world-map-apply.log`,
  `.build/prepare_world_check.py`, `.build/world-map-bundles/`.
  O script de preparação ainda aponta para `native/`; adaptar para esta pasta.
- Recursos próprios do Minecraft preparados localmente, nunca versionar:
  `.launcher/versions/native-66f3981-915a4701db5b/.local/horizon-client/26.3`.

Não anunciar mapa finito entregue, não copiar esse manifesto para o bundle ativo e
não instalar em mundos existentes antes de concluir a compilação e a validação.
