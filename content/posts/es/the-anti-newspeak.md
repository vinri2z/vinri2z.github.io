---
title: "La anti-neolengua"
date: "2026-06-26"
description: "Orwell imaginó un idioma construido para encoger el pensamiento. Acabamos de construir uno que hace lo contrario - para las máquinas."
---

En el apéndice de *1984*, Orwell expone la gramática de su distopía. No las telepantallas, no el Ministerio de la Verdad: el idioma. La neolengua era el instrumento de control más ambicioso del régimen, y su lógica era escalofriantemente simple: si eliminas las palabras para una cosa, eliminas la capacidad de pensarla. Despoja a "libertad" de todo significado salvo el trivial ("el perro está libre de pulgas") y la libertad política se vuelve, a lo largo de generaciones, literalmente impensable. No habría vocabulario en el que enmarcar la herejía. El crimen del pensamiento no se castigaría. Sería imposible.

La premisa que hay debajo es más antigua que Orwell y nunca ha llegado a desaparecer: *los límites de tu lenguaje son los límites de tu mundo.* Lo que no puedes decir, te cuesta pensarlo. Estrecha lo expresable y estrecharás lo concebible.

Durante casi toda la historia de la informática, el software ha vivido bajo su propia neolengua, y la prisión la construimos nosotros mismos.

## La máquina que solo sabía hablar con palabras exactas

Piensa en lo que ha significado, hasta hace muy poco, que dos sistemas se hablen entre sí.

Cada integración era un tratado negociado de antemano. Este servicio emite un campo llamado `customer_id`; aquel espera `customerId`. Uno envía una fecha como `2026-06-25`, el otro exige una marca de tiempo Unix. El contrato es rígido, total e implacable. Una sola carga mal formada - una coma de más, un corchete que falta, una cadena donde se prometió un número - y todo el intercambio se derrumba. No se degrada. Se derrumba. El sistema receptor no *interpreta* el mensaje confuso y se encoge de hombros; lanza una excepción y se detiene.

Esto es el software como neolengua. La máquina solo sabe pensar con las palabras exactas que se le entregaron. Cualquier cosa fuera de su vocabulario declarado no es solo desconocida: es *extranjera*, y extranjera significa fatal. Un objeto JSON que no coincide con el esquema no es un intento de comunicación ligeramente desviado. Es ruido. El sistema no tiene noción de "lo que probablemente quisiste decir", porque el significado nunca fue el objetivo. Solo lo era la conformidad.

Pasamos décadas construyendo aparatos elaborados para lidiar con esta rigidez: validadores de esquemas, adaptadores, middleware, toda la industria trituradora del ETL. Todo ello, al final, trabajo de traducción: esfuerzo gastado en forzar la expresión desordenada del mundo a encajar de nuevo en un vocabulario lo bastante estrecho como para que una máquina lo aceptara. El problema de la integración era, en el fondo, un problema de *lenguaje*. Dos sistemas que no hablaban el mismo dialecto sencillamente no podían conversar, y ninguna cantidad de buena voluntad cerraba la brecha. Alguien tenía que escribir el diccionario, a mano, de antemano.

## Romper la barrera

Entonces algo se invirtió.

La llegada de los grandes modelos de lenguaje - y, crucialmente, de protocolos como MCP que les permiten extender la mano y *actuar* - rompió la suposición más antigua de la pila: que una máquina solo puede procesar lo que coincide exactamente con su gramática declarada.

Ahora un objeto JSON mal formado no es el final de la conversación. Es el principio de una. Un modelo lo lee, comprende la *intención* detrás de la sintaxis rota y la repara. La coma de más, el campo con el nombre equivocado, el valor con una forma inesperada: dejan de ser fatales. El mensaje "extranjero" es interpretado y arreglado por otra máquina, al vuelo, igual que un humano fluido lee por encima de una errata sin perder el paso.

Eso es un auténtico cambio de paradigma, no una biblioteca nueva e ingeniosa. Por primera vez, el software tiene algo parecido a la *comprensión* situado entre los sistemas: una capa que comercia con el significado en lugar de con las coincidencias exactas. MCP le da manos a esa comprensión: puede llamar a la herramienta, consultar la fuente, arreglar la carga y enrutar el resultado, todo ello tolerando la imperfección que antes era letal.

Y aquí está la parte que merece detenerse a pensar. La flexibilidad y la fiabilidad solían ser una disyuntiva. Podías tener un sistema rígido y de fiar, o uno laxo y frágil, y el oficio de la ingeniería consistía en elegir dónde plantarse en esa línea. El nuevo arreglo derrumba la disyuntiva. El software ahora puede *doblarse sin romperse* - absorber lo inesperado, interpretar lo mal formado, tender un puente entre dos vocabularios que nunca se presentaron - y aun así llegar a un resultado fiable.

## Orwell, invertido

Aquí es donde el paralelismo gira, y gira con fuerza.

La pesadilla de Orwell era un lenguaje deliberadamente *estrechado* para encoger el espacio del pensamiento: control logrado por amputación, el mundo hecho más pequeño una palabra borrada cada vez. Lo que hemos construido para las máquinas pasa la película al revés. Estamos *ensanchando* lo expresable. El vocabulario que un sistema puede aceptar ya no es la jaula en la que vive. Una máquina ya no está atrapada por las palabras exactas que se le dieron; puede toparse con lo extranjero, lo parcial, lo mal formado, y darle sentido de todas formas.

Si los límites de tu lenguaje son los límites de tu mundo, entonces acabamos de entregarles a nuestras máquinas un mundo mucho más grande en el que vivir. La neolengua encogió lo decible para controlar lo pensable. Esto hace lo contrario: expande lo que se le puede decir *a* un sistema - y por tanto lo que un sistema puede hacer - sin renunciar a la fiabilidad que hizo del software algo digno de confianza en primer lugar.

Orwell advirtió que quien controla el lenguaje controla el pensamiento. Tenía razón. La pregunta interesante ahora es qué ocurre cuando aquello que controla el lenguaje ya no intenta hacernos pensar *menos* - sino que está, por una vez, construido para entender *más*.
