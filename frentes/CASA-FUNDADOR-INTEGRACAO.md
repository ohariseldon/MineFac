# Integração da casa do fundador — Veio 0.15.3

Integração local em 8 de novembro de 2026, solicitada pelo usuário a partir de
`E:/veio-casa-do-fundador.zip`. Referência da entrega Cloud: `b43d4a4`, Veio 0.15.0.
A comparação foi feita entre a base reconstruída pelo patch, os arquivos do ZIP e
a árvore local; o repositório completo do ZIP não substituiu a versão local.

Patch original: `c8f4206f4954d7938f1937a4261e910cd8a00271`, autoria Cloud/Claude,
preservada no commit de integração. SHA-256 do ZIP recebido:
`a77dee90067e4bd489ddde737aad80cec360abc4f2896bfe0cf7e4b247cbff06`.

## Comportamento integrado

- Casa mobiliada com painel, lareira, sofá, cama e mural de quatro mapas.
- Construção uma vez por mundo, com marca persistente e escolha do terreno.
- Presença em casa e refeições influenciam o ritmo das fábricas.
- Cama da casa e uma segunda cama externa; o renascimento respeita a cama marcada.
- Páginas da casa e das refeições no Guia da Vila Operária, com traduções.
- Foram preservados o tutorial atual, o transporte por braços/funis/carregadores,
  as otimizações locais e o launcher com escolha de visual.

Na integração foi removida uma declaração duplicada de `Host::acting_player` do
substituto offline. O estado em memória da casa, do mural e das refeições é
reiniciado no início de cada servidor para não passar de um mundo para outro;
os dados persistidos no painel continuam sendo carregados dele.

O ZIP não foi executado como um instalador e nenhum save de `.game` foi aberto,
recortado ou regenerado. A casa mantém o comportamento de construção automática
da entrega Cloud, inclusive em mundos que ainda não tenham a marca da casa.
Seu posicionamento e aparência ainda precisam de avaliação dentro do jogo.

## Verificação

- `python tools/check_project.py`: 480 JSON válidos, livros e traduções conferidos.
- `python tools/check_rust.py`: 136 testes passaram; quatro ferramentas de prévia
  e medição ficam ignoradas por padrão.
- `tools/check_native_api.py`: tipos conferidos contra os metadados reais do
  Horizon, sem erros.

Esses checks não substituem a compilação completa pelo launcher nem a validação
dentro do jogo. O pacote 0.15.3 leva o código integrado; o launcher compila o
módulo nativo quando essa versão é instalada.

## Mapa finito

O pedido de mapa de 15.000 blocos de lado continua registrado em [MAPA.md](../MAPA.md).
O rascunho foi preservado em
[work-in-progress/finite-world](../work-in-progress/finite-world/CONTINUAR.md), fora
do bundle nativo, com o bloqueio de compilação e os próximos passos documentados.
Não há mapa finito, muralha ou garantia de conteúdo ativados nesta versão.
