# Limitações D3

1. **Não é arte final B.** O estilo é deliberadamente geométrico (camadas).
   Os conceitos D1/D2 continuam a referência estética de “acabamento”.
2. **Não é pixel art manual humana.** É construção procedural documentada.
3. **Sockets não remedidos** a partir destes frames; compatibilidade com
   `player_sockets.ron` é teórica.
4. **Front/back** são esquemas; a autoridade é a vista lateral.
5. **Ataque — fases visuais ≠ mecânicas.** No jogo, a hitbox da espada básica
   pode estar ativa desde o início do `FitToAction` (sem startup RON na Iron
   Sword). Os 2 primeiros frames “ant” são só leitura visual; **não**
   afirmam startup mecânico de 2 frames.
6. **Equipamentos pesados:** overlay de metal **não** substitui silhueta
   larga; armadura pesada verdadeira precisará de sprites próprios.
7. **Espectral:** estudo de 4 passos; sem motes/atlas de produção.
8. **Sem integração Bevy** (proibido alterar o jogo nesta fase).
