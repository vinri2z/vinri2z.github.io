---
title: "The Weight of the Cloud"
date: "2026-07-01"
description: "We call it the cloud so we don't have to picture the mines, the turbines, and the water. I built a tool to make the weight visible."
---

The word "cloud" is one of the most successful pieces of marketing ever written. It takes a warehouse the size of a small town — packed with servers, drawing the power of a mid-sized city, cooled by millions of liters of water — and turns it into a soft white shape floating somewhere above us. Weightless. Clean. Somewhere else. Not our problem.

The whole point of the metaphor is to make the materiality disappear. And it works. Most of us type a prompt, get an answer, and never once picture the physical event we just triggered: electrons pulled off a grid, heat dumped into the air, water evaporated to carry that heat away. The interface is frictionless on purpose. Friction is exactly what would make us think.

I've spent a while now unable to stop thinking about it. So I built something.

## ai-footprint

[ai-footprint](https://github.com/vinri2z/ai-footprint) is a small tool that tracks the carbon *and* water footprint of AI coding agents — Claude Code, Codex, Cursor, Gemini CLI, Copilot, and thirty-odd others — in one place. It reads how many tokens you've actually run through each model, maps every model to per-token emission factors drawn from peer-reviewed research, and turns the total into grams of CO₂ and milliliters of water.

The centerpiece isn't the report. It's the status line. While you work, sitting quietly at the bottom of the terminal, is a running tally:

```
🪵 65g CO₂ · 💦 800mL · $0.50
```

That's one session. Sixty-five grams of carbon. Eight hundred milliliters of water — most of a bottle — evaporated somewhere to cool the machines that just helped me rename a function. The number ticks up as I work. It is impossible to un-see once it's there.

The emission factors come from [Jegham et al. (2025)](https://arxiv.org/abs/2505.09598), which measured LLM inference energy on real AWS hardware. A Sonnet-class model runs about 1,140 gCO₂e per million output tokens; larger models several times that. Water is derived from the same energy through a water-intensity factor of 3.32 liters per kilowatt-hour — the cooling towers and, mostly, the water burned generating the electricity in the first place. The numbers are order-of-magnitude estimates, not billing-grade precision, and I say so loudly in the [methodology](https://github.com/vinri2z/ai-footprint/blob/main/METHODOLOGY.md). Anthropic and the others don't publish their real figures, so an honest independent estimate is the best any of us can do. But an order of magnitude is enough. The point was never the third decimal place. The point was to make an invisible cost visible enough to feel.

## The paradox at the center of it

Here's the uncomfortable part, and the real reason I think this matters.

The instinct, once you see the number, is to become more efficient. Use a smaller model. Tighten the prompt. Waste fewer tokens. All good. But there's a trap waiting, and it's nearly two centuries old.

In 1865 the economist William Stanley Jevons noticed something strange about coal. As steam engines got more efficient — as they extracted more work from each lump of coal — Britain didn't burn *less* coal. It burned dramatically more. Efficiency made coal cheaper to use, cheaper use meant more uses, and total consumption exploded. The gain per unit was swallowed whole by the growth in units. This is the Jevons paradox, and it has held with grim reliability ever since.

AI is Jevons on fast-forward. Every efficiency gain — a cheaper model, a faster chip, a slicker agent — doesn't reduce our total footprint. It lowers the cost of operating in the digital world, and so we operate in it more. Vastly more. The friction that used to limit how much we produced, translated, generated, and computed is being dissolved, and into that frictionless space rushes an essentially unlimited appetite. We don't use AI to do the same work with less. We use it to do orders of magnitude more work, and the footprint climbs right along with the capability.

That is the mechanism by which AI becomes a genuine threat to a livable planet: not because a single query is expensive — it isn't — but because it makes the numerical world so cheap and fast to operate that we accelerate straight through whatever budget the physical world can afford. Every barrier that once slowed us down was, in retrospect, also protecting the resource base. Remove the barriers and you remove the brakes.

## Why a number in a terminal

I don't think a status line saves the planet. Let me be clear about that.

What it does is smaller and, I hope, more honest. It reconnects an action to its consequence. For developers, it makes the abstraction leak on purpose — you can no longer pretend the tokens are free, because the grams are right there. For everyone else, it's an argument by demonstration: the "cloud" you live in has mass, drinks water, and burns coal, and here is a way to watch it happen in real time.

The comfort of the cloud metaphor is that it lets us treat the digital as if it were separate from the physical — a second, cleaner world layered over the dirty one. It isn't. There is one world, and the data center is in it, drinking from the same rivers and pulling from the same grid as everything else. The most useful thing I can do as an engineer is refuse the metaphor. Put the weight back on. Make it visible, make it local, make it *mine* — and then let people decide, with the number in front of them, how much of the physical world they want to spend on the digital one.

The cloud was always heavy. We just built it not to look that way.
