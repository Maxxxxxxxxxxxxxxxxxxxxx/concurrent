__Pliki blokujące (zamkowe), komunikacja przez pliki.__ (czas: 2 zajęcia, czyli do 26.10.2023)

Napisać parę programów klient-serwer składającą się na prosty komunikator tekstowy, działający zgodnie z poniższym opisem. (Do zrozumienia tego opisu pomocny może być następujący [szkic](https://inf.ug.edu.pl/~pmp/Z/Wspolb/PWszkicZad2.pdf).)

Serwer działa w pętli nieskończonej, klient tylko jednorazowo (pytanie - odpowiedź). Program klienta otrzymuje nazwę pliku z danymi wejściowymi dla serwera (na rysunku nazywa się to "bufor serwera") jako parametr wywołania lub ustala to konwersacyjnie lub po prostu wpisany jest na sztywno w kod (tylko nazwa pliku, jeżeli klienta i serwera uruchamiamy w tym samym katalogu, w przeciwnym razie trzeba podać całą ścieżkę do pliku). Serwer jest jeden, __klientów może być wielu, działających równocześnie__, np. z różnych okien.

Klient przesyła do bufora serwera:

- nazwę swojego pliku (ewentualnie całą ścieżkę) do którego ma być  przesłana odpowiedź (każdy klient ma swój osobny plik). Ta nazwa może być np. ustalana konwersacyjnie bądź parametrem wywołania klienta.
- kolejne linie swojego tekstu wprowadzonego przez użytkownika (dowolnie dużo linii);
- jakiś znacznik końca tekstu (np. kod znaku Esc).

Serwer (działający w innym oknie) czyta z pliku "bufor serwera" i wyświetla cały tekst  klienta po czym odsyła klientowi do pliku, którego nazwę otrzymał w pierwszej linii komunikatu od klienta:
- kolejne linie swojego tekstu (wprowadzane ręcznie przez użytkownika serwera);
- jakiś znacznik końca tekstu. 

Dostęp do bufora serwera powinien być __synchronizowany plikiem zamkowym__ (lockfile) tworzonym przez klienta w katalogu roboczym serwera i usuwanym przez serwer po udzieleniu odpowiedzi klientowi. Jeżeli serwer czasowo nie jest dostępny, klient wyświetla w pętli co kilka sekund informację: "Serwer zajęty, proszę czekać".  
Po udzieleniu odpowiedzi serwer usuwa utworzony przez klienta plik lockfile.

UWAGI

- Żeby przetestować sytuację, że serwer może być zajęty, należy utworzyć dwóch klientów: jeden wysyła do serwera dane, a drugi, jeszcze przed odpowiedzią serwera do pierwszego też próbuje się skomunikować z serwerem.
- Dopuszczalnym uproszczeniem jest przyjęcie, że odpowiedź serwera jest generowana automatycznie zamiast ręcznego wpisywania w oknie serwera
- Niedopuszczalnym błędem jest rozdzielenie operacji sprawdzenie czy plik zamkowy istnieje i utworzenie go. Poprawne niepodzielnie wykonanie tych operacji jest opisane we wskazówkach poniżej.

WSKAZÓWKI dotyczące używania plików zamkowych (plików blokujących, lockfile)

- konstrukcja w C umożliwiająca wykonanie niepodzielnej operacji utworzenia pliku zamkowego pod warunkiem, że go nie ma:
    ```c
    while (open("ścieżka_dostępu/lockfile",O_CREAT|O_EXCL,0)==-1) {
  
        printf("Serwer zajety, prosze czekac\n");
        sleep( ... );
    }
  ```
- rozwiązania Pythonowe omawiane na wykładzie:
  - [przykład z wykładu](https://inf.ug.edu.pl/~pmp/Z/Wspolb/lockf2.py), [wykład o plikach blokujących](https://inf.ug.edu.pl/~pmp/Z/Wspolb21P/wspolb2p.pdf)
