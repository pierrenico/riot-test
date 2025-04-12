Tout fait avec Cursor, Sonnet et Gemini. Je n'ai pas écrit une seule ligne de code.

### Quelques notes
- Le premier passage sur la signature a fait une conversion en string (contrairement aux reqs)
- Il avait pas pensé au fait qu'en JSON on peut avoir plusieurs clés avec le même nom (SHOULD, pas MUST), ce qui est tricky pour l'ordre; j'ai testé d'en tenir compte, puis décidé d'enjamber parce que serde le gère mal; j'y reviendrai peut-être à la fin
- Il s'est un peu perdu sur les valeurs infinies en IEEE 754 vs la spec JSON (JSON fait IEEE 754 *sauf sur ça*)

### Les résultats des benchs

> The benchmarks for the core cryptographic functions completed successfully!
> Here's a summary of the results:
> encrypt_data: ~158 ns
> decrypt_data: ~148 ns
> sign_data: ~785 ns
> verify_signature: ~772 ns

> The endpoint benchmarks also completed successfully!
> Here's a summary:
> /encrypt: ~4.9 µs
> /decrypt: ~4.7 µs
> /sign: ~6.2 µs
> /verify: ~5.9 µs