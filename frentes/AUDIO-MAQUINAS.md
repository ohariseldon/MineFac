# Feedback sonoro das máquinas

Entrega na branch `codex/machine-audio`, sobre as formas de máquinas de
`codex/machine-identity` (PR #31) e a logística de `codex/interzone-logistics`
(PR #28). Não mescla as frentes pendentes, não incrementa versão e não altera
receitas, energia, inventários ou formato de saves. A integração final continua
com o Codex local.

## Comportamento

13 eventos, com duas variações cada, atendem 26 blocos. Prensa com batidas
metálicas; montagem com encaixes; britador com impactos graves; corte com lâmina;
extratoras e geração com motores; fornos com motor discreto e utensílio metálico
na conclusão; circuitos com ruído eletrônico; moinho com rotação; cozinha,
colheitadeira e semeadora com ruídos leves; silvicultora com serra; roda d'água e
turbina com mecanismo de madeira. Braços só clicam após transferência efetiva.
Drones emitem som na posição de voo, em vez de manter um zumbido no porto.

As variações são escolhidas pelo motor de áudio, com afinação espacial de
0,97–1,03. Categoria **Blocos**, legendas PT-BR/EN-US, atenuação de 8 blocos.
Solar, acumuladores, cabos, armazéns e lâmpadas ficam silenciosos. Trilhos e
animais mantêm os sons oferecidos pelo Minecraft; não há trilha musical global.

São fragmentos de 0,65–0,9 segundo repetidos em cadências de 1,2–2,3 segundos,
não loops permanentes. Consumidores só renovam com `Status == 1`; geradores só emitem quando
realmente acrescentam energia, antes da distribuição. Energia guardada sem
combustível não mantém o motor soando. Falta de material, energia,
saída cheia, redstone, quebra ou descarregamento interrompem a renovação; o
fragmento já iniciado termina em menos de um segundo na afinação mínima.
Conclusões usam aumento de `Produced` e têm prioridade sobre o som de trabalho.

Um representante por família/célula de 8×8×8, até duas novas vozes por dimensão
por tick e até quatro vozes sobrepostas perto de cada ouvinte. O limite inclui
conclusões: um produto pode terminar sem som em uma fábrica saturada. É feedback
ambiental, não contador sonoro de itens. Isso não limita a produção. Não há fila
de sons atrasados nem recuperação de sons perdidos.

O gancho global atualiza jogadores a cada dez ticks e expira vozes após vinte;
não há busca de blocos, entidades ou caminhos. Sem jogador próximo, não chama
`play_sound`. O pré-filtro de jogadores usa 12 blocos para tolerar movimentação
entre atualizações; o recurso de áudio aplica a atenuação efetiva. Há um teto de
256 registros de vozes no servidor. Estado efêmero reinicia na troca da época
do mundo ou regressão do relógio. Não usa campos NBT extras.

## Origem e manutenção

[Impact Sounds](https://kenney.nl/assets/impact-sounds),
[RPG Audio](https://kenney.nl/assets/rpg-audio) e
[Sci-Fi Sounds](https://kenney.nl/assets/sci-fi-sounds), de Kenney, todos **CC0**.
24 originais selecionados e licenças estão em `third_party/audio/kenney/`.
`SOURCES.json` registra URL pública e SHA-256 dos arquivos e dos ZIPs oficiais.
Os arquivos do Minecraft não são redistribuídos.

`tools/machine_audio.json` define fontes, cortes, afinação, impactos repetidos,
perfis, volumes e legendas. `python tools/build_machine_audio.py` requer ffmpeg,
verifica os hashes das fontes e gera OGGs mono 22.050 Hz, `sounds.json`, tabela
Rust e traduções. Filtros leves, normalização de pico e fades preservam sons
curtos. O mod usa as saídas versionadas; jogador e launcher não precisam de
ffmpeg nem baixar sons. O pacote inclui créditos e textos das licenças.

`python tools/preview_machine_audio.py` gera `.build/machine-audio.html`, uma
prévia independente com controles para ouvir cada variação. Não representa a
mixagem final no jogo.

## Evidência e teste jogável

- `python tools/check_project.py`: conteúdo, referências e cabeçalhos de áudio.
- `python tools/check_machine_audio.py --decode`: hashes, licenças, mono/Vorbis,
  taxa, duração, pico, sinal audível e fades; 26 arquivos decodificados.
- `python tools/check_rust.py`: 155 testes aprovados, dois opcionais ignorados.
  Casos novos cobrem estados silenciosos, conclusão, clique por transferência,
  fábrica densa, expiração, jogadores distantes, dimensões, época, emissão
  posicional/categoria e posição do drone na ida e volta.

Conferidos no código público do Horizon
`66f3981f522b24c250b21523a70080965b940424`: assinatura de `native_play_sound`,
resolução de namespaces, `sounds.json`, atenuação e legendas. O retorno da chamada
confirma enfileiramento, não reprodução audível. Não houve compilação pelo
launcher nem audição dentro do jogo nesta entrega.

Roteiro para a integração local:

1. Compilar com o launcher e instalar conteúdo e código juntos. Conferir Blocos
   e volume geral acima de zero, ativar legendas e operar uma prensa.
2. Comparar prensa, forno de ligas produzindo aço, moinho e colheitadeira com
   material/energia. Conferir o feedback de conclusão sem volume excessivo.
3. Parar por redstone, cortar energia, retirar ingredientes, encher a saída e
   quebrar a máquina: não iniciar fragmentos novos; aguardar o final do atual.
4. Montar várias máquinas vizinhas: conferir sobreposição discreta. Afastar-se
   e voltar; testar também com dois jogadores em regiões/dimensões diferentes.
5. Verificar braços com destino livre/cheio e drone na ida e no retorno.
6. Ajustar o volume de Blocos a zero e depois restaurar. Música e sons vanilla
   devem continuar seguindo suas próprias categorias.

Ajustes de timbre/volume e comportamento de carregamento dos recursos dependem
dessa validação jogável. Não se declara mixagem aprovada apenas pelos testes
offline.
