<div id="top"></div>
<div align="center">

<h3 align="center">Skolmaten-cli</h3>

  <p align="center">
    <b>Se matsedeln direkt från terminalen</b>
  </p>
</div>



<!-- KOMMA IGÅNG -->
## Komma igång

Det här är ett exempel på hur du kan sätta upp projektet. Följ bara dessa instruktioner för att få igång en lokal kopia.

### Förutsättningar

Om du inte redan har Rust så måste du installera det först
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Installation

1. Klona den här repon
   ```sh
   git clone https://github.com/LeonHellqvist/unikum-tools.git
   ```
2. Gå in i projektmappen
   ```sh
   cd skolmaten-cli
   ```
3. Kompilera release build
   ```sh
   cargo b -r
   ```
4. Flytta binary till din lokala binary mapp
   ```sh
   sudo mv target/release/skolmaten-cli /usr/local/bin/
   ```

<p align="right">(<a href="#top">Tillbaka till toppen</a>)</p>


<!-- BIDRA -->
## Bidra

Bidrag är det som gör communityn med öppen källkod så bra. Alla bidrag du gör är **mycket uppskattade**.

Om du har en idé som skulle göra det här bättre, forka repon och skapa en pull request. Du kan också bara öppna en issue med taggen "förbättring".
Glöm inte att ge projektet en stjärna, tack så mycket!

1. Forka projektet
2. Gör dina ändringar och bidrag
3. Öppna en pull request

<p align="right">(<a href="#top">Tillbaka till toppen</a>)</p>
