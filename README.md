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