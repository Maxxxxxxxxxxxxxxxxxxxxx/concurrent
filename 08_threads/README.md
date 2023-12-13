#### Zadanie 8. (czas: na 14.12.2022)

Napisać wielowątkowy program, który sumuje wszystkie elementy w (dużej) liście liczb całkowitych według jednego z poniższych schematów

1. tworzy dwa wątki współbieżne sumujące dwie połówki (plus minus jeden element) listy a po zakończeniu wątków sumuje uzyskane przez nie wyniki otrzymując sumę całości (punktacja: 5 punktów)

2. pozwala rozbić zadanie (iteracyjnie lub rekurencyjnie) na pewną ilość wątków. Ilość wątków regulowana w programie. (punktacja: 10 punktów)

###### \*Uwaga: W przypadku równoczesnego operowaniu na wspólnych zmiennych przez wątki należy pamiętać o wzajemnym wykluczaniu poprzez użycie blokady (Lock), ale też należy zadbać o to, aby jak najmniejszy fragment obliczeń był wykonywany na zasadzie wzajemnego wykluczania.
