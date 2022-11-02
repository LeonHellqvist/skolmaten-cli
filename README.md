<div id="top"></div>
<div align="center">

<h1 align="center">Skolmaten-cli</h1>

  <h4 align="center">
    Se matsedeln direkt från terminalen
  </h4>
</div>

<!-- KOMMA IGÅNG -->
## Hur man använder

För att välja din matsal söker du med hjälp av
```sh
skolmaten-cli sök <namn på matsal>
```
Då kommer du få upp en lista av alla matsalar som matchar sin sökning.
Välj sedan din matsal i listan.

Alternativt om du vet din matsals id kan du köra följande kommand
```sh
skolmaten-cli id <id>
```
Efter det är det bara att köra detta för att få fram matsedeln
```sh
skolmaten-cli
```
Alternativt kör detta för att se specifik vecka
```sh
skolmaten-cli vecka <veckonummer>
```


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
   git clone https://github.com/LeonHellqvist/skolmaten-cli.git
   ```
2. Gå in i projektmappen
   ```sh
   cd skolmaten-cli
   ```
3. Kompilera release build
   ```sh
   cargo b -r
   ```
4. Installera (frivilligt)
   ```sh
   cargo install --path .
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
