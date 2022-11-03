# Æ++
Programmeringsspråk på norsk

# todo
- [x] Reworked runtime and execution
- [ ] rework strings and memory
- [ ] Add memory read and write
- [ ] Add string manipulation
- [ ] f strings
- [x] Add type casting
- [x] Add rest of stack operators
- [x] Add constants
- [ ] Add functions
- [ ] Add import
- [ ] Make better error system

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
I Æ++ vill nesten alt av operasjoner fungere på top elementene på stabelen. Under er et simpel program som plusser to heltall sammen, og hvordan hver operasjon endrer stabelen
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
|heltall|`i32`|
|flyttall|`f32`|
|bool|`bool`|
|streng|`string`|
#
### omgjør
```
  "69"   heltall omgjør
# ^verdi ^type
skriv-ut # vil skrive ut 69 som et heltall
```
#
### konst nøkkelord
```
konst x 35 34 + slutt

x skriv-ut # dette vil skrive ut 69
```
#
### hvis og ellers
```
"skriv et tall: " spør heltall omgjør
10 > hvis
    "større enn 10"
ellers
    "mindre enn 10"
slutt
skriv-ut
``` 
#
### mens løkker
```
0 mens dup 15 <= gjør
    dup skriv-ut
    1 +
slutt
# skriver ut alle tallene fra 0 til 15
```
