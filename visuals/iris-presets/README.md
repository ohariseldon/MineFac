# Ajustes "Veio 8 GB" para shaders Iris

O Horizon roda shader packs no formato Iris/OptiFine. Estes arquivos são **só ajustes de opções**,
os mesmos que você faria no menu do shader. Não contêm código dos shaders, e os shaders em si são
baixados do Modrinth no PC de quem joga.

O launcher já faz isso para o BSL no perfil **Realismo máximo** (`--visual maximo`): baixa o
`BSL_v10.1.8.zip` do Modrinth, confere o SHA-512, copia o ajuste abaixo e escolhe o pack. Para os
outros, à mão:

Ponha o `.txt` em `shaderpacks/`, ao lado do `.zip` de mesmo nome (convenção do Iris:
`<arquivo do pack>.txt`, uma `OPÇÃO=valor` por linha).

| Pack | Base | O que mudou e por quê |
|---|---|---|
| Complementary Reimagined r5.9.3 | perfil MEDIUM | `RP_MODE=3` lê os mapas LabPBR do pacote de texturas do Veio (relevo e brilho por material); sombra 1, a 96 blocos; raios de luz no nível 1; reflexo de blocos 1; sem luz colorida nem reflexo em espaço do mundo, que são os mais caros; sem sombra de entidades. |
| BSL 10.1.8 | perfil LOW + filtro de sombra, AO e raios de luz | `ADVANCED_MATERIALS` e `MATERIAL_FORMAT=1` (LabPBR) para usar o relevo e o brilho das texturas; parallax ligado; sombra 1024 a 128 blocos; sem TAA nem sombra colorida. O BSL é o mais pesado dos três, e é aqui que mais se ganha. |
| Photon v1.4 | perfil low + VL e sombras suaves | Mapas LabPBR ligados (normal, especular, POM com 16 amostras em vez de 40); AO por SSAO em vez de GTAO; sem SSRT de sombra, reflexos de ambiente e cáusticas; nuvens com reconstrução temporal 4×. |

Os números de FPS de cada um no PC de referência (RTX 3050, 8 GB de RAM) entram aqui depois do
teste no jogo.
