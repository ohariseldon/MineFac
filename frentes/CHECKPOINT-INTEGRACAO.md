# Checkpoint para o próximo coordenador

Integração interrompida a pedido do usuário em 08/11/2026. Não continuar a consolidação automaticamente nem apresentar os ZIPs anteriores como a build final.

## Onde está o trabalho

- Repositório de integração isolado: `E:/GPTmodMine/.build/consolidated-016`.
- Branch de trabalho: `codex/consolidated-016`.
- A mesma ponta está disponibilizada no repositório principal pela branch `codex/checkpoint-consolidacao`.
- Checkpoint de código anterior a esta nota: `f99f66a`; merge principal das melhorias: `52f5155`.
- A pasta principal contém alterações de outra frente no mapa finito. Elas não foram sobrescritas nem incluídas neste checkpoint por cópia de arquivos.

## Melhorias preservadas

Tutorial com oito objetivos dentro do jogo, progresso individual salvo e comprovação de produção e transporte reais. Reconhece esteiras, braços, funis e carregadores, incluindo rotas mistas. O contador distingue ciclos consumidos de produtos extras de produtividade.

Quadro da fábrica, progressão por fases, pedidos diários e diagnóstico de gargalos. Seus alertas aguardam o tutorial ser concluído ou ocultado. O censo e seus registros temporários são limpos ao iniciar outro mundo.

Megafábricas: copiadora, marco de obra, elevadores e escadas dos carregadores. O retorno da copiadora vai a quem clicou em LAN; projetos em memória e ajustes pendentes não passam para outro mundo. Cobrança, recusa sem gastar materiais, cópia e giro têm testes.

Realismo 1.1 e formas distintas das máquinas. Os commits locais `3106849` e `2bc62ec` estão integrados por merge, assim como `13758bd`, `fd2a446`, o bundle 0.16.0 e todos os pacotes recebidos até megafábricas.

Base remota incorporada até `5217b62` (0.17.2), incluindo combate, plantão noturno do quartel, otimizações de NBT/painel/esteiras e busca incremental dos carregadores. A rotina de tick otimizada também executa o tutorial e o censo. Foram mantidos os campos visuais específicos de drones, caravanas, quadro e farol opcional, além da recuperação de ondas que não conseguem criar inimigos.

Histórico e decisões adicionais em [CONSOLIDACAO.md](../CONSOLIDACAO.md). A auditoria `branches-consolidadas.json` registra 29 referências remotas conferidas, todas incluídas na ponta auditada. As fontes dos 83 arquivos do último ZIP foram comparadas com a branch dos patches: nenhuma divergência.

## Verificações já realizadas

- 248 testes de regras aprovados; oito cenários opcionais fora da execução comum.
- Três cenários isolados aprovados: retomada/troca de mundo do tutorial, transporte misto sem esteiras e prioridade do objetivo sobre os alertas do quadro.
- Verificações específicas de desempenho e do plantão noturno aprovadas após a integração das otimizações.
- 558 JSON, traduções, 96 páginas do guia técnico e 68 texturas validados; quatro livros conferidos quanto ao espaço das páginas.
- 34 formas de máquina verificadas, com 27 silhuetas distintas.
- Shader 1.1.0 aprovado pelo `horizon-shader check`; seis testes do launcher aprovados na etapa anterior.

## O que não foi fechado

A candidata atual não teve uma nova compilação nativa completa nem um novo ZIP final após as últimas entregas. A compilação completa aprovada anteriormente era da consolidação 0.16.1, antes de quadro, megafábricas e base 0.17.2. O jogo não foi aberto por esta integração.

A conferência posterior usando metadados avulsos do Horizon encontrou um cache incoerente: `horizon_client` espera um hash de `horizon_core` diferente dos arquivos que outra frente recompilou no cache compartilhado. Isso é um impedimento dessa verificação intermediária; não constitui aprovação da API real da candidata. Evitar disputar `E:/GPTmodMine/.build/runtime/native` enquanto a outra frente estiver usando-o. O próximo coordenador deve fazer a compilação final com todos os componentes encerrados.

A versão da candidata segue a base 0.17.2; o launcher ainda tem a versão de pacote 0.16.1. Antes da entrega final, escolher uma versão consolidada nova, alinhar os manifestos/documentação, regenerar o conteúdo, compilar o launcher e gerar o pacote único. Não publicar `release/Veio-0.17.2.zip` como se contivesse esta integração: é o arquivo histórico recebido da branch remota.

O mapa finito permanece desativado nesta candidata. A frente local ainda estava alterando o checkpoint em `work-in-progress/finite-world`; sua entrega final deve ser tratada pelo novo coordenador. Nenhum trabalho dessa frente deve ser apagado ou substituído para aplicar este checkpoint.
