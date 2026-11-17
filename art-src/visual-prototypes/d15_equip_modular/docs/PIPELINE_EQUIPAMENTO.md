# Pipeline — equipamento modular (D15)

Como adicionar uma peça sem recriar o protagonista.

## 1. Adicionar um equipamento

1. Escolher categoria em `assets-user/equip/`.
2. Criar `equip/<cat>/<id>/` com `referencia/`, `modelo/`, `blender/`,
   `renders/`, `metadata.json` (ver `PADRAO_EQUIPAMENTO.md`).
3. Preencher `validation_status: draft`.

## 2. Preparar a referência

- Uma peça por imagem; fundo limpo; silhueta legível.
- Preferir ortográfica isolada (ex.: render só da espada).
- Evitar fotos com mão/corpo colados se a meta for só a arma.
- Registrar origem e licença no metadata / README da pasta.

## 3. Gerar malha no Hunyuan

```bash
source ~/AI-3D/environments/hunyuan3d/bin/activate
export HF_HOME=~/AI-3D/cache/huggingface TORCH_HOME=~/AI-3D/cache/torch
export TMPDIR=~/AI-3D/cache/temporary HY3DGEN_MODELS=~/AI-3D/cache/hy3dgen
# Liberar VRAM (fechar Gradio se estiver aberto)
python ~/AI-3D/scripts/generate_shape.py \
  --image ~/AI-3D/inputs/.../ref.png --name minha_peca --seed 7
```

- Só geometry (`--disable_tex` no Gradio).
- Copiar o GLB para `modelo/` e anotar no metadata como **experimental**.

## 4. Importar no Blender

```bash
blender --background --python ~/AI-3D/scripts/import_glb_blender.py -- caminho/model.glb
```

Ou abrir o protagonista **em cópia/piloto**, nunca sobrescrevendo
`swordsman_rig_v007.blend`.

## 5. Padronizar escala

1. Medir bbox da peça canônica (se existir) ou do socket.
2. Escala uniforme até o comprimento bater com a referência (ex. lâmina ~0,91 m
   no v007).
3. Origem no ponto de grip / encaixe, não no centro da bbox.

## 6. Definir sockets

Ver `sockets/SOCKETS_D15.md`. Parent rigid → osso `sword` / `hand.R` etc.

## 7. Testar o encaixe

1. Cena com base + peça.
2. Equipado / desequipado (hide da peça).
3. Conferir interseção com mão/luva e orientação da lâmina.

## 8. Validar a animação

Obrigatório antes de `anim_ok`:

- Idle / rest
- Run (pelo menos 1 ciclo)
- Attack básico

Armadura/capa: inspecionar volumes e clipping. GLB Hunyuan bruto quase sempre
precisa retopo + weights.

## 9. Renderizar sprites (futuro)

Fluxo previsto (não executado na D15):

1. Holdout do corpo para camadas (já usado em V2.2 arma/capa).
2. Mesmo pivô de sola e célula 128×160.
3. Publicar só via pipeline de `art-src/player/scripts/post/`, sem substituir
   atlas oficiais sem aprovação.

## 10. Preservar a identidade do protagonista

- Não alterar cabelo/rosto/proporções sem autorização.
- Hunyuan **não** redefine o herói; no máximo gera peças experimentais.
- Conflitos de design → registrar e pedir decisão (ver `BASE_SPEC.md`).
- Comparar silhueta com D7-B / D12-A em escala de jogo antes de promover arte.
