__komunikacja przez pliki__ (czas: 2 zajęcia, czyli do 18.10.2023)

Utworzyć prostą parę programów klient - serwer, komunikujące się przez dwa pliki (plik dla danych i plik wyników), działając na zasadzie ciągłego odpytywania plików (w pętli aktywnego czekania).


    wczytanie                     
    ---------> |--------------| ---> |dane| ----> |--------------|
               |proces klienta|                   |proces serwera|
    <--------- |--------------| <-- |wyniki| <--- |--------------|
    wyświetlenie                   

Klient pobiera z klawiatury i zapisuje do pliku dane: pojedynczą liczbę całkowitą. Serwer pobiera daną z pliku, oblicza jakąś prostą funkcję arytmetyczną (np. nieduży wielomian) i wynik zapisuje do pliku wyniki . Klient odbiera odpowiedź z pliku, wyświetla i kończy działanie. Serwer działa nadal w pętli oczekując na kolejne zgłoszenia.

UWAGI:

1. Dowolny język programowania (bash, Python, C, ...)
2. Zakładamy, że tylko jest tylko  jeden klient w czasie każdej komunikacji (pomijamy przypadek wielu klientów działających równocześnie).
3. Przetestować kilkukrotne uruchomienie klienta dla tego samego serwera - może pojawić się konieczność opróżniania plików po stronie serwera i po stronie klienta zaraz po odczytaniu wiadomości z pliku.
4. Uruchamiać najpierw serwer, a potem dopiero klienta.
