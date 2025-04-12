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

### Critiques

> serde_json, the widely used Rust library for handling JSON data, generally adheres closely to the JSON specification (RFC 8259). However, there are a few notable areas where its default behavior or capabilities might differ or extend the standard:
> Trailing Commas: By default, serde_json is more lenient than the strict JSON specification and allows trailing commas in arrays and objects (e.g., [1, 2, ] or {"a": 1, }). The official spec forbids these.
> Duplicate Keys in Objects: The JSON spec states that object key names should be unique but doesn't strictly define the behavior if they aren't. When parsing JSON with duplicate keys, serde_json's default behavior is to keep the last value associated with that key.
> Non-String Keys (Serialization): While JSON requires object keys to be strings, serde itself is more general. When serializing Rust types like HashMap where keys might not be strings (e.g., numbers), serde_json will usually error unless the key type has a natural string representation or custom serialization logic is provided.
> Extended Number Literals (Historically/Potentially): The JSON spec does not include Infinity, -Infinity, or NaN as valid number values. While standard serde_json parsing typically rejects these, there have been features or configurations in the past, or potentially in custom deserializers, that might allow them. However, relying on this is non-standard.
> Comments: The official JSON spec does not support comments. serde_json itself doesn't parse comments in its standard from_str or from_value functions. However, the ecosystem might have crates or methods that do strip comments before parsing, making it seem like serde_json supports them.
> For most standard use cases, serde_json provides a compliant and robust way to work with JSON data. The allowance of trailing commas is the most common deviation developers encounter in practice.