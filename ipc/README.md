**Zadanie 6.** (czas: na 29.11.2023) Wykorzystując **pamięć współdzieloną** i synchronizację procesów za pomocą **semaforów** zaprogramować poniżej opisaną grę inspirowaną [grą w trzy karty](https://pl.wikipedia.org/wiki/Trzy_karty).

Jest dwóch graczy uruchamiających swoje programy w osobnych oknach: Gracz 1 i Gracz 2. Są dwa obszary pamięci współdzielonej PW1 i PW2. Gracz 1 to ten, który włączył swój program jako pierwszy. Są 3 tury gry przebiegające następująco:

1. Gracz 1 wybiera jedną spośród 3 liter: A, B lub C (wpisywane z klawiatury) i zapisuje ja do pamięci współdzielonej PW1.
2. Gracz 2 (nie znając wyboru Gracza 1) też wybiera jedną spośród 3 liter: A, B lub C (wpisywane z klawiatury) po czym zapisuje ja do pamięci współdzielonej PW2.
3. Następnie Gracz 1 odczytuje z pamięci PW2 wybór Gracza 2, a Gracz 2 odczytuje z pamięci PW1 wybór Gracza 1. Jeżeli obydwaj gracze wybrali te same litery to wygrywa Gracz 2, jeżeli litery są różne to wygrywa Gracz 1. U każdego z graczy powinna pojawić się w jego oknie informacja, jakie litery zostały wybrane i czy dany Gracz wygrał, czy przegrał aktualną turę gry i jaki jest wynik sumaryczny.
4. Po trzech opisanych wyżej turach gry gra się kończy (i pamięci współdzielone i semafory powinny zostać usunięte).

Synchronizacja procesów powinna zapewnić, że odczyty z pamięci współdzielonych (punkty 3 i 4) nastąpią dopiero po wcześniejszych zapisach (punkty 1 i 2). Ponadto przejście do następnej tury, czyli ponowne wykonanie punktów 1 i 2 powinno nastąpić dopiero po zakończeniu punktów 3 i 4 z poprzedniej tury. To powinno być zapewnione przy wykorzystaniu semaforów, ale bez aktywnego czekania. (Można np. wymusić wykonanie punktów 1, 2, 3, 4 w tej właśnie kolejności, ale może są i bardziej elastyczne rozwiązania?)

Można się wzorować na sposobie synchronizacji dwóch komunikujących się programów [kom1.py](https://inf.ug.edu.pl/~pmp/Z/Wspolb21P/kom1.py), [kom2.py](https://inf.ug.edu.pl/~pmp/Z/Wspolb21P/kom2.py) podanych jako [przykłady z wykładu](https://inf.ug.edu.pl/~pmp/Z/Wspolb21P/semPwP.html)

Dodatkowym wymaganiem jest, żeby zrobić **jeden** (uniwersalny) program do użytku obu graczy - wstępną rywalizację o bycie Graczem 1 wygrywa ten proces, który jako pierwszy utworzy któryś z semaforów albo pamięć współdzieloną. W programie [wyscig.py](https://inf.ug.edu.pl/~pmp/Z/Wspolb21P/wyscig.py) jest przykład ilustrujący jak to można zrobić przy użyciu flagi sysv_ipc.IPC_CREX. (Rozwiązanie zrobione nie jako jeden program ale jako osobne programy dla Gracza 1 i Gracza 2 będzie niżej ocenione.)

Wskazówki

- IPC w Pythonie: https://semanchuk.com/philip/sysv_ipc/
- materiały z wykładu: [pamięć wspólna i semafory](https://inf.ug.edu.pl/~pmp/Z/Wspolb21P/wspolb7pnotatki2022p.pdf)
