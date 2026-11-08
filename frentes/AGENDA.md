# Agenda da vila (0.27)

O prefeito ganha um papel na história do jogo: o tempo passa com estações, domingos e feriados; personagens
chegam à prefeitura pedindo festas e obras para a cidade, e o prefeito transforma cada pedido numa missão
secundária para o jogador. Inspirado no calendário e no centro comunitário de Stardew Valley, dentro das
regras da economia do Veio (nada cria moeda; o dinheiro público volta como gasto público).

Código: `native/api/src/workforce/society/agenda.rs` (sobre `climate.rs`, que já define as estações).

## Calendário

- O ano tem 4 estações de 6 dias (`climate::SEASON_DAYS`), 24 dias no total. O **6º dia de cada estação é
  domingo**.
- **Feriados fixos** (6 por ano). No feriado **a vila para de trabalhar**: os postos descansam (estado
  *Feriado*, menos o quartel, que fica de plantão) e, de dia, os aldeões vão à praça mais perto da
  prefeitura (ou à própria prefeitura) comemorar, com música.

| Data | Feriado | Quem pede | O que pede |
|---|---|---|---|
| Primavera, 1 | Dia da Fundação | Seu Anselmo, o primeiro morador | 2 bolos, 32 pães, 4 bandeiras brancas |
| Primavera, 4 | Festa das Flores | Dona Margarida, das floristas | 16 papoulas, 16 dentes-de-leão, 4 garrafas de mel |
| Verão, 3 | Festa de São João | Seu Benedito, o sanfoneiro | 24 batatas assadas, 2 fogueiras, 16 biscoitos |
| Outono, 4 | Festa da Colheita | Tia Ceres, dos lavradores | 48 trigos, 8 tortas de abóbora, 16 pães |
| Inverno, 3 | Festa das Luzes | Mestre Lumen, o vidreiro | 16 lanternas, 8 velas, 16 vidros |
| Inverno, 6 | Virada do Ano | Pequena Aurora | 16 fogos, 1 bolo, 16 biscoitos |

- **Domingos sem feriado** têm um evento menor, que muda a cada semana: Feira de domingo (Dona Lia),
  Baile de domingo (Juca) ou Torneio de pesca (Pescador Tião). A vila não para; o evento só sobe o ânimo.

## Visitantes e missões

1. **Dois dias antes** de um feriado ou domingo, o visitante do evento chega à prefeitura (entidade
   `veio:visitor`, um comerciante viajante parado, com nome e roupa da profissão). O chat avisa.
2. **Clicar no visitante** mostra a conversa no chat: cumprimento, o pedido e o que falta (ou o
   agradecimento, se já está pronto). A tela de troca não abre.
3. **A missão:** entregar os materiais no espaço *Agenda* da prefeitura (3º espaço). A entrega vai para o
   pedido com a data mais próxima; o que não serve fica no espaço.
4. **Pronto:** a prefeitura **reembolsa à companhia** o valor de referência do que foi entregue, com a
   conta pública (até o que tiver). No dia do evento, o ânimo sobe.
5. **Obras cívicas:** a cada 4 dias, se não houver outra aberta, chega um pedido de **parque**, **praça** ou
   **monumento** para o bairro com mais moradores sem lazer. A missão é construir o bloco cívico nesse
   bairro (a até 32 blocos), em 6 dias. A receita do bloco usa os materiais (mudas, flores, cercas;
   tijolos, lanternas, sino; ouro, pedra talhada). A prefeitura paga a obra à companhia (240, 420 e 720
   moedas).

Até 3 pedidos abertos ao mesmo tempo. Visitantes de pedidos resolvidos vão embora; visitantes que nenhum
pedido conhece (de antes de reabrir o mundo) são retirados no tick deles, e os pedidos abertos chamam o
visitante de novo no turno seguinte.

## Efeitos

| O quê | Efeito |
|---|---|
| Feriado preparado | ânimo +20 a +25 no dia |
| Feriado sem preparo | a vila para do mesmo jeito; ânimo −10 |
| Domingo preparado | ânimo +8 |
| Obra cívica pronta | ânimo +10 (monumento +12); lazer permanente a 32 blocos |
| Obra cívica não feita no prazo | ânimo −5 |
| Todo dia | o ânimo volta 3 pontos em direção a 50 |

- **Ânimo** (0 a 100, 50 neutro) multiplica o IDH de cada família de **0,9x a 1,1x** (sem passar de 1).
- **Lazer** (parque, praça ou monumento a até 32 blocos) passa a ser o 4º serviço do IDH, junto com
  saneamento, energia e comércio. A partir da fase 2, o prefeito aponta casas sem lazer como problema.
- O feriado custa um dia de produção: é a troca entre produzir e o bem-estar da vila.

## Onde ver

- Prefeitura, aba **Agenda** (setas `<` `>` no canto do título): o dia (estação, dia, útil, domingo ou
  feriado), o ânimo, o próximo feriado e os pedidos abertos com o que falta e os dias.
- Livro na prefeitura: o relatório termina com a agenda completa.
- Painel dos blocos cívicos: quantas casas têm lazer graças a eles.

## Custo

Tudo roda no ciclo do prefeito. A virada do dia (eventos, pedidos novos, ânimo) acontece uma vez por dia;
visitantes e obras cívicas são conferidos no turno (1 min); a reunião do feriado chama até 24 aldeões a
cada 10 segundos. Blocos cívicos se registram a cada 5 segundos, como os poços.

## Integração

- A **Festa da colheita** das obras (`projects.rs`, festa repetível no centro de controle) continua
  existindo. A do calendário é municipal (prefeitura), na data fixa. Se quiserem uma só, o caminho é o
  feriado do calendário ativar o efeito `FESTIVAL` das obras no dia.
- Novos feriados, domingos ou obras cívicas: uma linha em `EVENTS` e os textos (`veio.agenda.<chave>.*`).
