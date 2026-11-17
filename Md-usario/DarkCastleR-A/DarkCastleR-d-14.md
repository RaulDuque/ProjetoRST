# DARK CASTLE RPG — MEGA PROMPT D14
## Instalação Local Completa do Hunyuan3D-2 — Ubuntu + NVIDIA RTX 4050 6 GB

Você é um engenheiro especialista em Linux, Python, CUDA, PyTorch, modelos generativos 3D, Blender e integração de ferramentas de IA.

Sua missão é instalar, configurar, testar e documentar uma instalação local do projeto oficial:

https://github.com/Tencent-Hunyuan/Hunyuan3D-2

A instalação será utilizada futuramente para produzir personagens, inimigos, equipamentos, armas, armaduras e outros modelos 3D para o Dark Castle RPG.

**O objetivo desta fase é disponibilizar um gerador 3D local funcional, com interface no navegador, exportação GLB/OBJ e ambiente independente do projeto Rust.**

Não altere a engine, a arte publicada ou os arquivos de gameplay do Dark Castle RPG.

---

# 1. HARDWARE REAL DO COMPUTADOR

A auditoria anterior identificou:

| Recurso | Especificação |
|---|---|
| Computador | Acer Nitro |
| Sistema | Ubuntu 26.04.1 LTS |
| Kernel | 7.0.0-43-generic |
| Arquitetura | x86_64 |
| GPU | NVIDIA GeForce RTX 4050 Laptop |
| VRAM | 6141 MiB |
| Compute capability | 8.9 |
| Driver NVIDIA | 580.178.04 |
| CUDA informada pelo driver | 13.0 |
| Processador | Intel Core i5-13420H |
| RAM | Aproximadamente 14,6 GiB |
| Swap | 4 GiB |
| Disco disponível | Aproximadamente 131 GiB |
| Python do sistema | 3.14.4 |
| PyTorch | Não instalado na auditoria |
| CUDA Toolkit | Não instalado na auditoria |
| Blender | 5.2.4 LTS |
| Blender MCP | Já configurado anteriormente |

Esses dados são um ponto de partida.

Antes de instalar, confirme novamente o estado atual.

Não presuma que os valores de memória livre sejam os mesmos.

---

# 2. REGRAS DE SEGURANÇA

Você tem autorização para:

- Instalar dependências Python em ambiente virtual próprio.
- Clonar o repositório oficial.
- Baixar pesos de modelos oficiais.
- Instalar PyTorch compatível.
- Configurar uma interface Gradio local.
- Executar testes controlados na GPU.
- Criar scripts auxiliares de inicialização.
- Exportar modelos GLB/OBJ.
- Criar documentação de instalação.

Você NÃO tem autorização para:

- Substituir o Python do sistema.
- Modificar o ambiente virtual do Dark Castle RPG.
- Alterar Cargo ou Rust.
- Modificar drivers NVIDIA.
- Atualizar o kernel.
- Instalar outro driver CUDA global.
- Modificar a configuração existente do Blender MCP.
- Alterar `src/`, `assets/`, `saves/` ou `Md-usario/`.
- Apagar caches ou projetos existentes.
- Fazer commit ou push.
- Executar comandos destrutivos.
- Expor a interface diretamente à internet.

Não execute `sudo` automaticamente.

Se alguma etapa exigir permissões administrativas, registre o motivo e utilize uma alternativa sem privilégios quando possível.

Não instale dependências em Python global.

Não utilize `pip --break-system-packages`.

---

# 3. ARQUITETURA DE DIRETÓRIOS

A instalação deve ficar fora do repositório do jogo.

Escolha um diretório apropriado dentro da pasta pessoal, respeitando o espaço disponível.

Estrutura sugerida:

```text
~/AI-3D/
├── Hunyuan3D-2/
├── environments/
│   └── hunyuan3d/
├── models/
├── cache/
│   ├── huggingface/
│   ├── torch/
│   └── temporary/
├── inputs/
├── outputs/
│   ├── geometry/
│   ├── textured/
│   └── blender/
├── scripts/
├── logs/
└── reports/
```

Não presuma que essa estrutura já exista.

Não sobrescreva diretórios existentes sem inspecioná-los.

Não coloque checkpoints grandes em `/tmp`, porque neste computador ele é um `tmpfs` baseado em RAM.

Configure os caches para o NVMe.

---

# 4. AUDITORIA PRÉ-INSTALAÇÃO

Execute verificações de leitura:

```bash
uname -a
nvidia-smi
python3 --version
df -h /
free -h
git --version
```

Verifique também:

- `uv`
- `uvx`
- `pip`
- `cmake`
- `gcc`
- `g++`
- `ninja`
- `nvcc`
- PyTorch
- Ambientes Python existentes.

Verifique a disponibilidade atual da GPU.

Se houver outro processo consumindo muita VRAM, não inicie a inferência.

Não encerre processos de outras aplicações automaticamente.

---

# 5. CONSULTAR O REPOSITÓRIO OFICIAL

Clone:

```bash
git clone https://github.com/Tencent-Hunyuan/Hunyuan3D-2.git
```

Se a pasta já existir, verifique sua origem e estado antes de reutilizar.

Não execute `git pull` indiscriminadamente.

Registre:

- URL de origem.
- Branch.
- Commit.
- Versão de lançamento, quando disponível.
- Arquivos de instalação.
- Licença.
- Requisitos.
- Scripts de geração.

Leia:

- `README.md`
- `requirements.txt`
- `setup.py`
- `gradio_app.py`
- `minimal_demo.py`
- `LICENSE`

Verifique instruções atuais antes de aplicar comandos.

**Não modifique o código oficial para esconder erros de compatibilidade.**

Se precisar criar patches locais, documente-os e mantenha-os separados.

---

# 6. LICENÇA E PROVENIÊNCIA

O repositório oficial usa a Tencent Hunyuan 3D 2.0 Community License.

Antes de baixar e utilizar os modelos:

1. Leia a licença.
2. Identifique restrições territoriais.
3. Identifique condições de distribuição.
4. Identifique condições de utilização comercial.
5. Identifique restrições relativas aos resultados gerados.
6. Verifique possíveis licenças adicionais dos checkpoints utilizados.
7. Registre como atribuir corretamente o modelo.

O objetivo inicial é uso local e experimental.

Não publique modelos gerados como assets finais do jogo nesta fase.

Não utilize imagens de terceiros sem verificar os direitos de uso.

---

# 7. INSTALAÇÃO DO PYTHON ISOLADO

O Python global é 3.14.4.

Não utilize essa versão automaticamente.

Investigue uma versão Python apropriada para o repositório e o ecossistema PyTorch, preferencialmente Python 3.11, caso seja compatível com as dependências atuais.

Utilize `uv` ou outro gerenciador já disponível para instalar a versão de Python no espaço do usuário.

Não substitua `/usr/bin/python3`.

Crie ambiente virtual dedicado.

Exemplo conceitual:

```bash
uv python install 3.11
uv venv ~/AI-3D/environments/hunyuan3d --python 3.11
```

Valide a instalação antes de prosseguir.

Se a versão 3.11 apresentar incompatibilidade comprovada, selecione outra versão adequada e documente a escolha.

Não altere ambientes Python de outros projetos.

---

# 8. INSTALAÇÃO DO PYTORCH COM CUDA

Instale PyTorch usando instruções compatíveis com:

- Linux x86_64.
- Python escolhido.
- NVIDIA RTX 4050.
- Compute capability 8.9.
- Driver NVIDIA 580.

**Não confunda CUDA 13.0 informada pelo driver com uma obrigação de instalar PyTorch CUDA 13.0.**

Selecione uma build de PyTorch com runtime CUDA compatível e suportado oficialmente.

Prefira a fonte oficial:

https://pytorch.org/get-started/locally/

Não instale CUDA Toolkit globalmente por padrão.

Depois, execute:

```python
import torch

print("PyTorch:", torch.__version__)
print("CUDA disponível:", torch.cuda.is_available())
print("CUDA runtime:", torch.version.cuda)

if torch.cuda.is_available():
    print("GPU:", torch.cuda.get_device_name(0))
    print("VRAM:", torch.cuda.get_device_properties(0).total_memory)
```

Exija `torch.cuda.is_available() == True` antes de tentar gerar modelos na GPU.

Se CUDA não funcionar, investigue a causa.

Não prossiga cegamente com inferência na CPU.

---

# 9. INSTALAÇÃO DAS DEPENDÊNCIAS OFICIAIS

Com o ambiente virtual ativado:

1. Inspecione `requirements.txt`.
2. Inspecione `setup.py`.
3. Identifique restrições de versões.
4. Verifique possíveis incompatibilidades com versões recentes de PyTorch, Transformers e Diffusers.
5. Instale dependências adequadas.
6. Instale o pacote local do projeto.
7. Valide imports.

Os comandos do README incluem:

```bash
pip install -r requirements.txt
pip install -e .
```

Utilize esses comandos somente dentro do ambiente virtual.

Se a resolução de dependências selecionar versões incompatíveis, não force uma instalação quebrada.

Corrija versões no ambiente isolado, com justificativa e documentação.

Evite instalar componentes necessários exclusivamente à geração de textura durante a primeira etapa.

---

# 10. NÃO INSTALAR TEXTURIZAÇÃO PESADA NO INÍCIO

A RTX 4050 possui aproximadamente 6 GiB de VRAM.

O README oficial informa requisitos muito maiores para o pipeline completo de forma e textura.

Portanto, inicialmente:

- Instale geração de geometria.
- Não carregue Hunyuan3D-Paint.
- Não carregue modelos text-to-image adicionais.
- Não ative geração multiview.
- Não compile extensões de texturização sem necessidade.

A interface oficial suporta:

`--disable_tex`

Essa opção deve ser utilizada na primeira execução.

A texturização poderá ser avaliada em uma etapa futura, após confirmar estabilidade.

---

# 11. MODELO INICIAL

Utilize o modelo oficial:

`tencent/Hunyuan3D-2mini`

Subpasta:

`hunyuan3d-dit-v2-mini`

Comece pela variante padrão mini, sem otimizações complexas adicionais.

A documentação oficial também lista:

- Mini Turbo.
- Mini Fast.
- Hunyuan3D-2 completo.
- Hunyuan3D-2 Turbo.
- Modelos multiview.

Não baixe todos os modelos antecipadamente.

Baixe apenas o necessário para o primeiro teste.

Se a variante padrão não couber em 6 GiB, investigue alternativas mais leves.

---

# 12. CONTROLE DE DOWNLOADS

Antes de baixar checkpoints:

1. Consulte tamanhos reais.
2. Verifique espaço disponível.
3. Configure cache no NVMe.
4. Evite duplicação dos pesos.
5. Preserve downloads concluídos.
6. Evite arquivos temporários grandes em RAM.

Configure variáveis de cache quando apropriado:

```bash
export HF_HOME="$HOME/AI-3D/cache/huggingface"
export TORCH_HOME="$HOME/AI-3D/cache/torch"
export TMPDIR="$HOME/AI-3D/cache/temporary"
```

Não execute downloads repetidos se os arquivos já estiverem íntegros.

Não copie tokens ou credenciais para logs.

Se o checkpoint exigir autenticação ou aceitação de termos, informe a necessidade.

Não tente contornar controles de acesso.

---

# 13. TESTE DE IMPORTAÇÃO

Antes de inferência, valide os imports essenciais.

Exemplo:

```python
import torch
import trimesh

from hy3dgen.shapegen import Hunyuan3DDiTFlowMatchingPipeline

print("CUDA:", torch.cuda.is_available())
print("Imports Hunyuan: OK")
```

Se houver erro:

- Capture traceback.
- Identifique dependência incompatível.
- Corrija dentro do ambiente isolado.
- Reexecute.

Não altere arquivos do sistema.

Não considere a instalação pronta apenas porque `pip install` terminou.

---

# 14. TESTE DE GPU E MEMÓRIA

Antes da primeira geração:

- Registre VRAM livre.
- Registre RAM livre.
- Registre swap.
- Verifique temperatura.
- Verifique processos na GPU.

Execute apenas uma geração por vez.

Não inicie Blender renderizando ao mesmo tempo.

Não execute duas instâncias do Hunyuan.

Não habilite batch de geração.

Se ocorrer OOM:

1. Registre a mensagem.
2. Encerre somente o processo criado nesta etapa.
3. Libere referências e recursos quando possível.
4. Não mate outros processos do usuário.
5. Não reinicie o sistema.
6. Não aumente swap automaticamente.
7. Não instale mais drivers.
8. Reavalie o modelo e os parâmetros.

Não entre em loops repetidos de geração com OOM.

---

# 15. PRIMEIRA GERAÇÃO DE MODELO 3D

Utilize preferencialmente uma imagem de demonstração incluída no repositório oficial ou uma imagem local autorizada pelo proprietário.

Não dependa de arquivos de conversa que não estejam presentes no computador.

Comece com parâmetros moderados.

Registre:

- Modelo.
- Checkpoint.
- Seed.
- Steps.
- Guidance scale.
- Resolução de extração de malha.
- VRAM inicial.
- Pico de VRAM.
- RAM.
- Tempo efetivamente medido.
- Dimensões do modelo exportado.

Não invente tempos de geração.

Não faça geração em lote.

Salve o resultado em formato GLB.

---

# 16. SCRIPT REPRODUZÍVEL

Crie um script Python de teste em:

`~/AI-3D/scripts/generate_shape.py`

O script deve:

1. Aceitar caminho de imagem.
2. Validar a existência da imagem.
3. Carregar o modelo configurado.
4. Permitir definir seed.
5. Permitir definir steps.
6. Gerar a malha.
7. Exportar GLB.
8. Registrar estatísticas.
9. Registrar tempo de geração.
10. Tratar erros CUDA/OOM.
11. Salvar logs.
12. Nunca sobrescrever resultados anteriores automaticamente.

Não deixe caminhos pessoais hardcoded.

Use `Path.home()` ou parâmetros de linha de comando.

O script precisa poder ser reutilizado futuramente.

---

# 17. INTERFACE GRADIO LOCAL

Depois de confirmar que a geração por Python funciona, configure a interface Gradio oficial.

Comando de referência:

```bash
python gradio_app.py \
  --model_path tencent/Hunyuan3D-2mini \
  --subfolder hunyuan3d-dit-v2-mini \
  --texgen_model_path tencent/Hunyuan3D-2 \
  --disable_tex \
  --low_vram_mode \
  --host 127.0.0.1 \
  --port 8080
```

Verifique os argumentos no código real da versão clonada.

A opção `--texgen_model_path` não deve provocar carregamento do modelo de texturas quando `--disable_tex` estiver ativo.

Não exponha a interface em `0.0.0.0`.

Não crie túnel público.

Não configure inicialização automática.

Não abra portas no firewall.

A interface deve ficar acessível somente localmente, por exemplo:

`http://127.0.0.1:8080`

Se a porta estiver ocupada, selecione outra porta local livre.

Não encerre serviços existentes.

---

# 18. PRESERVAÇÃO DOS RESULTADOS

A aplicação oficial pode utilizar diretórios de cache de resultados.

Não confie nesses diretórios como armazenamento permanente.

Crie um mecanismo simples para salvar os modelos aprovados em:

`~/AI-3D/outputs/geometry/`

Cada geração deve possuir:

- ID.
- Data.
- Nome.
- Imagem de entrada.
- Modelo utilizado.
- Seed.
- Parâmetros.
- Arquivo GLB.
- Estatísticas.
- Metadados.
- Miniatura, quando possível.

Não sobrescreva uma geração anterior.

Não exclua resultados automaticamente.

Se o Gradio utilizar cache temporário, mantenha cópias persistentes fora dele.

---

# 19. IMPORTAÇÃO NO BLENDER

O Blender 5.2.4 e o Blender MCP já estão configurados.

Não reinstale o MCP.

Depois de gerar o primeiro GLB:

1. Abra o Blender.
2. Crie uma cena de laboratório.
3. Importe o modelo.
4. Verifique orientação.
5. Verifique escala.
6. Verifique geometria.
7. Verifique normais.
8. Verifique materiais.
9. Verifique quantidade de vértices e faces.
10. Registre se a malha possui partes separáveis.

Salve uma cópia `.blend` no diretório de resultados.

Não altere os arquivos Blender do protagonista original.

Não considere a malha automaticamente pronta para animação.

---

# 20. INTEGRAÇÃO COM BLENDER MCP

Investigue como o Claude Code poderá controlar o Blender já configurado.

Objetivos futuros:

- Importar GLB.
- Ajustar escala.
- Inspecionar malha.
- Separar peças.
- Aplicar materiais.
- Criar sockets.
- Preparar rig.
- Exportar sprites.

Não crie automações complexas nesta fase.

O objetivo inicial é provar:

**Imagem → Hunyuan local → GLB → Blender.**

Não integre ao Bevy ainda.

---

# 21. SCRIPT DE INICIALIZAÇÃO

Crie um script reutilizável:

`~/AI-3D/scripts/start_hunyuan.sh`

Esse script deve:

- Verificar o ambiente Python.
- Verificar CUDA.
- Verificar VRAM.
- Configurar caches.
- Ativar o ambiente.
- Iniciar Gradio local.
- Utilizar o modelo escolhido.
- Desativar texturização.
- Registrar logs.
- Apresentar o endereço local.

Não utilize `sudo`.

Não execute em segundo plano automaticamente.

Não configure systemd.

Não inicie junto com o Ubuntu.

O usuário deve controlar quando o serviço inicia e termina.

---

# 22. TESTE DE ESTABILIDADE

Execute testes moderados:

A. Import do projeto.

B. Carregamento do modelo.

C. Geração de uma malha.

D. Exportação GLB.

E. Abertura no Blender.

F. Inicialização da interface Gradio.

G. Encerramento limpo da aplicação.

Não faça stress test prolongado.

Não execute geração contínua.

Se ocorrer pressão excessiva de memória ou temperatura, interrompa a etapa de inferência.

Documente qualquer limitação.

---

# 23. ALTERNATIVAS PARA VRAM INSUFICIENTE

Se o modelo mini não funcionar na configuração inicial, investigue:

1. Hunyuan3D-2mini-Fast.
2. Hunyuan3D-2mini-Turbo.
3. Redução de parâmetros de inferência.
4. Redução de resolução de extração de malha.
5. Gestão de memória.
6. Offload suportado.
7. Versões de dependências compatíveis.

Não presuma que `--low_vram_mode` reduz a VRAM da etapa de geração de geometria: inspecione sua implementação real.

Não ative FlashVDM sem validar a extensão e sua compatibilidade.

Não modifique drivers.

Não instale CUDA Toolkit global automaticamente.

Se nenhuma configuração funcionar, encerre a tentativa com diagnóstico completo, sem comprometer o sistema.

---

# 24. NÃO INSTALAR O MODELO COMPLETO SEM TESTE

O Hunyuan3D-2 completo usa mais recursos do que o mini.

Antes de tentar:

- Confirme sucesso do mini.
- Meça VRAM.
- Meça RAM.
- Verifique espaço.
- Estime o risco de OOM.
- Identifique estratégias compatíveis.

Não baixe o checkpoint completo por padrão.

Não execute o pipeline completo de texturas.

Não tente treinamento.

A RTX 4050 de 6 GiB deve ser tratada como ambiente experimental de memória limitada.

---

# 25. DOCUMENTAÇÃO FINAL

Crie:

`~/AI-3D/reports/RELATORIO_INSTALACAO.md`

Inclua:

A. Sistema operacional.

B. GPU e VRAM.

C. Driver NVIDIA.

D. Versão Python utilizada.

E. Versão PyTorch.

F. Versão CUDA runtime.

G. Commit do repositório Tencent.

H. Dependências instaladas.

I. Modelos baixados.

J. Tamanho dos checkpoints.

K. Estrutura de diretórios.

L. Teste de imports.

M. Teste de CUDA.

N. Geração de malha.

O. Exportação GLB.

P. Importação Blender.

Q. Interface Gradio.

R. Scripts criados.

S. Performance medida.

T. Problemas encontrados.

U. Correções aplicadas.

V. Limitações de VRAM.

W. Comandos para iniciar.

X. Comandos para encerrar.

Y. Localização dos modelos gerados.

Z. Próximas recomendações.

---

# 26. CRITÉRIOS DE ACEITAÇÃO

## Instalação

- [ ] Repositório oficial clonado.
- [ ] Python isolado.
- [ ] Ambiente virtual criado.
- [ ] PyTorch instalado.
- [ ] CUDA reconhecida.
- [ ] Dependências instaladas.
- [ ] Imports funcionando.

## Modelos

- [ ] Modelo mini baixado.
- [ ] Checkpoint carregado.
- [ ] Geometria gerada.
- [ ] GLB exportado.
- [ ] Estatísticas registradas.

## Interface

- [ ] Gradio iniciado.
- [ ] Interface acessível localmente.
- [ ] Sem exposição pública.
- [ ] Texturização desativada.
- [ ] Encerramento limpo.

## Blender

- [ ] GLB importado.
- [ ] Geometria inspecionada.
- [ ] Escala documentada.
- [ ] Cena salva separadamente.

## Segurança

- [ ] Python do sistema intacto.
- [ ] Driver NVIDIA intacto.
- [ ] Kernel intacto.
- [ ] Dark Castle RPG intacto.
- [ ] Blender original intacto.
- [ ] MCP preservado.
- [ ] Sem sudo.
- [ ] Sem commit.
- [ ] Sem push.

Classifique cada item como:

- Concluído.
- Parcial.
- Bloqueado.
- Não iniciado.

Não marque como concluído sem evidência.

---

# 27. PONTOS DE PARADA

Execute em ordem:

**Marco A — Auditoria:** validar sistema e recursos.

**Marco B — Repositório:** clonar e estudar requisitos.

**Marco C — Python:** preparar ambiente isolado.

**Marco D — PyTorch:** validar GPU.

**Marco E — Dependências:** instalar e testar imports.

**Marco F — Modelo:** baixar mini.

**Marco G — Inferência:** gerar uma malha.

**Marco H — Exportação:** salvar GLB.

**Marco I — Gradio:** iniciar interface local.

**Marco J — Blender:** importar resultado.

**Marco K — Documentação:** entregar relatório.

Não avance para geração de textura se a geometria ainda não funcionar.

Não avance para modelo completo se o mini apresentar OOM.

Não altere os drivers para tentar resolver incompatibilidades de Python.

---

# 28. RELATÓRIO FINAL NO TERMINAL

Depois de concluir, responda diretamente com:

1. Estado da instalação.
2. Versão de Python.
3. Versão de PyTorch.
4. CUDA detectada.
5. Modelo instalado.
6. VRAM utilizada.
7. Geração concluída ou bloqueada.
8. Caminho do GLB.
9. Estado do Gradio.
10. Endereço local.
11. Estado da importação no Blender.
12. Scripts de inicialização.
13. Diretório dos resultados.
14. Problemas e limitações.
15. Próxima etapa recomendada.

Inclua os comandos exatos para o proprietário iniciar novamente o Hunyuan3D depois de reiniciar o computador.

**Não faça commit nem push.**

**Não modifique o Dark Castle RPG.**

---

# 29. DIRETRIZ FINAL

Queremos que o Acer Nitro se torne uma pequena estação local de geração 3D.

O usuário deve conseguir:

1. Abrir o Hunyuan3D no navegador.
2. Enviar uma imagem.
3. Gerar um modelo 3D.
4. Salvar o modelo.
5. Abrir o modelo no Blender.
6. Utilizar Claude Code + Blender MCP para ajustes posteriores.

Priorize estabilidade, segurança, reprodutibilidade e preservação do sistema.

Não tente ativar todas as funcionalidades simultaneamente.

Não esconda falhas de VRAM.

Não substitua a instalação Python do Ubuntu.

**O sucesso da D14 é ter uma geração 3D local comprovadamente funcional, não apenas um ambiente Python instalado.**