# NetSnitch 

Esse é o repositório que contém o código do protótipo desenvolvido durante
o App Challenge de 2023, um evento promovido pelo Inatel, cujo tema deste ano
foi: **Criar um Medidor de tráfego de Rede".

## Como executar o projeto localmente?

Você precisa ter o `node` e `python` instalados em sua máquina. Também é
necessário que você tenha clonado o repositório do sniffer desenvolvido
pela organização do evento. Partindo desse ponto, execute os seguintes passos:

- Clone o repositório em sua máquina local.
- Execute o [sniffer](https://github.com/Viasat/Viasat-NetworkTrafficMeter) antes de executar a aplicação.
- Ao entrar na pasta do NetSnitch, execute os seguintes comandos:

```bash
npm install
npm run tauri dev
```

**Observação**: Por padrão, em ambiente de desenvolvimento, a aplicação também abrirá as
Devtools, para fins de diagnóstico durante o processo de desenvolvimento. É só clicar no
botão de fechar no canto superior esquerdo para se livrar dessa janela! 
