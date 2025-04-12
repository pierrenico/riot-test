Le premier passage sur la signature a fait une conversion en string (contrairement aux reqs).
Il avait pas pensé au fait qu'en JSON on peut avoir plusieurs clés avec le même nom, ce qui est tricky pour l'ordre.
Il s'est un peu perdu sur les valeurs infinies en IEEE 754 vs la spec JSON.