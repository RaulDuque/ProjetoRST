# N4-1-A — Comparações visuais e evidências

Pasta: `docs/development/n4/evidence/n41a/`.

## Offline (dados → imagem)

| Arquivo | Conteúdo |
|---|---|
| `n41a_idle_overlay.png` | idle: original, máscara da família (verde) e, para A e para B: o item sozinho, as camadas ocultadas (vermelho; nenhuma no idle) e o resultado |
| `n41a_run_c1_overlay.png` | o mesmo para os 3 quadros C1 (A oculta `shirt_torso`; B não oculta nada) |
| `n41a_idle_hide_check.png`, `n41a_run_c1_hide_check.png` | "e se o item ocultasse a camisa do tronco": os texels que ficariam expostos aparecem em vermelho. É onde a sobreposição simples não basta, e o motivo de B não ocultar nada e de A não ocultar no idle |
| `n41a_sheet_<item>_<clip>.png` | spritesheet composta exportada offline (personagem + item) |
| `n41a_run_c1_items.gif` | corrida C1 sem item, com A e com B (2/15 s por quadro, ciclo de 0,4 s) |
| `coverage.json` | por item e quadro: texels do item, buracos com as regras dele (0), texels ocultos, ocultos não cobertos (0), expostos se ocultasse a camisa, e a checagem do Bevy |

## No Bevy real (`cargo run --example n4_sprite_machine -- --equipment [--run-c1]`)

16 capturas `bevy_n41a_*.png`: idle e corrida #0, #2 e #4, cada um sem item, com A e com B; espelhado com A; grade e pivô com B em fundo claro. À esquerda fica sempre o original aprovado.

**Checagem pixel a pixel (`check_bevy.py`):** o canvas da direita de cada captura em fundo escuro foi comparado com a composição calculada offline a partir dos mesmos dados (camadas da base + item, com o HIDE do item e a ordem da família, espelhada quando for o caso). **14 de 14 capturas com 0 pixels diferentes**; as 2 de grade em fundo claro ficaram de fora, porque as linhas da grade não entram na comparação. As checagens anteriores (idle N4-A e corrida C1) continuam com 0.

## Onde a sobreposição não funciona (pontos registrados)

1. **Idle + HIDE:** a camisa do idle é uma camada só (tronco + manga). Ocultá-la apagaria a manga. A família não oculta nada no idle. Para um item que precise cobrir a camisa do idle por inteiro, seria preciso separar a manga do tronco da camisa no idle: uma mudança nos dados da N4-A, não feita.
2. **Item mais curto + HIDE:** B exporia a parte de baixo e a gola da camisa. A regra é "só oculte o que cobre"; o verificador recusa.
3. **Decote sem gorjal + HIDE:** a primeira versão de A falhou no verificador (17 a 31 texels expostos no pescoço). Virou gorjal.
4. **Linha do ombro:** onde o tronco encontra o ombro da manga (corrida), o contorno do peitoral forma uma linha reta. É limitação da geometria (polígono do tronco), registrada em `N41A_LIMITACOES.md`.
5. **Ombros e braços** não são cobertos (não há ombreiras): a manga da camisa continua visível. É o comportamento correto para um peitoral; uma armadura com ombreiras seria outra família.
