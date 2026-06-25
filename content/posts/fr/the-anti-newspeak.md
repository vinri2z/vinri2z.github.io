---
title: "L'anti-novlangue"
date: "2026-06-26"
description: "Orwell a imaginé une langue conçue pour rétrécir la pensée. Nous venons d'en construire une qui fait l'inverse — pour les machines."
---

Dans l'appendice de *1984*, Orwell expose la grammaire de sa dystopie. Pas les télécrans, pas le Ministère de la Vérité — la langue. La novlangue était l'instrument de contrôle le plus ambitieux du régime, et sa logique était d'une simplicité glaçante : si l'on supprime les mots d'une chose, on supprime la capacité de la penser. Dépouillez « liberté » de tout sens hormis le trivial (« le chien est libre de puces ») et la liberté politique devient, au fil des générations, littéralement impensable. Il n'y aurait aucun vocabulaire dans lequel formuler l'hérésie. Le crime par la pensée ne serait pas puni. Il serait impossible.

La prémisse qui se cache dessous est plus ancienne qu'Orwell et n'a jamais vraiment disparu : *les limites de votre langage sont les limites de votre monde.* Ce que vous ne pouvez pas dire, vous peinez à le penser. Rétrécissez l'exprimable et vous rétrécissez le concevable.

Pendant presque toute l'histoire de l'informatique, le logiciel a vécu sous sa propre novlangue — et la prison, c'est nous qui l'avons bâtie.

## La machine qui ne savait parler qu'avec des mots exacts

Songez à ce que cela a signifié, jusqu'à très récemment, pour deux systèmes de se parler.

Chaque intégration était un traité négocié à l'avance. Ce service émet un champ nommé `customer_id` ; cet autre en attend un `customerId`. L'un envoie une date sous la forme `2026-06-25`, l'autre exige un horodatage Unix. Le contrat est rigide, total et impitoyable. Une seule charge malformée — une virgule en trop, un crochet manquant, une chaîne là où l'on avait promis un nombre — et tout l'échange s'effondre. Il ne se dégrade pas. Il s'effondre. Le système récepteur n'*interprète* pas le message brouillé en haussant les épaules ; il lève une exception et s'arrête.

C'est le logiciel comme novlangue. La machine ne sait penser qu'avec les mots précis qu'on lui a remis. Tout ce qui sort de son vocabulaire déclaré n'est pas seulement inconnu : c'est *étranger*, et étranger veut dire fatal. Un objet JSON qui ne correspond pas au schéma n'est pas une tentative de communication légèrement décalée. C'est du bruit. Le système n'a aucune notion de « ce que vous vouliez probablement dire », parce que le sens n'a jamais été le but. Seule l'était la conformité.

Nous avons passé des décennies à construire des appareillages élaborés pour composer avec cette rigidité : validateurs de schémas, adaptateurs, middleware, toute l'industrie laborieuse de l'ETL. Tout cela, au fond, du travail de traduction : un effort dépensé à forcer l'expression désordonnée du monde à rentrer de nouveau dans un vocabulaire assez étroit pour qu'une machine l'accepte. Le problème de l'intégration était, au fond, un problème de *langage*. Deux systèmes qui ne parlaient pas le même dialecte ne pouvaient tout simplement pas converser, et aucune bonne volonté ne comblait l'écart. Quelqu'un devait écrire le dictionnaire, à la main, à l'avance.

## Briser la barrière

Puis quelque chose s'est inversé.

L'arrivée des grands modèles de langage — et, surtout, de protocoles comme MCP qui leur permettent de tendre la main et d'*agir* — a brisé la plus vieille supposition de la pile : qu'une machine ne peut traiter que ce qui correspond exactement à sa grammaire déclarée.

Désormais, un objet JSON malformé n'est plus la fin de la conversation. C'en est le début. Un modèle le lit, comprend l'*intention* derrière la syntaxe brisée, et la répare. La virgule en trop, le champ mal nommé, la valeur à la forme inattendue : tout cela cesse d'être fatal. Le message « étranger » est interprété et corrigé par une autre machine, à la volée, comme un humain fluide lit par-dessus une coquille sans rompre sa foulée.

C'est là un véritable changement de paradigme, pas une nouvelle bibliothèque astucieuse. Pour la première fois, le logiciel possède quelque chose comme une *compréhension* placée entre les systèmes — une couche qui trafique le sens plutôt que les correspondances exactes. MCP donne des mains à cette compréhension : elle peut appeler l'outil, interroger la source, corriger la charge et router le résultat, le tout en tolérant l'imperfection qui était autrefois mortelle.

Et voici la partie sur laquelle il vaut la peine de s'attarder. La flexibilité et la fiabilité étaient autrefois un compromis. On pouvait avoir un système rigide et fiable, ou souple et fragile, et l'art de l'ingénierie consistait à choisir où se tenir sur cette ligne. Le nouvel arrangement fait s'effondrer le compromis. Le logiciel peut désormais *plier sans rompre* — absorber l'inattendu, interpréter le malformé, jeter un pont entre deux vocabulaires qui ne s'étaient jamais présentés — tout en aboutissant à un résultat fiable.

## Orwell, inversé

C'est ici que le parallèle bascule, et bascule brutalement.

Le cauchemar d'Orwell était une langue délibérément *rétrécie* pour réduire l'espace de la pensée — un contrôle obtenu par amputation, le monde rendu plus petit un mot supprimé à la fois. Ce que nous avons construit pour les machines fait défiler le film à l'envers. Nous *élargissons* l'exprimable. Le vocabulaire qu'un système peut accepter n'est plus la cage où il vit. Une machine n'est plus prisonnière des mots exacts qu'on lui a donnés ; elle peut rencontrer l'étranger, le partiel, le malformé, et lui donner sens malgré tout.

Si les limites de votre langage sont les limites de votre monde, alors nous venons de remettre à nos machines un monde bien plus vaste où vivre. La novlangue a rétréci le dicible pour contrôler le pensable. Ceci fait l'inverse : cela élargit ce que l'on peut dire *à* un système — et donc ce qu'un système peut faire — sans renoncer à la fiabilité qui faisait du logiciel une chose digne de confiance au départ.

Orwell avertissait que celui qui contrôle le langage contrôle la pensée. Il avait raison. La question intéressante, désormais, est de savoir ce qui se passe quand la chose qui contrôle le langage ne cherche plus à nous faire penser *moins* — mais est, pour une fois, conçue pour comprendre *davantage*.
