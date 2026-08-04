---
title: "El peso de la nube"
date: "2026-07-01"
description: "La llamamos «la nube» para no tener que imaginar las minas, las turbinas y el agua. Construí una herramienta para hacer visible ese peso."
---

La palabra «nube» es una de las piezas de marketing más logradas jamás escritas. Toma una nave del tamaño de un pueblo pequeño —repleta de servidores, consumiendo la energía de una ciudad mediana, refrigerada con millones de litros de agua— y la convierte en una forma blanca y suave que flota en algún lugar por encima de nosotros. Ingrávida. Limpia. En otra parte. No es asunto nuestro.

Todo el sentido de la metáfora es hacer desaparecer la materialidad. Y funciona. La mayoría escribimos un *prompt*, obtenemos una respuesta y nunca llegamos a imaginar el hecho físico que acabamos de desencadenar: electrones arrancados de una red eléctrica, calor volcado al aire, agua evaporada para llevarse ese calor. La interfaz no tiene fricción a propósito. La fricción es justo lo que nos haría pensar.

Llevo ya un tiempo sin poder dejar de pensarlo. Así que construí algo.

## ai-footprint

[ai-footprint](https://github.com/vinri2z/ai-footprint) es una pequeña herramienta que rastrea la huella de carbono *y* de agua de los agentes de IA para programar —Claude Code, Codex, Cursor, Gemini CLI, Copilot y una treintena más— en un solo lugar. Lee cuántos *tokens* has hecho pasar realmente por cada modelo, asigna a cada modelo unos factores de emisión por *token* extraídos de investigación revisada por pares, y convierte el total en gramos de CO₂ y mililitros de agua.

Lo central no es el informe. Es la línea de estado. Mientras trabajas, en silencio, al pie de la terminal, hay un contador en marcha:

```
🪵 65g CO₂ · 💦 800mL · $0.50
```

Eso es una sola sesión. Sesenta y cinco gramos de carbono. Ochocientos mililitros de agua —casi una botella— evaporados en algún sitio para refrigerar las máquinas que acaban de ayudarme a renombrar una función. El número sube mientras trabajo. Es imposible dejar de verlo una vez que está ahí.

Los factores de emisión provienen de [Jegham et al. (2025)](https://arxiv.org/abs/2505.09598), que midió el consumo energético de la inferencia de modelos de lenguaje en hardware real de AWS. Un modelo de clase Sonnet ronda los 1.140 gCO₂e por millón de *tokens* de salida; los modelos más grandes, varias veces eso. El agua se deriva de la misma energía mediante un factor de intensidad hídrica de 3,32 litros por kilovatio-hora: las torres de refrigeración y, sobre todo, el agua quemada al generar la electricidad de partida. Los números son estimaciones de orden de magnitud, no precisión de facturación, y lo digo bien alto en la [metodología](https://github.com/vinri2z/ai-footprint/blob/main/METHODOLOGY.md). Anthropic y las demás no publican sus cifras reales, así que una estimación independiente y honesta es lo mejor que cualquiera de nosotros puede hacer. Pero un orden de magnitud basta. Nunca se trató del tercer decimal. Se trataba de hacer visible un coste invisible lo suficiente como para *sentirlo*.

## La paradoja en el centro de todo

Aquí viene la parte incómoda, y la verdadera razón por la que creo que esto importa.

El instinto, una vez que ves el número, es volverse más eficiente. Usar un modelo más pequeño. Ajustar el *prompt*. Desperdiciar menos *tokens*. Todo bien. Pero hay una trampa esperando, y tiene casi dos siglos.

En 1865 el economista William Stanley Jevons observó algo extraño con el carbón. A medida que las máquinas de vapor se volvían más eficientes —a medida que extraían más trabajo de cada trozo de carbón—, Gran Bretaña no quemaba *menos* carbón. Quemaba muchísimo más. La eficiencia abarataba el uso del carbón, el uso más barato significaba más usos, y el consumo total se disparó. La ganancia por unidad quedaba engullida entera por el crecimiento en el número de unidades. Es la paradoja de Jevons, y se ha cumplido con lúgubre fiabilidad desde entonces.

La IA es Jevons en cámara rápida. Cada mejora de eficiencia —un modelo más barato, un chip más rápido, un agente más pulido— no reduce nuestra huella total. Abarata operar en el mundo numérico, así que operamos en él más. Muchísimo más. La fricción que antes limitaba cuánto producíamos, traducíamos, generábamos y computábamos se está disolviendo, y en ese espacio sin fricción irrumpe un apetito esencialmente ilimitado. No usamos la IA para hacer el mismo trabajo con menos. La usamos para hacer órdenes de magnitud más trabajo, y la huella trepa junto con la capacidad.

Ese es el mecanismo por el que la IA se convierte en una amenaza real para un planeta habitable: no porque una sola consulta sea cara —no lo es—, sino porque hace que el mundo numérico sea tan barato y rápido de operar que aceleramos directamente atravesando cualquier presupuesto que el mundo físico pueda permitirse. Cada barrera que antes nos frenaba estaba, en retrospectiva, protegiendo también la base de recursos. Quita las barreras y quitas los frenos.

## Por qué un número en una terminal

No creo que una línea de estado salve el planeta. Que quede claro.

Lo que hace es algo más pequeño y, espero, más honesto. Reconecta una acción con su consecuencia. Para quienes programan, hace que la abstracción tenga fugas a propósito: ya no puedes fingir que los *tokens* son gratis, porque los gramos están ahí mismo. Para todos los demás, es un argumento por demostración: la «nube» en la que vives tiene masa, bebe agua y quema carbón, y aquí tienes una manera de verlo suceder en tiempo real.

El consuelo de la metáfora de la nube es que nos deja tratar lo digital como si estuviera separado de lo físico: un segundo mundo, más limpio, superpuesto al sucio. No lo está. Hay un solo mundo, y el centro de datos está dentro de él, bebiendo de los mismos ríos y tirando de la misma red que todo lo demás. Lo más útil que puedo hacer como ingeniero es rechazar la metáfora. Devolverle el peso. Hacerlo visible, hacerlo local, hacerlo *mío*, y luego dejar que la gente decida, con el número delante, cuánto del mundo físico quiere gastar en el digital.

La nube siempre fue pesada. Solo que la construimos para que no lo pareciera.
