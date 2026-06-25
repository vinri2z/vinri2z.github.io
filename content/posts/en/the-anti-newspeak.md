---
title: "The Anti-Newspeak"
date: "2026-06-26"
description: "Orwell imagined a language built to shrink thought. We just built one that does the opposite — for machines."
---

In the appendix to *1984*, Orwell lays out the grammar of his dystopia. Not the telescreens, not the Ministry of Truth — the language. Newspeak was the regime's most ambitious instrument of control, and its logic was chillingly simple: if you remove the words for a thing, you remove the ability to think it. Strip "freedom" of every meaning except the trivial ("the dog is free from lice") and political freedom becomes, over generations, literally unthinkable. There would be no vocabulary in which to frame the heresy. Thoughtcrime wouldn't be punished. It would be impossible.

The premise underneath is older than Orwell and has never quite gone away: *the limits of your language are the limits of your world.* What you cannot say, you struggle to think. Narrow the expressible and you narrow the conceivable.

For most of computing history, software has lived under its own Newspeak — and we built the prison ourselves.

## The machine that could only speak in exact words

Think about what it has meant, until very recently, for two systems to talk to each other.

Every integration was a treaty negotiated in advance. This service emits a field called `customer_id`; that one expects `customerId`. One sends a date as `2026-06-25`, the other demands a Unix timestamp. The contract is rigid, total, and unforgiving. A single malformed payload — a trailing comma, a missing bracket, a string where a number was promised — and the whole exchange collapses. Not degrades. Collapses. The receiving system doesn't *interpret* the garbled message and shrug; it throws an exception and stops.

This is software as Newspeak. The machine can only think in the precise words it was handed. Anything outside its declared vocabulary isn't merely unfamiliar — it is *foreign*, and foreign means fatal. A JSON object that doesn't match the schema isn't a slightly-off attempt at communication. It is noise. The system has no concept of "what you probably meant," because meaning was never the point. Only conformance was.

We spent decades building elaborate apparatus to cope with this rigidity: schema validators, adapters, middleware, the entire grinding industry of ETL. All of it, in the end, translation work — labor spent forcing the world's messy expression back into a vocabulary narrow enough for a machine to accept. The integration problem was, at bottom, a *language* problem. Two systems that didn't speak the same dialect simply could not converse, and no amount of goodwill closed the gap. Someone had to write the dictionary, by hand, in advance.

## Breaking the barrier

Then something inverted.

The arrival of large language models — and, crucially, of protocols like MCP that let them reach out and *act* — broke the oldest assumption in the stack: that a machine can only process what exactly matches its declared grammar.

Now a malformed JSON object isn't the end of the conversation. It's the beginning of one. A model reads it, understands the *intent* behind the broken syntax, and repairs it. The trailing comma, the field named wrong, the value in the unexpected shape — these stop being fatal. The "foreign" message gets interpreted and fixed by another machine, on the fly, the way a fluent human reads past a typo without breaking stride.

That is a genuine shift in paradigm, not a clever new library. For the first time, software has something like *comprehension* sitting between systems — a layer that traffics in meaning rather than in exact matches. MCP gives that comprehension hands: it can call the tool, query the source, fix the payload, and route the result, all while tolerating the imperfection that used to be lethal.

And here's the part worth sitting with. Flexibility and reliability used to be a trade-off. You could have a system that was rigid and dependable, or loose and fragile, and the engineering craft lay in choosing where on that line to stand. The new arrangement collapses the trade-off. Software can now *bend without breaking* — absorb the unexpected, interpret the malformed, bridge two vocabularies that were never introduced — and still land on a reliable result.

## Orwell, inverted

This is where the parallel turns, and turns sharply.

Orwell's nightmare was a language deliberately *narrowed* to shrink the space of thought — control achieved by amputation, the world made smaller one deleted word at a time. What we have built for machines runs the film backward. We are *widening* the expressible. The vocabulary a system can accept is no longer the cage it lives in. A machine is no longer trapped by the exact words it was given; it can meet the foreign, the partial, the malformed, and make sense of it anyway.

If the limits of your language are the limits of your world, then we have just handed our machines a far larger world to live in. Newspeak shrank the sayable to control the thinkable. This does the opposite: it expands what can be said *to* a system — and therefore what a system can do — without surrendering the reliability that made software trustworthy in the first place.

Orwell warned that whoever controls language controls thought. He was right. The interesting question now is what happens when the thing controlling the language is no longer trying to make us think *less* — but is, for once, built to understand *more*.
