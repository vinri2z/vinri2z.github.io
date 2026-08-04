---
title: "Le poids du nuage"
date: "2026-07-01"
description: "On l'appelle « le cloud » pour ne pas avoir à imaginer les mines, les turbines et l'eau. J'ai construit un outil pour rendre ce poids visible."
---

Le mot « nuage » est l'un des coups de marketing les plus réussis jamais écrits. Il prend un entrepôt de la taille d'une petite ville — bourré de serveurs, tirant l'électricité d'une ville moyenne, refroidi par des millions de litres d'eau — et le transforme en une forme blanche et douce qui flotte quelque part au-dessus de nous. Sans poids. Propre. Ailleurs. Pas notre problème.

Tout l'intérêt de la métaphore est de faire disparaître la matérialité. Et ça marche. La plupart d'entre nous tapons un *prompt*, obtenons une réponse, et n'imaginons jamais l'événement physique que nous venons de déclencher : des électrons arrachés à un réseau, de la chaleur déversée dans l'air, de l'eau évaporée pour évacuer cette chaleur. L'interface est sans friction, exprès. La friction, c'est précisément ce qui nous ferait réfléchir.

Cela fait un moment que je n'arrive plus à cesser d'y penser. Alors j'ai construit quelque chose.

## ai-footprint

[ai-footprint](https://github.com/vinri2z/ai-footprint) est un petit outil qui mesure l'empreinte carbone *et* eau des agents d'IA de programmation — Claude Code, Codex, Cursor, Gemini CLI, Copilot et une trentaine d'autres — au même endroit. Il lit combien de *tokens* vous avez réellement fait passer par chaque modèle, associe à chaque modèle des facteurs d'émission par *token* tirés de recherches évaluées par des pairs, et convertit le total en grammes de CO₂ et millilitres d'eau.

L'essentiel n'est pas le rapport. C'est la ligne d'état. Pendant que vous travaillez, discrètement, en bas du terminal, un compteur tourne :

```
🪵 65g CO₂ · 💦 800mL · $0.50
```

Ça, c'est une seule session. Soixante-cinq grammes de carbone. Huit cents millilitres d'eau — presque une bouteille — évaporés quelque part pour refroidir les machines qui viennent de m'aider à renommer une fonction. Le nombre monte à mesure que je travaille. Impossible de ne plus le voir une fois qu'il est là.

Les facteurs d'émission viennent de [Jegham et al. (2025)](https://arxiv.org/abs/2505.09598), qui a mesuré la consommation énergétique de l'inférence des modèles de langage sur du matériel AWS réel. Un modèle de classe Sonnet tourne autour de 1 140 gCO₂e par million de *tokens* de sortie ; les modèles plus gros, plusieurs fois cela. L'eau se déduit de la même énergie via un facteur d'intensité hydrique de 3,32 litres par kilowattheure : les tours de refroidissement et, surtout, l'eau consommée pour produire l'électricité au départ. Les chiffres sont des estimations à l'ordre de grandeur, pas une précision de facturation, et je le dis haut et fort dans la [méthodologie](https://github.com/vinri2z/ai-footprint/blob/main/METHODOLOGY.md). Anthropic et les autres ne publient pas leurs vrais chiffres, alors une estimation indépendante et honnête est le mieux que chacun de nous puisse faire. Mais un ordre de grandeur suffit. Il n'a jamais été question de la troisième décimale. Il était question de rendre un coût invisible assez visible pour qu'on le *ressente*.

## Le paradoxe au centre de tout

Voici la part inconfortable, et la vraie raison pour laquelle je pense que cela compte.

L'instinct, une fois qu'on voit le nombre, c'est de devenir plus efficace. Prendre un modèle plus petit. Resserrer le *prompt*. Gaspiller moins de *tokens*. Tout cela est bon. Mais un piège attend, et il a près de deux siècles.

En 1865, l'économiste William Stanley Jevons remarqua quelque chose d'étrange à propos du charbon. À mesure que les machines à vapeur devenaient plus efficaces — qu'elles tiraient plus de travail de chaque morceau de charbon —, la Grande-Bretagne n'en brûlait pas *moins*. Elle en brûlait bien plus. L'efficacité rendait le charbon moins cher à utiliser, l'usage moins cher signifiait davantage d'usages, et la consommation totale explosa. Le gain par unité était avalé tout entier par la croissance du nombre d'unités. C'est le paradoxe de Jevons, et il se vérifie avec une fiabilité sinistre depuis lors.

L'IA, c'est Jevons en accéléré. Chaque gain d'efficacité — un modèle moins cher, une puce plus rapide, un agent plus abouti — ne réduit pas notre empreinte totale. Il abaisse le coût d'agir dans le monde numérique, alors nous y agissons davantage. Bien davantage. La friction qui limitait autrefois ce que nous produisions, traduisions, générions et calculions se dissout, et dans cet espace sans friction se rue un appétit essentiellement illimité. Nous n'utilisons pas l'IA pour faire le même travail avec moins. Nous l'utilisons pour faire des ordres de grandeur de travail en plus, et l'empreinte grimpe au rythme de la capacité.

C'est le mécanisme par lequel l'IA devient une véritable menace pour une planète habitable : non parce qu'une requête isolée serait chère — elle ne l'est pas —, mais parce qu'elle rend le monde numérique si bon marché et si rapide à opérer que nous fonçons tout droit à travers le budget que le monde physique peut se permettre. Chaque barrière qui nous freinait autrefois protégeait aussi, rétrospectivement, la base de ressources. Ôtez les barrières et vous ôtez les freins.

## Pourquoi un nombre dans un terminal

Je ne crois pas qu'une ligne d'état sauve la planète. Que ce soit clair.

Ce qu'elle fait est plus modeste et, je l'espère, plus honnête. Elle relie de nouveau une action à sa conséquence. Pour ceux qui programment, elle fait fuir l'abstraction exprès : on ne peut plus faire comme si les *tokens* étaient gratuits, parce que les grammes sont juste là. Pour tous les autres, c'est un argument par la démonstration : le « cloud » dans lequel vous vivez a une masse, boit de l'eau et brûle du charbon, et voici un moyen de le voir se produire en temps réel.

Le réconfort de la métaphore du nuage, c'est qu'elle nous laisse traiter le numérique comme s'il était séparé du physique : un second monde, plus propre, posé par-dessus le sale. Il ne l'est pas. Il n'y a qu'un seul monde, et le centre de données est dedans, buvant aux mêmes rivières et tirant sur le même réseau que tout le reste. La chose la plus utile que je puisse faire en tant qu'ingénieur, c'est de refuser la métaphore. Y remettre le poids. Le rendre visible, le rendre local, le rendre *mien* — puis laisser les gens décider, le nombre sous les yeux, quelle part du monde physique ils veulent dépenser dans le numérique.

Le nuage a toujours été lourd. Nous l'avons simplement construit pour qu'il n'en ait pas l'air.
