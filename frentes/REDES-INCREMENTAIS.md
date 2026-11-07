# Reconstrução incremental dos fios

Base: main `b43d4a4`. O orçamento anterior limitava duas reconstruções inteiras por
tick; cada uma ainda podia percorrer e ordenar uma vizinhança densa. Este lote
substitui esse caminho de execução por etapas retomáveis em `power/rebuild.rs`.

## Etapas e limites

1. Coletar candidatos dos chunks próximos. Cada passo consulta uma entrada do
   cadastro ou avança um chunk vazio. O cadastro usa BTreeMap por chunk, com cursor
   por posição; não copia todos os postes próximos ao começar. Cada candidato
   é validado no mundo antes de entrar no recorte.
2. Manter as seis escolhas próprias numa lista de tamanho fixo, ordenada por
   distância e posição. Não ordenar uma lista de todos os candidatos.
3. Para cada possível conexão reversa, comparar um par de postes por passo,
   parando ao achar seis candidatos melhores. Guardar alvo, cursor e ranking para
   continuar depois. Preservar tiers, alcances, desempates e união simétrica.
4. Preparar a saída em ordem de posição, validando novamente uma extremidade
   por passo. Comparar com o NBT antigo durante essa preparação. Publicar a lista
   completa ao terminar, sem um sort ou flatten de toda a vizinhança na conclusão.

Cada atendimento executa no máximo 64 etapas. A reserva global `Topology` agora
é de 4.096 etapas por tick para todas as dimensões, até 64 atendimentos completos.
Um lote que termina cedo também reserva suas 64 unidades. O FIFO do orçamento
distribui a vez entre os postes. Checagens de revisão fazem um número fixo de
consultas aos chunks próximos por atendimento, fora da contagem de etapas.

`Links` continua sendo a última lista completa. Energia, desenho dos fios e área
alimentada continuam usando essa lista enquanto a reconstrução espera ou avança.
Não é publicada uma metade do novo resultado. A sincronização ocorre quando a
lista final realmente muda. Alcance, taxas de energia e regras de escolha não
mudaram; a união das escolhas pode conter mais de seis conexões por poste,
como no algoritmo anterior.

## Mudanças no mundo e memória

Inclusão, remoção ou troca de tier no cadastro incrementa a revisão do chunk.
Uma mudança nos chunks acompanhados invalida o cálculo parcial, que recomeça
sem apagar os fios atuais. Uma extremidade removida/reposta é conferida novamente
durante a preparação; divergência invalida o resultado. Como nos caches anteriores,
alterações ainda não observadas no cadastro são descobertas nas leituras do mundo;
não há snapshot transacional de todos os blocos do jogo.

O trabalho parcial fica em memória, separado do save. Ao reabrir um mundo,
`TopologyPending` inicia uma reconstrução nova, preservando `Links`. Não se grava
o recorte inteiro de candidatos no NBT. Pausa de callbacks ou mudança de tier
descarta o trabalho antigo. Cada tick de simulação examina no máximo oito registros
de trabalho para recolher os que deixaram de responder; a coleta percorre o mapa
em rodízio, sem uma limpeza de todas as reconstruções de uma vez.

A memória ainda depende da quantidade de reconstruções pendentes e da densidade
dos recortes. Inserções nos vetores podem realocar; BTreeMap e alocação não têm
tempo constante garantido. O limite de etapas restringe as buscas e comparações,
não impõe um número de milissegundos à máquina do jogador. Construção contínua
na região pode provocar reinícios e aumentar a demora para atualizar os fios.

## Verificação

- Resultado comparado com o resolvedor anterior em recortes densos, tiers mistos,
  empates e distâncias nos limites de alcance, avançando só três etapas por chamada.
- Coleta real com limite de 64 leituras por atendimento, remoção no meio da
  reconstrução e manutenção da lista antiga até publicar a nova.
- Inclusão/troca no cadastro durante o trabalho, retomada com revisão nova e
  recolhimento de trabalho abandonado.
- 121 reconstruções simultâneas numa grade densa: todos os postes terminam,
  nenhuma rodada supera 4.096 etapas ou leituras de mundo, e todos os resultados
  coincidem com o resolvedor anterior. Contadores de etapas existem só nos testes.
- Os testes anteriores continuam cobrindo entrega de energia enquanto a rede
  espera, conservação de FE e a descoberta de área com mil postes.

São testes offline de lógica, orçamento e equivalência; não medem FPS. O cenário
histórico `dense_pole_links` em `tools/check_performance.py` fica como referência
do resolvedor anterior, não como medição deste novo caminho incremental.

Verificação do lote: 126 testes aprovados e duas ferramentas opcionais ignoradas;
475 JSONs, 93 páginas e 58 texturas conferidos. A compilação de metadados com a
API real passou sem avisos antes do último ajuste interno de reutilização do
registro de trabalhos. A repetição final não pôde usar o cache antigo: outra
frente recompilou dependências do launcher, e o `horizon_client` antigo deixou de
ser compatível com elas (`E0463`). Não se substituiu esse cache nem se reconstruiu
o motor. O código final passou na suíte offline; sua checagem final com a API
real e a linkedição ficam pendentes da consolidação do novo launcher. Sem teste
de FPS no jogo nesta entrega.

## Frentes na nuvem

As referências foram atualizadas antes do lote. A main ainda continha o orçamento
global do PR #27; não havia novos commits Cloud nem alterações posteriores nas
branches de desempenho do Grok. Economia, moradia, LAN, noite e pesquisa de jazidas
continuam separadas. O trabalho local de tutorial, launcher e mapa foi preservado.
Para mudar o cadastro de postes, atualizar sua revisão somente quando o conteúdo
muda; alterá-la a cada leitura reiniciaria os trabalhos continuamente.
