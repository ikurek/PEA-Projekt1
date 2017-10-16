# PEA - Projekt 1
## Implementacja algorytmów programowania dynamicznego i przeglądu zupełnego do rozwiązania problemu komiwojażera

### Zasada działania programowania dynamicznego:
Załóżmy, że graf reprezentujący 4 miasta, pomiędzy którymi porusza się komiwojażer opisany jest w następujący sposób:

| \ 	|  **0** 	|  **1** 	|  **2** 	|  **3** 	|
|:-:	|:--:	|:--:	|:--:	|:--:	|
| **0** 	| -1 	| 10 	| 15 	| 20 	|
| **1** 	|  5 	| -1 	|  9 	| 10 	|
| **2** 	|  6 	| 13 	| -1 	| 12 	|
| **3** 	|  8 	|  8 	|  9 	| -1 	|

Wartości w macierzy odpowiadają kosztowi trasy pomiędzy dwoma wierzchołkami, przedstawionymi w nagłówkach tabeli, przykładowo:

* Trasa pomiędzy wierzchołkiem 1 i 2 ma koszt 13
* Trasa pomiędzy wierzchołkiem 0 i 3 ma koszt 8
* Itd...

Wartość -1 występuje w momencie, w którym próbujemy sprawdzić trasę pomiędzy tym samym wierchołkiem, jest to bezpieczna opcja, ponieważ w tym problemi koszt trasy musi być większy od zera.

Przyjmijmy, że naszym wieszchołkiem startowym będzie wierzchołek przedstawiony w tabeli jako 0. Komiwojażer, może z niego przejść na dowolny z pozostałych wierzchołków: 1, 2 lub 3. Sprawdźmy więc, jaki kosz będzie miała każda z podróży:

* _koszt(0 -> 1)_ = 5
* _koszt(0 -> 2)_ = 6
* _koszt(0 -> 3)_ = 8

Dla każdej z możliwych dróg, rozpatrzmy możliwość dalszego ruchu.

* _koszt(0 -> 1)_ = 5
    * _koszt(0 -> 1 -> 2)_ = _koszt(0 -> 1)_ + _koszt(1 -> 2)_ = 5 + 13 = 18
    * _koszt(0 -> 1 -> 3)_ = _koszt(0 -> 1)_ + _koszt(1 -> 3)_ = 5 + 8 = 13
* _koszt(0 -> 2)_ = 6
    * _koszt(0 -> 2 -> 1)_ = _koszt(0 -> 2)_ + _koszt(2 -> 1)_ = 6 + 9 = 15
    * _koszt(0 -> 2 -> 3)_ = _koszt(0 -> 2)_ + _koszt(2 -> 3)_ = 6 + 9 = 15
* _koszt(0 -> 3)_ = 8
    * _koszt(0 -> 3 -> 1)_ = _koszt(0 -> 3)_ + _koszt(3 -> 1)_ = 8 + 10 = 18
    * _koszt(0 -> 3 -> 2)_ = _koszt(0 -> 3)_ + _koszt(3 -> 2)_ = 8 + 12 = 20

Na tym etapie można już zauważyć, że wartości kolejnych sprawdzanych dróg, zależne są od poprzednich. Problem, którego rozwiązanie próbujemy znaleźć da się więc rozwiązać, znajdując rozwiązanie opisującej go funkcji rekurencyjnej. Zobaczmy jak wygląda kolejny krok, przyjmując minimalne wartości z poprzednich: