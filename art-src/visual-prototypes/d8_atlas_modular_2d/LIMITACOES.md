# Limitações D8

1. Separação automática D2 não gera todas as camadas pedidas com qualidade
   igual ao sprite completo (`coat_back`/`cape_front`/`hair_back` vazios).
2. Luvas/bainha são recortes fracos — trocas usam proxies.
3. Armadura pesada D4 não compartilha o mesmo shading denso do clássico D2.
4. Em run pesado, placas seguem sockets por translação; sem deformação de malha.
5. Método B de capa pode esticar textura; C só desloca.
6. Sem integração Bevy / sem novos itens de inventário.
7. Testes não aprovam arte.
