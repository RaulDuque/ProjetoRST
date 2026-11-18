# N3.1 — Comparação do mesmo gesto

O gesto é o `attack_basic` do manequim V2.2, quadros 0 (impacto), 3 (seguimento) e 6 (recuperação). Mesma pose, mesmo enquadramento ortográfico, mesma luz do arquivo.

- Caminho A: o render bruto. `docs/development/n3_1/evidence/raw_attack_00_impacto.png` e os vizinhos. O recorte do rosto está em `raw_face_crop.png`.
- Caminho B: a redução automática. `auto_attack_00_impacto.png` e a folha `same_gesture_attack.png`. O rótulo da folha diz que não é pintura.
- Caminho C: desenho independente do mesmo gesto. Não executado. Não há artista nesta sessão, e um script de cápsulas não conta como desenho à mão.

`before_after_n3.png` coloca a figura geométrica da N3 ao lado deste gesto. São desenhos diferentes. A cápsula não é uma alternativa equivalente.

No bruto, o perfil tem rosto, casaco, bota e capa vermelha atrás do corpo. A mão do golpe está vazia porque a espada foi escondida de propósito. Na redução de 102 px a silhueta do casaco e da capa permanece; o rosto some. A paleta quantizada é sobretudo azul-escuro e o vermelho da capa. Pele, metal e tecido não ficam separados.

`backgrounds_attack.png` mostra o mesmo quadro automático sobre cinza neutro e sobre um fundo gótico escuro. A leitura no escuro depende da massa do casaco e da capa, não só de um fio branco.

`silhouette_attack.png` é a massa opaca do quadro automático, sem cor interna.
