**Zadanie 3.** (czas: 2 zajęcia, czyli do 8.11.2023)

Pewien tekst jest zapisany w kilku plikach w ten sposób, że zaczyna się w jednym ustalonym pliku a w treści tego pliku i również w pozostałych plikach mogą się pojawiać dyrektywy postaci **\input{filename}** włączające w miejsce tej dyrektywy tekst z pliku **filename**, przy czym dyrektywy takie mogą się pojawiać we wszystkich plikach. Zatem przeglądanie pliku **plikA.txt** o zawartości

**Stoi na stacji lokomotywa,  
\input{plikB.txt}  
Wagony do niej podoczepiali,  
\input{plikD.txt}  
A tych wagonów jest ze czterdzieści.**

gdzie plik **plikB.txt** ma zawartość:

**ciężka ogromna i pot z niej spływa.  
\input{plikC.txt}  
Żar z rozgrzanego jej brzucha bucha**

plik **plikC.txt** ma zawartość:  
**Stoi i sapie, dyszy i dmucha,**

plik **plikD.txt** ma zawartość:  
**ciężkie ogromne, z żelaza i stali.**

daje ostatecznie tekst

**Stoi na stacji lokomotywa,  
ciężka ogromna i pot z niej spływa.  
Stoi i sapie, dyszy i dmucha,  
Żar z rozgrzanego jej brzucha bucha  
Wagony do niej podoczepiali,  
ciężkie ogromne, z żelaza i stali.  
A tych wagonów jest ze czterdzieści.**

Napisać program, który otrzymuje dwa parametry:

- p - nazwę pliku z początkiem tekstu
- s - jakieś słowo

i wypisuje ilość wystąpień słowa s w tekście zaczynającym się w pliku p, z uwzględnieniem wszystkich dyrektyw **\input**, również zagnieżdżonych we włączanych plikach (możemy założyć, że dyrektywy te nie generują cykli, oraz przyjąć, że występują jako osobne linie w tekście).

Wymagane jest, aby przeglądanie każdego pliku z dyrektywy **\input** odbywało się w nowym, odgałęzionym procesie. Do przekazania informacji zwrotnej od procesów potomnych do rodzicielskich można wykorzystać mechanizm **exit-wait** omawiany na wykładzie. Uwaga: proces rodzicielski powinien rozpocząć oczekiwanie na zakończenie procesów potomnych (**wait**) dopiero po przejrzeniu całego swojego pliku.

Podsumowanie ważnych wymagań:

- dyrektywy **\input** mogą pojawiać się wielokrotnie w każdym tekście i pojawiać również we włączanych plikach (przetestować na lepszym przykładzie niż ten powyżej)
- obsługa dyrektyw **\input** powinna odbywać się w rozgałęzionych procesach potomnych
- proces rodzicielski powinien rozpocząć oczekiwanie na zakończenie procesów potomnych (**wait**) dopiero po przejrzeniu całego swojego pliku a nie natychmiast po rozgałęzieniu procesu potomnego.

Wskazówki:

- materiały z wykładu: [Tworzenie procesów i prymitywne sposoby ich synchronizacji: fork,exit,wait.](https://inf.ug.edu.pl/~pmp/Z/Wspolb21P/wspolb3P.pdf)
- przykłady z wykładu: [fork.py](https://inf.ug.edu.pl/~pmp/Z/Wspolb21P/fork.py), [forkwait.py](https://inf.ug.edu.pl/~pmp/Z/Wspolb21P/forkwait.py), [forkwaitP.py](https://inf.ug.edu.pl/~pmp/Z/Wspolb21P/forkwaitP.py), [forkWyr.py](https://inf.ug.edu.pl/~pmp/Z/Wspolb21P/forkWyr.py)
- dokumentacja Pythona https://docs.python.org/3/library/os.html

