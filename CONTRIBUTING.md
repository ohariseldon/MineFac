# Trabalhar no MineFac / Veio

Este repositório contém o Veio, um jogo de automação industrial executado como mod nativo do Horizon. A versão atual é a **0.8.0**, com energia por postes e fios, transporte, processamento, contratos, tutorial dentro do jogo, colheitadeira, torre Tesla, poluição com ataques de Saqueadores, módulos de melhoria e visual no estilo do Minecraft.

## Começar na nuvem

1. Clone este repositório e crie uma branch a partir da `main` atualizada.
2. Leia o [README](README.md) para conhecer o comportamento implementado.
3. Execute `python tools/check_project.py`. Requer Python 3.11 ou superior e nenhuma biblioteca adicional.
4. Execute `python tools/check_rust.py`. Requer Rust 1.90 ou superior e funciona sem internet.
5. Faça as alterações e envie um pull request para `main`. Atualize sua branch antes de integrar trabalhos simultâneos.

O código e o conteúdo podem ser editados na nuvem sem instalar o launcher, baixar Minecraft ou abrir um mundo. A verificação de conteúdo confere JSON, versões, traduções, navegação do guia, texturas 16x16, modelos e receitas liberadas por avanços.

A verificação Rust compila `native/api/src` contra `native/offline-check/horizon-client`, um substituto com as assinaturas reais do Horizon `66f3981` para os métodos que o Veio usa, e roda os testes de regras. Ela não comprova funcionamento no jogo e não substitui a compilação do launcher. Ao usar um método novo do `Host` ou um gancho novo, copie a assinatura de `native-hooks.json` e de `crates/horizon-client/src/game/` do Horizon para o substituto.

## Onde alterar

- `native/api/src/lib.rs`: simulação, energia, inventários, transporte, combustíveis, jazidas e regras de interação.
- `native/api/src/farm.rs`: colheitadeira.
- `native/api/src/defense.rs`: torre Tesla e registro de monstros hostis.
- `native/api/src/power.rs`: postes, fios aéreos e área de alimentação.
- `native/api/src/pollution.rs`: poluição por chunk, avisos, ondas noturnas e o objetivo dos Saqueadores.
- `native/api/src/upgrades.rs`: módulos de velocidade, eficiência e produtividade.
- `native/api/src/tests.rs`: testes de regras.
- `native/api/src/presentation.rs`: painéis no estilo vanilla e desenho animado das máquinas e esteiras.
- `native/api/src/contracts.rs`: pedidos e recompensas da central de projetos.
- `native/api/src/tutorial.rs`: ajuda no inventário, rótulos e dicas dos painéis.
- `content/data/veio`: registros, receitas, avanços e tabelas de saque.
- `tools/build_content.py`: lista única de blocos e itens. Gera modelos, blockstates, registros, menus, saques, abas criativas e a tag de picareta.
- `tools/build_textures.py`: pixel art 16x16 desenhada em texto. Gera todos os PNG do Veio.
- `content/assets/veio`: traduções `pt_br` e `en_us` e os arquivos gerados acima.
- `tools/build_tutorial.py`: fonte das páginas do guia, ajuda traduzida e roteiro inicial. Rode este script ao alterar o tutorial e inclua os JSON gerados no mesmo commit.
- `CREDITS.md`: origem das imagens.

Ao adicionar uma máquina: inclua-a em `BLOCKS` de `tools/build_content.py`, desenhe a frente em `tools/build_textures.py`, crie a receita e o avanço que a libera, acrescente nome, ajuda e página do guia em `tools/build_tutorial.py` e rode os três geradores antes das verificações.

As receitas do guia são extraídas das receitas de fabricação reais. Ao mudar ingredientes de máquinas, gere novamente o guia. Ao mudar regras de processamento, atualize também os textos explicativos do gerador do tutorial.

## Compilar e instalar para jogar

A versão 0.6 foi compilada com Horizon `66f3981f522b24c250b21523a70080965b940424`, base `nightly-2026-11-05-66f3981`, Rust 1.99.1 e perfil `dev`. A 0.7 e a 0.8 foram escritas contra as assinaturas desse mesmo commit e conferida com a verificação offline; a primeira compilação pelo launcher confirma. As APIs do Horizon evoluem; conserve essa referência ao investigar incompatibilidades. O launcher resolve e prepara o ambiente nativo. Um `cargo build` isolado nesta pasta não substitui esse processo.

Em uma máquina Windows com Horizon Launcher instalado e uma instância chamada Veio, execute, ajustando os caminhos:

```powershell
& "$env:LOCALAPPDATA\Programs\Horizon Launcher\horizon-launcher.exe" native-build "E:\GPTmodMine\native" --instance Veio --trust-native-code --profile dev
```

Com o jogo fechado, copie o **conteúdo** de `content/` para a pasta `horizon-mods/veio/` da instância. Ela deve conter diretamente `horizon-mod.toml`, `assets/` e `data/`. Selecione Veio no launcher para jogar.

No ambiente local original, launcher, instâncias e cache estão no E:, configurados por `HORIZON_LAUNCHER_LOCATION_FILE=E:\GPTmodMine\.build\location.json`. Esse arquivo pertence à instalação local e não é distribuído. Uma máquina nova deve configurar seus próprios caminhos no launcher. Os diretórios `.build/`, `.launcher/` e `.game/` são recriados localmente e não precisam ser baixados do GitHub.

## Desempenho

Nada de varrer blocos ou entidades a cada tick. Guarde listas em NBT ou em registros em memória e recalcule em intervalos escalonados pela posição. Use `draw.keep()` quando o desenho não depende do tempo e `draw.spin()` para peças que giram em velocidade constante.

## Combinar o trabalho

- Concentre cada branch em uma mudança e descreva o efeito para o jogador no pull request.
- Faça a verificação leve ao concluir uma entrega. Evite abrir o jogo e repetir testes a cada ajuste; o usuário prefere testar os lotes concluídos.
- Separe comportamento implementado de propostas futuras. O README registra as limitações atuais dos contratos e da progressão.
- Preserve mundos existentes e dados salvos ao mudar máquinas. Explique quando uma alteração exigir migração.
- Mantenha downloads, compiladores, executáveis, mundos, contas e configurações pessoais fora do Git. Não distribua arquivos do Minecraft. As referências de textura são resolvidas pela instalação do jogador.
