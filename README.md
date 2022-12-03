# Æ++
Programmeringsspråk på norsk

# todo
- [x] Reworked runtime and execution
- [x] rework strings and memory
- [x] Add memory read and write
- [x] Add string manipulation
- [x] pointer manipulation
- [x] Add type casting
- [x] Add rest of stack operators
- [x] Add constants
- [x] Add Else-if
- [x] Add functions
- [x] make printing better
- [x] scoping
- [x] syntax sugar
- [ ] drop memory
- [ ] implement pointer struct for memory safety
- [ ] structure
- [ ] var
- [ ] local mem
- [ ] Add import
- [ ] Make better error system
- [ ] Make better docs

# Hvordan bruke
## Windows
Du trenger [git](https://git-scm.com/) og [rust](https://www.rust-lang.org/) installert først
```
git clone https://github.com/kegma1/ae-plus-plus.git
cd .\ae-plus-plus
python.exe .\install.py
.\aepp.exe <-Flagg> [./Sti]
```

## Flagg
Flagg er frivillig.
| Flagg |Beskrivelse|
|:---------:|:------------|
|-d|Debug flagg, vil skrive ut stabelen når programmet kræsjer.|

# Referanse

## Stack
Æ++ er et stable basert programmeringsspråk. Det vil si at programmert fungerer veldig annerledes enn i konvensjonell språk.
I Æ++ vill nesten alt av operasjoner fungere på top elementene på stabelen. Under er et simpel program som plusser to Int sammen, og hvordan hver operasjon endrer stabelen
```
35 34 +
-------------------------------------------------------
35
----------------
|35|  |  |  |      <- dytter 35 til stabelen
----------------

34
----------------
|35|34|  |  |      <- dytter 34 til stabelen
----------------

+
----------------
|  |  |  |  |      -> poper de to øverste tallene på stabelen
----------------

+
----------------
|69|  |  |  |      <- dytter summen på stabelen
----------------
```

## Operasjoner

### Matte operasjoner
| Operasjon |C-ekvivalent|
|:---------:|:------------|
|+|`a b + -> a + b`|
|-|`a b - -> a - b`|
|*|`a b * -> a * b`|
|/|`a b / -> a % b, a / b`|
#
### Logiske operasjoner
| Operasjon |C-ekvivalent|
|:---------:|:------------|
|ikke|`a ikke -> !a`|
|og|`a b og -> a && b`|
|eller|`a b eller -> a \|\| b`|
#
### likhets operasjoner
| Operasjon |C-ekvivalent|
|:---------:|:------------|
|=|`a b = -> a == b`|
|<|`a b < -> a < b`|
|>|`a b > -> a > b`|
|<=|`a b <= -> a <= b`|
|>=|`a b >= -> a >= b`|
#
### Stabel operasjoner
| Operasjon |Beskrivelse|
|:---------:|:------------|
|dup|`a -> a a`|
|rot|`a b c -> b c a`|
|slipp|`a b -> a`|
|snu|`a b -> b a`|
|over|`a b -> a b a`|
#
### typer
| navn |Beskrivelse|
|:---------:|:------------|
|Helt|`i32`|
|Flyt|`f32`|
|bool|`bool`|
|Str|`string`|
|Pek|`Ptr`|
|Bokst|`Char`|
#
### omgjør
```
  "69"   Int omgjør
# ^verdi ^type
skriv # vil skrive ut 69 som et Int
```
#
### konst nøkkelord
```
konst x 35 34 + slutt

x skriv # dette vil skrive ut 69
```
#
### minne nøkkelord
når man definerer et minne trenger man i rekkefølge et navn, en type, og en lengde på hvor stor buffer vi skal dekke.
når du skriver minne navnet vil den dytte en peker til det første elementet i bufferen, du kan lagre data ved '.' operatoren og lese data med ','.
du kan velge andre elementer i bufferen med '+' eller '-'. foreksempel hvis x har en lengde på 10 og peker til adresse 20 vil denne koden `x 5 +` skape en peker som peker til adresse 25.
```
minne x Helt 3 slutt
1 x ->
2 x 1 + ->
3 x 2 + ->
x 1 + @ skrivnl # skriver ut 2

```
minnet vil se ut som: |1|2|3| | | | |...
#
### hvis og ellers
```
"skriv et tall: " spør Int omgjør
hvis 10 > gjør
    "større enn 10"
ellers
    "mindre enn 10"
slutt
skrivnl
``` 
#
### ellvis
```
"skriv et tall: " spør Int omgjør
hvis dup 10 > gjør
    "større enn 10"
ellvis dup 5 = gjør
    "tallet er 5"
ellers
    "mindre enn 10"
slutt
skrivnl
``` 
#
### når løkker
```
0 når dup 15 <= gjør
    dup skrivnl
    1 +
slutt
# skriver ut alle tallene fra 0 til 15
```
#
### let bindinger
```
1 2 3
let x y z inni
    x z +
    y -
slutt
skrivnl # skriver ut 2
```
#
### funksjoner
før du kaller en funksjon må du passe på at du har alle argumentene i rett rekkefølge. 
Når funksjonen blir utført vil du bare ha tilgang til de verdiene som ble gitt inn når den ble kallet, når funksjonen er kommet til slutten vil den dytte retur verdien til toppen av forrige stabel.
```
# funk <navn> <argument typer> -- <retur type> inni
funk sum Helt Helt -- Helt inni
    x
slutt
2 2 sum skrivnl # skriver-ut 4
```
#
## Streng manipulasjon
En streng er i bunn og grunn en peker til en bokstav buffer. dette vil si at hvis man ønsker å endre på en streng kan man omgjøre streng-pekeren til en standard peker ved hjelp av slik
```
"hallo\n" Pek omgjør
dup "m" snu -> 1 +
dup "o" snu -> 1 +
dup "r" snu -> 1 +
dup "d" snu -> 1 +
dup "i" snu -> 1 +
5 - Str omgjør 
skrivnl # skriver ut mordi
```

