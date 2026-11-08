# Transporte entre zonas

Primeira integração sobre a `main` 65cab50: terminais ferroviários, portos de
drones e a fila existente do capataz/carregador compartilham os mesmos filtros
e inventários. Esta entrega está em código e testes offline; aguarda compilação
pelo launcher e validação no jogo. Não inclui os PRs separados de balanceamento
ou pesquisa de jazidas, ainda não integrados à base.

## A intenção do jogador

O jogador define o produto de saída de cada terminal e liga as zonas. A fila
do capataz encontra produtos compatíveis nas máquinas e cria as viagens dos
carregadores. Produtos recebidos podem abastecer as máquinas locais.

```text
Zona da mina                             Zona da oficina
Extratora -> carregador -> terminal A     terminal B -> carregador -> prensa
                           |                  ^
                           +-- vagão/drone ---+

Volta: terminal B envia chapas; terminal A recebe as chapas.
```

O sistema não inventa uma receita, destino ou trajeto ferroviário. Se várias
máquinas oferecem um mesmo produto, aplica as regras existentes da fila.
Para um carregador buscar diretamente numa máquina e entregar no terminal,
coloque a mesa do capataz e o posto ao alcance dos dois pontos. Sem capataz,
o carregador continua seu comportamento local de abastecer com o estoque do
posto e recolher para ele; não cria sozinho toda a fila entre máquinas.

## Configurar

1. Fabrique o **Terminal ferroviário** ou o **Porto de drones**. Receitas
   liberadas ao obter módulo mecânico e circuitos, respectivamente.
2. Use o **Cartão de rota** no terminal: percorre os canais 0–16; agachado
   percorre no sentido contrário. Canal 0 desativa a associação automática.
   O cartão é uma ferramenta reutilizável, não é consumido.
3. Agache e clique com o produto que deve **sair**. Copia o filtro sem gastar
   o item. Agachar com mão vazia ou chave limpa o filtro de envio.
4. Dois terminais do mesmo tipo, dimensão e canal reconhecem o filtro de
   saída do parceiro como produto de recebimento. Mais de dois torna a
   associação ambígua; drones aguardam em vez de escolher um destino arbitrário.
5. Para ferrovias longas ou uma associação manual, clique **sem agachar** com
   o produto que deve **chegar**. Isso fixa o filtro de recebimento. Não precisa
   manter o terminal distante carregado para preservar esse filtro. Trocar o
   canal limpa essa configuração, que pode ser reaplicada.
6. Abra o painel com a mão vazia. Os três slots da esquerda são **Enviar**;
   os três da direita são **Recebido**. O ícone sobre o bloco indica o envio.

Os lados podem enviar produtos diferentes: A envia cobre bruto, B envia chapas.
Drones não iniciam uma rota cujas duas pontas exportam o mesmo produto, para
evitar que ele fique voltando automaticamente.

## Ferrovia

Use trilhos e vagões com baú do Minecraft. Os terminais são os pontos de
integração entre trabalhadores e transporte, sem reescrever a física do trem:

- Carregadores abastecem **Enviar** e retiram apenas de **Recebido**.
- Funis retiram **Enviar** do terminal para a plataforma de carga.
- Funis da plataforma de descarga inserem somente o produto aceito em
  **Recebido**. Carga incompatível fica no funil/vagão.
- Esteiras/braços que usam a saída normal do mod retiram o recebido. Os funis
  usam a saída ferroviária específica, separada para carregar o vagão.

Construa plataformas separadas para descarregar e carregar, ou alterne os
funis com redstone. Deixar carga e descarga ligadas sobre o mesmo vagão pode
recolher imediatamente o que acabou de carregar. Trilhos energizados,
detectores e redstone definem paradas, partida e volta. Um circuito de viagem
pode repetir sem intervenção, mas o mod não constrói esse circuito ou escolhe
entroncamentos automaticamente.

Os trilhos, trilhos energizados, vagão com baú e funil também aparecem na aba
Veio — Logística. Esta entrega não adiciona locomotiva, acoplamento de vagões,
sinais ferroviários próprios ou um planejador automático de trilhos.

## Drones

Cada porto incorpora um drone com uma viagem de cada vez. Não é necessário
contratar outro aldeão para pilotá-lo.

| Regra | Valor |
|---|---|
| Alcance entre portos | 64 blocos de distância em 3D, mesma dimensão |
| Carga | até 16 itens de um produto por viagem |
| Energia armazenada | 8.000 FE |
| Custo de ida e volta | 800 + 32 × distância arredondada para cima, em FE |
| Tempo por trecho | 20 + 2 × distância arredondada para cima, em ticks |
| Pausa | sinal de redstone no porto de origem |
| Chamar de volta | agachar + chave no porto de origem |

Exemplo a 16 blocos: 1.312 FE por ida/volta, 52 ticks por trecho, fora
carregamento, descarga e pausas. Trens continuam sendo a opção para volumes
maiores e distâncias longas; drones resolvem ligações curtas com menos estrutura.

Deixe o trajeto direto livre, aproximadamente três blocos acima dos portos.
Em portos com alturas diferentes, o trajeto também inclina. O drone verifica
o espaço do próximo passo e se o chunk está carregado. Um obstáculo ou chunk
descarregado pausa o avanço; não atravessa o terreno nem força carregamento.
Não há navegação para contornar obstáculos nesta primeira entrega.

A carga sai do estoque **Enviar** e permanece no slot **Voo** do porto de
origem até ser aceita no destino. A animação mostra o drone transportando-a;
não existe uma segunda cópia em uma entidade. Ao receber, o destino guarda
em **Recebido**, sem misturar com o que deve enviar na volta.

Destino cheio aceita apenas o que couber; o restante aguarda no voo, sem
cobrar energia novamente. Depois da entrega, há o trecho de volta antes de
despachar outra carga. Agachar + chave retorna a carga não entregue ao estoque
de envio; se ele estiver cheio, ela aguarda no slot Voo. Retirar manualmente
esse slot impede a entrega daqueles itens, sem duplicá-los.

Os parâmetros da rota ficam bloqueados durante a viagem. Chame de volta antes
de reconfigurar. O drone é uma animação de uma viagem persistida no porto,
não uma entidade independente com vida, inventário ou combate próprio.

## Conservação, saves e limites

- Estoques, filtros, canal, destino, carga em voo, progresso e retorno ficam
  nos itens/NBT do bloco. A volta de um save não deve entregar a mesma carga
  duas vezes nem cobrar uma segunda vez pela mesma viagem.
- O registro em memória só descobre parceiros carregados; é separado por
  epoch do mundo, dimensão, tipo de terminal e canal, com anúncios vencidos.
  A entrega revalida o bloco real de destino antes da inserção.
- Retirar/remover o destino interrompe a entrega, conservando a carga. O
  jogador pode recolocar o terminal compatível ou chamar o drone de volta.
- O slot Voo fica fora das entradas/saídas de trabalhadores e funis. Como os
  demais itens do bloco, participa da conservação do inventário no mundo.
- Transporta itens simples, sem dano ou componentes NBT personalizados.
  Ferramentas encantadas e outros itens com dados não são convertidos em cópias
  simples; são recusados.
- Não há carregamento forçado de chunks. Máquinas, trabalhadores, drones e
  vagões dependem das áreas simuladas pelo jogo. Um filtro ferroviário
  configurado permanece válido mesmo com a outra zona descarregada.
- Novos IDs não alteram os inventários ou os campos dos blocos antigos. O menu
  mantém 13 dados; nos novos terminais, o slot 8 mostra a fase de voo e o 12,
  o canal. A sincronização visual do voo ocorre a cada quatro passos ou mudança
  de estado, evitando uma atualização de inventário a cada quadro.

## Validação

Executar da raiz: `python3 tools/check_project.py` e `python3 tools/check_rust.py`.
O conteúdo inclui modelos, receitas, avanços, traduções, ajuda e guia em ambos
os idiomas. O gerador do guia continua dentro do limite de 100 páginas.

Testes novos: fluxo real de carregador máquina → terminal e recebido → máquina;
separação dos filtros; voo com energia e tempo; retomada de save; descarga
parcial; destino cheio/removido; redstone; cancelamento com estoque cheio;
obstáculos; fronteira descarregada; alcance; ambiguidade; separação de mundos,
dimensões e modos; cartão e amostra preservados; filtro ferroviário persistente.

As assinaturas novas `LevelRef::epoch`, `level_block` e `level_is_loaded` foram
conferidas na referência Horizon 66f3981. `native_entity_merge_nbt` documenta
que não recarrega vagões, e `native_move_items` trabalha com contêineres em
blocos: por isso a adaptação ferroviária usa funis reais. Não foi alegada
validação da integração no launcher, do circuito ferroviário no jogo ou da
aparência em movimento. Esses testes ficam para a integração local, com bump
da versão consolidada e reconstrução nativa; este PR não gera pacote de release.
